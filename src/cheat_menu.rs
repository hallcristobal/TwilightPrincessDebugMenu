use core::fmt::Write;
use libtp::system;

use controller;
use utils::*;

static mut cursor: usize = 0;
static mut scroll_offset: usize = 0;
static mut already_pressed_a: bool = false;

pub fn transition_into() {
    unsafe {
        already_pressed_a = false;
    }
}

struct Cheat {
    id: CheatId,
    name: &'static str,
    active: bool,
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
    for cheat in unsafe { &cheats } {
        if cheat.active {
            match cheat.id {
                Invincible => {}
InvincibleEnemies => {}
InfiniteAir => {}
InifinteBombs => {}
InfiniteRupees => {}
InfiniteArrows => {}
MoonJumpEnabled => {}
TeleportEnabled => {}
            }
        } else {
            match cheat.id {
                Invincible => {}
InvincibleEnemies => {}
InfiniteAir => {}
InifinteBombs => {}
InfiniteRupees => {}
InfiniteArrows => {}
MoonJumpEnabled => {}
TeleportEnabled => {}
            }
        }
    }
}

static mut cheats: [Cheat; 8] = [
    Cheat::new(Invincible, "Invincible", true),
    Cheat::new(InvincibleEnemies, "Invincible Enemies", true),
    Cheat::new(InfiniteAir, "Infinite Air", true),
    Cheat::new(InifinteBombs, "Infinite Bombs", true),
    Cheat::new(InfiniteRupees, "Infinite Rupees", true),
    Cheat::new(InfiniteArrows, "Infinite Arrows", true),
    Cheat::new(MoonJumpEnabled, "Moon Jump Enabled", true),
    Cheat::new(TeleportEnabled, "Teleport Enabled", true),
];

#[derive(Copy, Clone)]
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

use self::CheatId::*;

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
