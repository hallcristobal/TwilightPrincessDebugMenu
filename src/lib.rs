#![no_std]
#![feature(const_fn, panic_implementation, proc_macro_non_items)]
#![allow(non_upper_case_globals)]

extern crate arrayvec;
extern crate gcn_fonts;
#[macro_use]
extern crate gcn;
extern crate libtp;
#[macro_use]
extern crate lazy_static;

use gcn_fonts::prelude::*;

pub mod controller;
pub mod main_menu;
pub mod memory;
pub mod menu;
pub mod print;
pub mod settings;
pub mod utils;

use core::panic::PanicInfo;

#[panic_implementation]
#[no_mangle]
pub fn my_panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;
    let mut message = arrayvec::ArrayString::<[u8; 1024]>::new();
    write!(message, "{}\0", info).ok();
    unsafe {
        gcn::os::report(message.as_ptr());
    }
    loop {}
}

pub static mut visible: bool = false;

struct State {
    font: UploadedFont,
    menu: menu::Menu,
    settings: settings::Settings,
}

static mut STATE: Option<State> = None;
static FONT: Font = include_font! { path: "res/Calamity-Bold.ttf", size: 18.0 };

unsafe fn get_state() -> &'static mut State {
    STATE.get_or_insert_with(|| State {
        font: FONT.upload(),
        menu: menu::Menu::default(),
        settings: settings::default_settings(),
    })
}

#[no_mangle]
pub extern "C" fn game_loop() {
    let d_down = controller::DPAD_DOWN.is_pressed();
    let rt_down = controller::R.is_down();

    if unsafe { visible } {
        utils::render();
    } else if d_down && rt_down {
        unsafe {
            visible = true;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    print::setup_draw();
    if visible {
        get_state().menu.draw();
    }
    memory::render_watches();
}
