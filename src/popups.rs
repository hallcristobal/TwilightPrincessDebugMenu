use arrayvec::ArrayString;
use core::fmt::Write;
use libtp::system::get_frame_count;
use libtp::system::memory::read;
use libtp::Addr;
use print;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flag(pub Addr, pub u8);
static mut global_flags_cache: [u8; 420] = [0; 420]; // 804069B0 -> 80406B53
static mut end_frame: u32 = 0;
pub static mut visible: bool = false;
static mut flag: Flag = Flag(0, 0);

pub fn check_global_flags() {
    if unsafe { visible } && get_frame_count() > unsafe { end_frame } {
        unsafe {
            visible = false;
        }
    }

    for (index, cached_value) in unsafe { global_flags_cache.iter_mut().enumerate() } {
        let addr = 0x804069B0 + index;
        let current_value = read::<u8>(addr);
        let diff = current_value & (0xFF ^ *cached_value);
        if diff != 0 {
            for bit in 0..8 {
                if diff & (1 << bit) != 0 {
                    show_popup(addr, bit);
                    report!("Flag {:02X} {} has been set", 0xFF & addr, bit);
                }
            }
            *cached_value |= diff;
        }
    }
}

fn show_popup(addr: Addr, bit: u8) {
    unsafe {
        end_frame = get_frame_count() + 200;
        visible = true;
        flag = Flag(addr, bit);
    }
}

pub fn draw_popup() {
    if unsafe { visible } {
        let Flag(addr, bit) = unsafe { flag };
        let mut string = ArrayString::<[u8; 10]>::new();
        let _ = write!(string, "Flag {:02X} {} has been set", 0xFF & addr, bit);
        unsafe {
            print::printf(string.as_str(), 50.5, 50.5, 0xFF_FF_FF_FF);
        }
    }
}
