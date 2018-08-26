use libtp::game::controller;
use libtp::system::{memory, ZEL_AUDIO, get_link_momentum};
use libtp::{addrs, Addr};

pub static mut doing_cheats: bool = false;

pub struct Command {
    pub active: bool,
    pub buttons: u16,
    pub command: fn(),
}

pub const STORE_POSITION: usize = 0;
pub const LOAD_POSITION: usize = 1;
pub const MOON_JUMP: usize = 2;

pub static mut COMMANDS: [Command; 3] = [
    Command {
        active: true,
        buttons: 0x0028,
        command: store_position,
    },
    Command {
        active: true,
        buttons: 0x0024,
        command: load_position,
    },
    Command {
        active: true,
        buttons: 0x0120,
        command: moon_jump,
    },
];

static mut saved_x: f32 = 0.0;
static mut saved_y: f32 = 0.0;
static mut saved_z: f32 = 0.0;
static mut saved_a: u16 = 0;

static mut saved_c0: f32 = 0.0;
static mut saved_c1: f32 = 0.0;
static mut saved_c2: f32 = 0.0;
static mut saved_c3: f32 = 0.0;
static mut saved_c4: f32 = 0.0;
static mut saved_c5: f32 = 0.0;
static mut saved_c6: f32 = 0.0;
static mut saved_c7: f32 = 0.0;

fn store_position() {
    unsafe {
        let camera_ptr = memory::read::<Addr>(addrs::MATRIX_PTR);
        if let Some(ref mut link) = ZEL_AUDIO.link_debug_ptr {
            saved_x = link.x;
            saved_y = link.y;
            saved_z = link.z;
            saved_a = link.facing;
        }
        if camera_ptr > 0x8000_0000 && camera_ptr < 0x8FFF_FFFF {
            saved_c0 = memory::read(camera_ptr + 0x009C);
            saved_c1 = memory::read(camera_ptr + 0x00A0);
            saved_c2 = memory::read(camera_ptr + 0x00A4);
            saved_c3 = memory::read(camera_ptr + 0x00A8);
            saved_c4 = memory::read(camera_ptr + 0x00AC);
            saved_c5 = memory::read(camera_ptr + 0x00B0);
            saved_c6 = memory::read(camera_ptr + 0x01E8);
            saved_c7 = memory::read(camera_ptr + 0x0448);
        }
        memory::write::<u8>(0x80B1_6D00, 0x05);
    }
}

fn load_position() {
    unsafe {
        let camera_ptr = memory::read::<Addr>(addrs::MATRIX_PTR);
        if let Some(ref mut link) = ZEL_AUDIO.link_debug_ptr {
            link.x = saved_x;
            link.y = saved_y;
            link.z = saved_z;
            link.facing = saved_a;
        }
        if camera_ptr > 0x8000_0000 && camera_ptr < 0x8FFF_FFFF {
            memory::write(camera_ptr + 0x009C, saved_c0);
            memory::write(camera_ptr + 0x00A0, saved_c1);
            memory::write(camera_ptr + 0x00A4, saved_c2);
            memory::write(camera_ptr + 0x00A8, saved_c3);
            memory::write(camera_ptr + 0x00AC, saved_c4);
            memory::write(camera_ptr + 0x00B0, saved_c5);
            memory::write(camera_ptr + 0x01E8, saved_c6);
            memory::write(camera_ptr + 0x0448, saved_c7);
        }
        memory::write::<u8>(0x80B1_6D00, 0x05);
    }
}

fn moon_jump() {
    if let Some(ref mut momentum) = get_link_momentum() {
        (*momentum).link_momentum.y = 56.0;
    }
}

pub fn process_inputs() {
    unsafe {
        COMMANDS.iter_mut().for_each(|c| {
            if c.active && controller::PAD_STATUS.sval == c.buttons {
                (c.command)();
                controller::set_buttons_down(0x0);
                controller::set_buttons_pressed(0x0);
                controller::PAD_STATUS.sval = 0x0;
                controller::PAD_BUTTON.sval = 0x0;
            }
        });
    }
}
