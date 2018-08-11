#![no_std]
#![feature(panic_implementation, proc_macro_non_items)]

extern crate gcn_fonts;

use gcn_fonts::prelude::*;

pub mod print;

struct State {
    font: UploadedFont,
}

static mut STATE: Option<State> = None;
static FONT: Font = include_font! { path: "res/Calamity-Bold.ttf", size: 20.0 };

unsafe fn get_state() -> &'static mut State {
    STATE.get_or_insert_with(|| State {
        font: FONT.upload(),
    })
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    print::setup_draw();
    print::printf("Hello from Rust!", 20.0, 20.0, 0xFF_FF_FF_FF);
}

use core::panic::PanicInfo;
#[panic_implementation]
#[no_mangle]
pub fn my_panic(_: &PanicInfo) -> ! {
    loop {}
}
