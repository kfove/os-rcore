#![no_std]
#![no_main]
#![feature(panic_info_message)]

// 引入 panic_handler
mod console;
mod lang_items;
mod qemu_exit;
mod sbi;
use crate::qemu_exit::{QEMUExit, QEMU_EXIT_HANDLE};

// 嵌入汇编
use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

// 避免函数名混淆
#[no_mangle]
fn rust_main() -> ! {
    extern "C" {
        fn stext(); // .text 段的头地址
        fn etext(); // .text 段的尾地址
        fn srodata(); // Read-Only data 段的头地址
        fn erodata(); // Read-Only data 段的尾地址
        fn sdata(); // .data 段的头地址
        fn edata(); // .data 段的尾地址
        fn sbss(); // BSS 段的头地址
        fn ebss(); // end addr of BSS segment
        fn boot_stack(); // 栈
        fn boot_stack_top(); // 栈顶
    }
    clear_bss();
    println!("Hello World!");
    println!("Hello, world!");
    println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    println!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    println!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    QEMU_EXIT_HANDLE.exit_success();
}

pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    })
}
