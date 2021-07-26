#![allow(unused)]

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUT_CHAR: usize = 1;
const SBI_CONSOLE_GET_CHAR: usize = 2;
// const SBI_CLEAR_IPI: usize = 3;
// const SBI_SEND_IPI: usize = 4;
// const SBI_REMOTE_FENCE_I: usize = 5;
// const SBI_REMOTE_SFENCE_VMA: usize = 6;
// const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

#[inline(always)]
fn sbi_call(id: usize, args: [usize; 3]) -> usize {
    let mut ret;
    unsafe {
        asm!("ecall",
        inlateout("x10") args[0] => ret,
        in("x11") args[1],
        in("x12") args[2],
        in("x17") id,
        options(nostack)
        )
    }
    ret
}

pub fn set_timer(timer: usize) {
    sbi_call(SBI_SET_TIMER, [timer, 0, 0]);
}

pub fn console_put_char(c: usize) {
    sbi_call(SBI_CONSOLE_PUT_CHAR, [c, 0, 0]);
}

pub fn console_get_char() -> usize {
    sbi_call(SBI_CONSOLE_GET_CHAR, [0, 0, 0])
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, [0, 0, 0]);
    panic!("It should shutdown!");
}

