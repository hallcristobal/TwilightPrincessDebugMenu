use arrayvec::ArrayVec;
use core::fmt::Write;
use libtp::link::{Inventory, Link};

use utils::*;
use {commands, controller};

static mut cursor: usize = 0;
static mut scroll_offset: usize = 0;
static mut already_pressed_a: bool = false;

pub const CHEAT_AMNT: usize = 7;

pub fn transition_into() {
    unsafe {
        already_pressed_a = false;
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
enum CheatId {
    Invincible,
    InvincibleEnemies,
    InfiniteAir,
    InifinteBombs,
    InfiniteRupees,
    InfiniteArrows,
    MoonJumpEnabled,
    TeleportEnabled,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cheat {
    id: CheatId,
    name: &'static str,
    pub active: bool,
    togglable: bool,
}

impl Cheat {
    const fn new(id: CheatId, name: &'static str, togglable: bool) -> Self {
        Cheat {
            id: id,
            name: name,
            active: false,
            togglable: togglable,
        }
    }
}

pub fn apply_cheats() {
    let link = Link::get_link();
    let inventory = Inventory::get_inventory();
    for cheat in unsafe { &cheats } {
        if cheat.active {
            match cheat.id {
                Invincible => {
                    link.heart_quarters = (link.heart_pieces / 5) * 4;
                }
                InvincibleEnemies => {}
                InfiniteAir => {
                    let mut air = Link::get_air();
                    *air = 600;
                }
                InifinteBombs => {
                    inventory.bomb_bag_1_amnt = 99;
                    inventory.bomb_bag_2_amnt = 99;
                    inventory.bomb_bag_3_amnt = 99;
                }
                InfiniteRupees => {
                    link.rupees = 1000;
                }
                InfiniteArrows => {
                    inventory.arrow_count = 99;
                }
                MoonJumpEnabled => unsafe {
                    commands::COMMANDS[commands::MOON_JUMP].active = true;
                },
                TeleportEnabled => {
                    unsafe {
                        commands::COMMANDS[commands::LOAD_POSITION].active = true;
                    }
                    unsafe {
                        commands::COMMANDS[commands::STORE_POSITION].active = true;
                    }
                }
            }
        } else {
            match cheat.id {
                MoonJumpEnabled => unsafe {
                    commands::COMMANDS[commands::MOON_JUMP].active = false;
                },
                TeleportEnabled => {
                    unsafe {
                        commands::COMMANDS[commands::LOAD_POSITION].active = false;
                    }
                    unsafe {
                        commands::COMMANDS[commands::STORE_POSITION].active = false;
                    }
                }
                _ => {}
            }
        }
    }
}

static mut cheats: [Cheat; CHEAT_AMNT] = [
    Cheat::new(Invincible, "Invincible", true),
    // Cheat::new(InvincibleEnemies, "Invincible Enemies", true),
    Cheat::new(InfiniteAir, "Infinite Air", true),
    Cheat::new(InifinteBombs, "Infinite Bombs", true),
    Cheat::new(InfiniteRupees, "Infinite Rupees", true),
    Cheat::new(InfiniteArrows, "Infinite Arrows", true),
    Cheat::new(MoonJumpEnabled, "Moon Jump Enabled", true),
    Cheat::new(TeleportEnabled, "Teleport Enabled", true),
];

use self::CheatId::*;

pub unsafe fn get_cheats() -> &'static [Cheat] {
    &cheats
}

pub unsafe fn load_cheats(new: ArrayVec<[bool; CHEAT_AMNT]>) {
    new.iter().enumerate().for_each(|(i, b)| {
        cheats[i].active = *b;
    });
}

pub fn render() {
    let state = unsafe { super::get_state() };
    let lines = state.menu.lines_mut();

    let down_a = controller::A.is_down();
    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        transition(MenuState::MainMenu);
        return;
    }
    unsafe {
        scroll_move_cursor(cheats.len(), &mut cursor, &mut scroll_offset);
    }

    let cheat_id = unsafe { cursor };
    let cheat = unsafe { &mut cheats[cheat_id] };

    unsafe {
        already_pressed_a |= pressed_a;
    }

    if cheat.togglable {
        cheat.active ^= pressed_a;
    } else if unsafe { already_pressed_a } {
        cheat.active = down_a;
    }

    for (index, (line, cheat)) in lines
        .into_iter()
        .zip(unsafe { cheats.iter().skip(scroll_offset) })
        .enumerate()
        .take(state.settings.max_lines)
    {
        let index = index + unsafe { scroll_offset };

        let checkbox = if cheat.active { "[x] " } else { "[ ] " };

        let text = cheat.name;
        let text = if text.len() > 45 { &text[..45] } else { text };

        let _ = write!(line.begin(), "{}{}", checkbox, text);
        line.selected = index == unsafe { cursor };
    }
}
