pub const DPAD_LEFT: u16 = 0x0001;
pub const DPAD_RIGHT: u16 = 0x0002;
pub const DPAD_DOWN: u16 = 0x0004;
pub const DPAD_UP: u16 = 0x0008;
pub const Z: u16 = 0x0010;
pub const R: u16 = 0x0020;
pub const L: u16 = 0x0040;
pub const A: u16 = 0x0100;
pub const B: u16 = 0x0200;
pub const X: u16 = 0x0400;
pub const Y: u16 = 0x0800;
pub const START: u16 = 0x1000;

pub fn buttons_down() -> u16 {
    unsafe { PAD_BUTTON.buttons }
}

pub fn buttons_pressed() -> u16 {
    unsafe { PAD_BUTTON.buttons_down }
}

pub fn set_buttons_down(buttons: u16) {
   unsafe { PAD_BUTTON.buttons =  buttons; }
}

pub fn set_buttons_pressed(buttons: u16) {
    unsafe { PAD_BUTTON.buttons_down  = buttons }
}

pub fn is_down(buttons: u16) -> bool {
    buttons_down() & buttons == buttons
}

pub fn is_pressed(buttons: u16) -> bool {
    buttons_pressed() & buttons == buttons
}

#[repr(C)]
pub struct PadStatus {
    pub sval: u16
}

#[repr(C)]
pub struct PadButton {
    pub analog_cardinal: u8,
    pub c_cardinal: u8,
    pub buttons: u16,
    _p1: [u8; 2],
    pub buttons_down: u16,
    _p2: [u8; 18],
    pub sval: u16
}

extern "C" {
    #[link_name = "JUTGamePad::mPadButton"]
    pub static mut PAD_BUTTON: PadButton;
    #[link_name = "JUTGamePad::mPadStatus"]
    pub static mut PAD_STATUS: PadStatus;
}
