#![no_std]
#![no_main]
#![feature(panic_info_message)]

// 引入 panic_handler
mod console;
mod lang_items;
mod sbi;

// 嵌入汇编
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

// 避免函数名混淆
#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    println!("Hello World!");
    println!("你好 Rust!");
    loop {}
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
