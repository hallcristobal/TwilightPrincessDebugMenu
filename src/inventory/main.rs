use super::super::get_state;
use core::fmt::Write;

use controller;
use utils::{move_cursor, transition, MenuState};

use super::{inv_menu_state, InventoryMenu};

static mut cursor: usize = 0;

pub fn transition_into() {}

const MENU_ITEM_INVENTORY: usize = 0;
const MENU_ITEM_QUEST: usize = 1;
// const MENU_ITEM_AMOUNTS: usize = 2;

const ITEMS: [&str; 2] = ["Item Wheel", "Collection Menu" /* "Amounts"*/];

pub fn render() {
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

    if pressed_a {
        unsafe {
            if cursor == MENU_ITEM_INVENTORY {
                inv_menu_state = InventoryMenu::Equipment;
                transition(MenuState::InventoryMenu);
                return;
            } else if cursor == MENU_ITEM_QUEST {
                inv_menu_state = InventoryMenu::Quest;
                transition(MenuState::InventoryMenu);
            } //else if cursor == MENU_ITEM_AMOUNTS {
              //     inv_menu_state = InventoryMenu::Amounts;
              //     transition(MenuState::InventoryMenu);
              //     return;
              // }
        }
    }

    move_cursor(ITEMS.len(), unsafe { &mut cursor });

    for (index, (line, item)) in lines.into_iter().zip(ITEMS.iter()).enumerate() {
        let index = index;
        let _ = write!(line.begin(), "{}", item);
        if index == unsafe { cursor } {
            line.selected = true;
        }
    }
}
