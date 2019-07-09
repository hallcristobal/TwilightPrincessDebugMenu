use super::super::get_state;
use core::fmt::Write;
use libtp::link::Link;

use controller;
use utils::{move_cursor, transition, MenuState};

use super::{inv_menu_state, InventoryMenu};

static mut cursor: usize = 0;

pub fn transition_into() {}

const MENU_ITEM_RUPEES: usize = 0;

const ITEMS: [&str; 1] = ["Rupees"];

pub fn render() {
    let link = Link::get_link();
    let state = unsafe { get_state() };
    let lines = state.menu.lines_mut();
    let pressed_b = controller::B.is_pressed();
    let pressed_a = controller::A.is_pressed();

    if pressed_b {
        unsafe {
            inv_menu_state = InventoryMenu::Main;
        }
        transition(MenuState::MainMenu);
        return;
    }

    move_cursor(ITEMS.len(), unsafe { &mut cursor });

    for (index, (line, item)) in lines.into_iter().zip(ITEMS.iter()).enumerate() {
        let index = index;
        let _ = match index {
            MENU_ITEM_RUPEES => write!(line.begin(), "{}: {}", item, link.rupees),
            _ => write!(line.begin(), "")
        };

        if index == unsafe { cursor } {
            line.selected = true;
        }
    }
}
