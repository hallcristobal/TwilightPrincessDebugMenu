#![no_std]

#![feature(panic_implementation, proc_macro_non_items)]
extern crate gcn;
extern crate gcn_fonts;

use gcn_fonts::prelude::*;
use gcn::gx;

pub mod print;

struct State {
    perspective: gx::Mtx44,
    font: UploadedFont,
}

static mut STATE: Option<State> = None;
static FONT: Font = include_font! { path: "res/Calamity-Bold.ttf", size: 20.0 };

unsafe fn get_state() -> &'static mut State {
    STATE.get_or_insert_with(|| {
        gx::set_z_mode(gx::ENABLE, gx::LEQUAL, gx::TRUE);

        let mut perspective: gx::Mtx44 = gx::Mtx44 {
            cells: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        };
        gu_ortho(&mut perspective, 0.0, 480.0, 0.0, 640.0, 0.0, 300.0);
        State {
            perspective,
            font: FONT.upload(),
        }
    })
}

fn gu_ortho(mt: &mut gx::Mtx44, t: f32, b: f32, l: f32, r: f32, n: f32, f: f32) {
    let tmp = 1.0 / (r - l);
    mt.cells[0][0] = 2.0 * tmp;
    mt.cells[0][1] = 0.0;
    mt.cells[0][2] = 0.0;
    mt.cells[0][3] = -(r + l) * tmp;

    let tmp = 1.0 / (t - b);
    mt.cells[1][0] = 0.0;
    mt.cells[1][1] = 2.0 * tmp;
    mt.cells[1][2] = 0.0;
    mt.cells[1][3] = -(t + b) * tmp;

    let tmp = 1.0 / (f - n);
    mt.cells[2][0] = 0.0;
    mt.cells[2][1] = 0.0;
    mt.cells[2][2] = -(1.0) * tmp;
    mt.cells[2][3] = -(f) * tmp;

    mt.cells[3][0] = 0.0;
    mt.cells[3][1] = 0.0;
    mt.cells[3][2] = 0.0;
    mt.cells[3][3] = 1.0;
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    print::setup_draw();
    print::printf("Hello from Rust!", 20.0, 20.0, 0xFF_FF_FF_FF, 0x77_77_77_FF);
}

use core::panic::PanicInfo;
#[panic_implementation]
#[no_mangle]
pub fn my_panic(_: &PanicInfo) -> ! {
    loop {}
}
