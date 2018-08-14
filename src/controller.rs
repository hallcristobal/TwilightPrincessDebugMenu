use libtp::game::controller;
use libtp::system;

use visible as debug_menu_visible;

pub const DPAD_LEFT: Button = Button(0);
pub const DPAD_RIGHT: Button = Button(1);
pub const DPAD_DOWN: Button = Button(2);
pub const DPAD_UP: Button = Button(3);
pub const Z: Button = Button(4);
pub const R: Button = Button(5);
pub const L: Button = Button(6);
pub const A: Button = Button(7);
pub const B: Button = Button(8);
pub const X: Button = Button(9);
pub const Y: Button = Button(10);
pub const START: Button = Button(11);

const REPEAT_TIME: u32 = 6;
const REPEAT_DELAY: u32 = 10;

pub struct Button(usize);

impl Button {
    pub fn is_pressed(&self) -> bool {
        let delta = unsafe { system::get_frame_count() - button_states[self.0].pressed_frame };
        let just_clicked = delta == 0;
        let held_down_long_enough = delta > REPEAT_DELAY;
        let is_repeat_frame = held_down_long_enough && delta % REPEAT_TIME == 0;

        self.is_down() && (just_clicked || is_repeat_frame)
    }

    pub fn is_down(&self) -> bool {
        unsafe { button_states[self.0].is_down }
    }
}

static mut button_states: [ButtonState; 12] = [
    ButtonState::new(controller::DPAD_LEFT),
    ButtonState::new(controller::DPAD_RIGHT),
    ButtonState::new(controller::DPAD_DOWN),
    ButtonState::new(controller::DPAD_UP),
    ButtonState::new(controller::Z),
    ButtonState::new(controller::R),
    ButtonState::new(controller::L),
    ButtonState::new(controller::A),
    ButtonState::new(controller::B),
    ButtonState::new(controller::X),
    ButtonState::new(controller::Y),
    ButtonState::new(controller::START),
];

struct ButtonState {
    button: u16,
    pressed_frame: u32,
    is_down: bool,
}

impl ButtonState {
    const fn new(button: u16) -> Self {
        ButtonState {
            button: button,
            pressed_frame: 0xFFFFFFFF,
            is_down: false,
        }
    }
}

static mut buttons_pressed: u16 = 0;
static mut buttons_down: u16 = 0;
static mut buttons_down_last_frame: u16 = 0;

#[no_mangle]
pub extern "C" fn read_controller() -> u32 {
    unsafe {
        buttons_down_last_frame = buttons_down;
        buttons_down = controller::PAD_STATUS.sval;
        buttons_pressed = buttons_down & (0xFFFF ^ buttons_down_last_frame);

        for state in &mut button_states {
            state.is_down = state.button & buttons_down != 0;
            if state.button & buttons_pressed != 0 {
                state.pressed_frame = system::get_frame_count() + 1;
            }
        }
    }
    if unsafe { debug_menu_visible } {
        controller::set_buttons_down(0x0);
        controller::set_buttons_pressed(0x0);
        unsafe {
            controller::PAD_STATUS.sval = 0x0;
            controller::PAD_BUTTON.sval = 0x0;
        }
    }
    0x80000000
}
