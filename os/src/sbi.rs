#![allow(unused)]

use core::arch::asm;

const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_SHUTDOWN: usize = 8;

#[inline(always)]
fn sbi_call(extension: usize, func: usize, arg0: usize, arg1: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
        "ecall",
        inlateout("x10") func => ret,
        in("x11") arg0,
        in("x12") arg1,
        in("x17") extension,
        );
    }
    ret
}

// 输出一个字符
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

pub fn shut_down() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    loop {}
}
