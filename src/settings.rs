use super::get_state;
use arrayvec::ArrayVec;
use controller;
use core::fmt::Write;
use gcn::card::Card;
use utils::*;

use cheat_menu::CHEAT_AMNT;
use memory::{Watch, MAX_WATCH};

#[derive(Serialize, Deserialize)]
pub struct SaveLayout(ArrayVec<[Watch; MAX_WATCH]>, ArrayVec<[bool; CHEAT_AMNT]>);

pub struct Settings {
    pub drop_shadow: bool,
    pub max_lines: usize,
}

static mut cursor: usize = 0;

pub fn transition_into() {}

pub fn unpack_save(save: SaveLayout) {
    let mut items = ::memory::ITEMS.borrow_mut();
    items.clear();
    items.extend(save.0);

    unsafe {
        ::cheat_menu::load_cheats(save.1);
    }
}

pub fn render() {
    const CARD_SAVE: usize = 0;
    const CARD_LOAD: usize = 1;

    let state = unsafe { get_state() };
    let lines = state.menu.lines_mut();
    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        transition(MenuState::MainMenu);
        return;
    }

    let contents = ["Save Card", "Load Card"];

    move_cursor(contents.len(), unsafe { &mut cursor });

    if pressed_a {
        unsafe {
            report!("Hello");
            match cursor {
                CARD_SAVE => {
                    let cheats = ::cheat_menu::get_cheats();
                    let items = ::memory::ITEMS.borrow();
                    let active_c = cheats
                        .iter()
                        .map(|c| c.active)
                        .collect::<ArrayVec<[_; CHEAT_AMNT]>>();
                    match Card::create("tpgz01") {
                        Err(e) => report!("Failed to create card: {:?}", e),
                        Ok(mut card) => {
                            match card.serialize_write(&SaveLayout(items.clone(), active_c)) {
                                Ok(_) => report!("Wrote to mem card"),
                                Err(e) => report!("Failed to write mem card: {:?}", e),
                            }
                        }
                    }
                }
                CARD_LOAD => match Card::open("tpgz01") {
                    Ok(mut card) => match card.deserialize_read(unpack_save) {
                        Ok(_) => {
                            report!("Read mem card");
                        }
                        Err(e) => report!("Failed to read mem card: {:?}", e),
                    },
                    Err(e) => report!("Failed to open mem card: {:?}", e),
                },
                _ => unreachable!(),
            }
        }
    }

    for (index, (line, &content)) in lines.iter_mut().zip(&contents).enumerate() {
        let _ = write!(line.begin(), "{}", content);
        line.selected = index == unsafe { cursor };
    }
}
