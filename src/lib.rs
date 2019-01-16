#![no_std]
#![feature(proc_macro_hygiene)]
#![feature(const_fn)]
#![allow(non_upper_case_globals)]
#![feature(trace_macros)]

extern crate arrayvec;
extern crate gcn_fonts;
#[macro_use]
extern crate gcn;
extern crate libtp;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde;

use gcn::card::{Card, CardError};
use gcn_fonts::prelude::*;

pub mod cheat_menu;
pub mod commands;
pub mod controller;
pub mod inventory;
pub mod main_menu;
pub mod memory;
pub mod menu;
pub mod popups;
pub mod print;
pub mod settings;
pub mod utils;
pub mod warping;

pub static mut visible: bool = false;

struct State {
    font: UploadedFont,
    menu: menu::Menu,
    settings: settings::Settings,
}

static mut STATE: Option<State> = None;
pub static mut LOADED_SAVE: bool = false;

static FONT: Font = include_font! { path: "res/Calamity-Bold.ttf", size: 18.0 };

unsafe fn get_state() -> &'static mut State {
    STATE.get_or_insert_with(|| State {
        font: FONT.upload(),
        menu: menu::Menu::default(),
        settings: settings::Settings {
            drop_shadow: true,
            max_lines: 16,
        },
    })
}

#[no_mangle]
pub extern "C" fn init_once() {
    // hack
    libtp::system::memory::write::<[u8; 40]>(
        0x8000_5080,
        [
            0xA8, 0x1B, 0x05, 0x62, 0xAB, 0xBB, 0x05, 0x60, 0x7C, 0x00, 0xE8, 0x00, 0x40, 0x82,
            0x00, 0x18, 0x1F, 0xBD, 0xEF, 0xFF, 0xB3, 0xBB, 0x05, 0x60, 0x3B, 0xBD, 0x00, 0x01,
            0xB3, 0xBB, 0x05, 0x62, 0xA8, 0x1B, 0x05, 0x62, 0x48, 0x08, 0x2E, 0x88,
        ],
    );
}

#[no_mangle]
pub extern "C" fn game_loop() {
    if unsafe { !LOADED_SAVE } {
        match Card::open("tpgz01") {
            Ok(mut card) => {
                match card.deserialize_read(settings::unpack_save) {
                    Ok(_) => {
                        report!("Read mem card");
                    }
                    Err(e) => {
                        report!("Failed to read mem card: {:?}", e);
                    }
                }
                unsafe {
                    LOADED_SAVE = true;
                }
            }
            Err(CardError::Busy) | Err(CardError::NoCard) => {}
            Err(e) => {
                report!("Failed to open mem card: {:?}", e);
                unsafe {
                    LOADED_SAVE = true;
                }
                settings::defaults();
            }
        }
    }
    cheat_menu::apply_cheats();
    let d_down = controller::DPAD_DOWN.is_pressed();
    let rt_down = controller::R.is_down();
    let lt_down = controller::L.is_down();

    if unsafe { visible } {
        utils::render();
    } else if d_down && rt_down && lt_down && unsafe { !popups::visible } {
        unsafe {
            visible = true;
        }
    } else {
        // popups::check_global_flags();
    }
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    print::setup_draw();
    if visible {
        get_state().menu.draw();
    }
    memory::render_watches();
    popups::draw_popup();
}
