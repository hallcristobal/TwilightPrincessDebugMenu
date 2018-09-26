use core::fmt::Write;
use libtp::warping::Warp;
use visible;

use super::*;
use controller;
use utils::*;

static mut cursor: usize = 0;

pub fn transition_into() {}

fn shorten(s: &str) -> &str {
    if s.len() > 45 {
        &s[..45]
    } else {
        s
    }
}

static mut area_index: usize = 0;
static mut room_index: usize = 0;
static mut spawn_index: usize = 0;

pub fn render() {
    let state = unsafe { super::get_state() };
    let lines = state.menu.lines_mut();

    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();
    let dpad_left = controller::DPAD_LEFT.is_pressed();
    let dpad_right = controller::DPAD_RIGHT.is_pressed();

    if pressed_b {
        unsafe {
            warp_menu_state = WarpMenu::Main;
        }
        transition(MenuState::MainMenu);
        return;
    }

    let contents = [
        "Type",
        "Area",
        "Room",
        "Spawn Point",
        "State",
        "",
        "Execute",
    ];

    move_cursor(contents.len(), unsafe { &mut cursor });

    let type_string = unsafe { stage_type.str() };
    let areas = unsafe { stage_type.areas() };
    let (area, area_name) = areas[unsafe { area_index }];
    let rooms = area.get_rooms();
    let spawn_points = rooms[unsafe { room_index }].spawn_points;
    let states = shorten("state");

    for (index, (line, &content)) in lines.iter_mut().zip(&contents).enumerate() {
        unsafe {
            let _ = match index {
                0 => write!(line.begin(), "{}: {}", content, type_string),
                1 => write!(line.begin(), "{}: {}", content, area_name),
                2 => write!(line.begin(), "{}: {}", content, rooms[room_index].name),
                3 => write!(line.begin(), "{}: {}", content, spawn_points[spawn_index]),
                4 => write!(line.begin(), "{}: {}", content, states),
                _ => Ok(()),
            };
        };

        line.selected = index == unsafe { cursor };
    }
}
