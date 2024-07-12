#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

use log::{debug, error, info, trace, warn};

#[macro_use]
mod console;
mod batch;
mod lang_item;
mod logging;
mod sbi;
mod sync;
mod syscall;
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    logging::init();
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn skernal();
        fn ekernal();
        fn boot_stack_lower_bound();
        fn boot_stack_top();
    }
    error!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    warn!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    debug!(
        ".stack [{:#x}, {:#x})",
        boot_stack_lower_bound as usize, boot_stack_top as usize
    );
    trace!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    println!("kernal [{:#x}, {:#x})", skernal as usize, ekernal as usize);
    trap::init();
    batch::init();
    batch::run_next_app();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
