use core::fmt::Write;
use libtp::system::boss_flags_value;

use utils::*;
use {controller, get_state, visible, warping};

static mut cursor: usize = 0;

pub fn transition_into() {}

pub fn render() {
    const INVENTORY_INDEX: usize = 0;
    const CHEAT_INDEX: usize = 1;
    const WARPING_INDEX: usize = 2;
    const MEMORY_INDEX: usize = 3;
    const SETTINGS_INDEX: usize = 4;
    const QUICK_WARP_INDEX: usize = 6;
    const BOSS_FLAGS_INDEX: usize = 8;
    const ALTER_BOSS_FLAGS_INDEX: usize = 9;

    let state = unsafe { get_state() };
    let boss_flags = boss_flags_value();
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
        "Inventory",
        "Cheats",
        "Warping",
        "Memory",
        "Settings",
        "",
        "Quick Warp",
        "",
        "Boss Flags: ",
        "Set/Clear Boss Flags",
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
            SETTINGS_INDEX => {
                transition(MenuState::Settings);
                return;
            }
            WARPING_INDEX => {
                transition(MenuState::Warp);
                return;
            }
            QUICK_WARP_INDEX => {
                warping::load_saved_warp();
                return;
            }
            ALTER_BOSS_FLAGS_INDEX => {
                if *boss_flags == 0 {
                    *boss_flags = 10;
                } else {
                    *boss_flags = 0;
                }
                return;
            }
            _ => {}
        }
    }

    for (index, (line, &content)) in lines.iter_mut().zip(&contents).enumerate() {
        let _ = match index {
            BOSS_FLAGS_INDEX => write!(line.begin(), "{}: {}", content, boss_flags),
            _ => write!(line.begin(), "{}", content),
        };
        line.selected = index == unsafe { cursor };
    }
}
