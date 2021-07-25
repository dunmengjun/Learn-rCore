use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};
use super::PageTableEntry;
use core::fmt::{self, Debug, Formatter};

/// Definitions
#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddress(pub usize);

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddress(pub usize);

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysPageNumber(pub usize);

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNumber(pub usize);

/// Debugging

impl Debug for VirtAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VA:{:#x}", self.0))
    }
}
impl Debug for VirtPageNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VPN:{:#x}", self.0))
    }
}
impl Debug for PhysAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}
impl Debug for PhysPageNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PPN:{:#x}", self.0))
    }
}

/// T: {PhysAddr, VirtAddr, PhysPageNum, VirtPageNum}
/// T -> usize: T.0
/// usize -> T: usize.into()

impl From<usize> for PhysAddress {
    fn from(v: usize) -> Self { Self(v) }
}
impl From<usize> for PhysPageNumber {
    fn from(v: usize) -> Self { Self(v) }
}
impl From<usize> for VirtAddress {
    fn from(v: usize) -> Self { Self(v) }
}
impl From<usize> for VirtPageNumber {
    fn from(v: usize) -> Self { Self(v) }
}
impl From<PhysAddress> for usize {
    fn from(v: PhysAddress) -> Self { v.0 }
}
impl From<PhysPageNumber> for usize {
    fn from(v: PhysPageNumber) -> Self { v.0 }
}
impl From<VirtAddress> for usize {
    fn from(v: VirtAddress) -> Self { v.0 }
}
impl From<VirtPageNumber> for usize {
    fn from(v: VirtPageNumber) -> Self { v.0 }
}

impl VirtAddress {
    pub fn floor(&self) -> VirtPageNumber { VirtPageNumber(self.0 / PAGE_SIZE) }
    pub fn ceil(&self) -> VirtPageNumber { VirtPageNumber((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE) }
    pub fn page_offset(&self) -> usize { self.0 & (PAGE_SIZE - 1) }
    pub fn aligned(&self) -> bool { self.page_offset() == 0 }
}
impl From<VirtAddress> for VirtPageNumber {
    fn from(v: VirtAddress) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}
impl From<VirtPageNumber> for VirtAddress {
    fn from(v: VirtPageNumber) -> Self { Self(v.0 << PAGE_SIZE_BITS) }
}
impl PhysAddress {
    pub fn floor(&self) -> PhysPageNumber { PhysPageNumber(self.0 / PAGE_SIZE) }
    pub fn ceil(&self) -> PhysPageNumber { PhysPageNumber((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE) }
    pub fn page_offset(&self) -> usize { self.0 & (PAGE_SIZE - 1) }
    pub fn aligned(&self) -> bool { self.page_offset() == 0 }
}
impl From<PhysAddress> for PhysPageNumber {
    fn from(v: PhysAddress) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}
impl From<PhysPageNumber> for PhysAddress {
    fn from(v: PhysPageNumber) -> Self { Self(v.0 << PAGE_SIZE_BITS) }
}

impl VirtPageNumber {
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut idx = [0usize; 3];
        for i in (0..3).rev() {
            idx[i] = vpn & 511;
            vpn >>= 9;
        }
        idx
    }
}

impl PhysAddress {
    pub fn get_ref<T>(&self) -> &'static T {
        unsafe {
            (self.0 as *const T).as_ref().unwrap()
        }
    }
    pub fn get_mut<T>(&self) -> &'static mut T {
        unsafe {
            (self.0 as *mut T).as_mut().unwrap()
        }
    }
}
impl PhysPageNumber {
    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddress = self.clone().into();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, 512)
        }
    }
    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysAddress = self.clone().into();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut u8, 4096)
        }
    }
    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysAddress = self.clone().into();
        pa.get_mut()
    }
}

pub trait StepByOne {
    fn step(&mut self);
}
impl StepByOne for VirtPageNumber {
    fn step(&mut self) {
        self.0 += 1;
    }
}
impl StepByOne for PhysPageNumber {
    fn step(&mut self) {
        self.0 += 1;
    }
}

#[derive(Copy, Clone)]
pub struct SimpleRange<T> where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug, {
    l: T,
    r: T,
}
impl<T> SimpleRange<T> where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug, {
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end, "start {:?} > end {:?}!", start, end);
        Self { l: start, r: end }
    }
    pub fn get_start(&self) -> T { self.l }
    pub fn get_end(&self) -> T { self.r }
}
impl<T> IntoIterator for SimpleRange<T> where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug, {
    type Item = T;
    type IntoIter = SimpleRangeIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        SimpleRangeIterator::new(self.l, self.r)
    }
}
pub struct SimpleRangeIterator<T> where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug, {
    current: T,
    end: T,
}
impl<T> SimpleRangeIterator<T> where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug, {
    pub fn new(l: T, r: T) -> Self {
        Self { current: l, end: r, }
    }
}
impl<T> Iterator for SimpleRangeIterator<T> where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug, {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let t = self.current;
            self.current.step();
            Some(t)
        }
    }
}
pub type VPNRange = SimpleRange<VirtPageNumber>;