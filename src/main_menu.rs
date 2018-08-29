use core::fmt::Write;

use controller;
use utils::*;
use {card, get_state, visible};

static mut cursor: usize = 0;
static mut card: Option<card::Card> = None;

pub fn transition_into() {}

pub fn render() {
    const MEMORY_INDEX: usize = 0;
    const INVENTORY_INDEX: usize = 1;
    const CHEAT_INDEX: usize = 2;
    const CARD_INDEX: usize = 3;
    const CARD_CREATE: usize = 4;
    const CARD_SAVE: usize = 5;

    let state = unsafe { get_state() };
    let lines = state.menu.lines_mut();
    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        unsafe {
            visible = false;
        }
        return;
    }

    let contents = [
        "Memory",
        "Inventory",
        "Cheat Menu",
        "Card",
        "Create",
        "Save",
    ];

    move_cursor(contents.len(), unsafe { &mut cursor });

    if pressed_a {
        match unsafe { cursor } {
            MEMORY_INDEX => {
                transition(MenuState::Memory);
                return;
            }
            INVENTORY_INDEX => {
                transition(MenuState::InventoryMenu);
                return;
            }
            CHEAT_INDEX => {
                transition(MenuState::CheatMenu);
                return;
            }
            CARD_INDEX => {
                if let Ok(c) = card::Card::init() {
                    unsafe {
                        card = Some(c);
                    }
                }
                return;
            }
            CARD_CREATE => unsafe {
                if let Some(ref mut c) = card {
                    let _ = c.create();
                }
            },
            CARD_SAVE => unsafe {
                if let Some(ref mut c) = card {
                    let _ = c.write();
                }
            },
            _ => {}
        }
    }

    for (index, (line, &content)) in lines.iter_mut().zip(&contents).enumerate() {
        let _ = write!(line.begin(), "{}", content);
        line.selected = index == unsafe { cursor };
    }
}
