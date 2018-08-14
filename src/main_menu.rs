use core::fmt::Write;

use controller;
use utils::*;
use {get_state, visible};

static mut cursor: usize = 0;

pub fn transition_into() {}

pub fn render() {
    const MEMORY_INDEX: usize = 0;

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

    let contents = ["Memory"];

    move_cursor(contents.len(), unsafe { &mut cursor });

    if pressed_a {
        if unsafe { cursor } == MEMORY_INDEX {
            transition(MenuState::Memory);
            return;
        }
    }

    for (index, (line, &content)) in lines.iter_mut().zip(&contents).enumerate() {
        let _ = write!(line.begin(), "{}", content);
        line.selected = index == unsafe { cursor };
    }
}
