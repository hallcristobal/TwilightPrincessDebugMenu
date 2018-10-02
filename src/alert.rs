use arrayvec::ArrayString;
use core::cell::RefCell;
use core::fmt::{Arguments, Write};
use libtp::system::get_frame_count;
use libtp::system::mutex::Mutex;
use print;

lazy_static! {
    static ref message: Mutex<ArrayString<[u8; 64]>> = Mutex(RefCell::new(ArrayString::new()));
}
static mut end_frame: u32 = 0;
pub static mut visible: bool = false;

pub fn check_alert_popup() {
    if unsafe { visible } && get_frame_count() > unsafe { end_frame } {
        unsafe {
            visible = false;
            message.borrow_mut().clear();
        }
    }
}

pub fn show_alert(args: &Arguments) {
    unsafe {
        end_frame = get_frame_count() + 200;
        visible = true;
        message.borrow_mut().clear();
        write!(message.borrow_mut(), "{}\0", args);
    }
}

pub fn draw_alert() {
    if unsafe { visible } {
        unsafe {
            print::printf(message.borrow().as_str(), 50.5, 50.5, 0xFF_55_55_FF);
        }
    }
}
