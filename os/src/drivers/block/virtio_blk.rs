
use virtio_drivers::{VirtIOBlk, VirtIOHeader};
use crate::mm::{
    PhysAddress,
    VirtAddress,
    frame_alloc_unwrap,
    frame_dealloc,
    PhysPageNumber,
    FrameTracker,
    StepByOne,
    PageTable,
    kernel_token,
};
use super::BlockDevice;
use spin::Mutex;
use alloc::vec::Vec;
use lazy_static::*;

#[allow(unused)]
const VIRTIO0: usize = 0x10001000;

pub struct VirtIOBlock(Mutex<VirtIOBlk<'static>>);

lazy_static! {
    static ref QUEUE_FRAMES: Mutex<Vec<FrameTracker>> = Mutex::new(Vec::new());
}

impl BlockDevice for VirtIOBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        self.0.lock().read_block(block_id, buf).expect("Error when reading VirtIOBlk");
    }
    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0.lock().write_block(block_id, buf).expect("Error when writing VirtIOBlk");
    }
}

impl VirtIOBlock {
    #[allow(unused)]
    pub fn new() -> Self {
        Self(Mutex::new(VirtIOBlk::new(
            unsafe { &mut *(VIRTIO0 as *mut VirtIOHeader) }
        ).unwrap()))
    }
}

#[no_mangle]
pub extern "C" fn virtio_dma_alloc(pages: usize) -> PhysAddress {
    let mut ppn_base = PhysPageNumber(0);
    for i in 0..pages {
        let frame = frame_alloc_unwrap();
        if i == 0 { ppn_base = frame.ppn; }
        assert_eq!(frame.ppn.0, ppn_base.0 + i);
        QUEUE_FRAMES.lock().push(frame);
    }
    ppn_base.into()
}

#[no_mangle]
pub extern "C" fn virtio_dma_dealloc(pa: PhysAddress, pages: usize) -> i32 {
    let mut ppn_base: PhysPageNumber = pa.into();
    for _ in 0..pages {
        frame_dealloc(ppn_base);
        ppn_base.step();
    }
    0
}

#[no_mangle]
pub extern "C" fn virtio_phys_to_virt(paddr: PhysAddress) -> VirtAddress {
    VirtAddress(paddr.0)
}

#[no_mangle]
pub extern "C" fn virtio_virt_to_phys(vaddr: VirtAddress) -> PhysAddress {
    PageTable::from_token(kernel_token()).translate_va(vaddr).unwrap()
}
