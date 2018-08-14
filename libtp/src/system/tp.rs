use super::memory;

#[repr(C)]
pub struct GlobalCounters {
    pub game_counter: u32,
    pub game_counter2: u32,
    pub non_menu_counter: u32,
}

#[repr(C)]
pub struct ZelAudio {
    _p0: [u8; 0xF08],
    pub link_debug_ptr: Option<&'static mut LinkDebug>
}

#[repr(C)]
pub struct LinkDebug {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    _p0: [u8; 0xA],
    pub facing: u16,
    _p1: [u8; 0x44],
    pub speed: f32
}

extern "C" {
    #[link_name = "g_Counter"]
    pub static mut GLOBAL_COUNTERS: GlobalCounters;
    #[link_name = "g_mDoAud_zelAudio"]
    pub static mut ZEL_AUDIO: ZelAudio;
}

pub fn get_frame_count() -> u32 {
    unsafe { GLOBAL_COUNTERS.game_counter }
}

pub fn get_link_debug() -> Option<&'static mut LinkDebug> {
    unsafe {
        if let Some(ref mut link_debug) = ZEL_AUDIO.link_debug_ptr {
            Some(*link_debug)
        } else {
            None
        }
    }
}
