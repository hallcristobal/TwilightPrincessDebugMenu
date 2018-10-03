use arrayvec::ArrayVec;
use controller;
use core::fmt::Write;
use gcn::card::Card;
use get_state;
use utils::*;

use cheat_menu::{cheats, cheats_mut, CHEAT_AMNT};
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

pub fn defaults() {
    use memory::{Type, ITEMS};
    let mut items = ITEMS.borrow_mut();
    items.clear();
    items.push(Watch::new(
        0x803dce54,
        430.0,
        295.0,
        Type::f32,
        false,
        true,
        Some(0x5c),
    ));
    items.push(Watch::new(
        0x803dce54,
        430.0,
        315.0,
        Type::u16,
        false,
        true,
        Some(0x16),
    ));
    items.push(Watch::new(
        0x803dce54,
        450.0,
        340.0,
        Type::f32,
        false,
        true,
        Some(0x0),
    ));
    items.push(Watch::new(
        0x803dce54,
        450.0,
        360.0,
        Type::f32,
        false,
        true,
        Some(0x4),
    ));
    items.push(Watch::new(
        0x803dce54,
        450.0,
        380.0,
        Type::f32,
        false,
        true,
        Some(0x8),
    ));
    items.push(Watch::new(
        0x8040afc0,
        410.0,
        330.0,
        Type::String,
        false,
        false,
        None,
    ));
    items.push(Watch::new(
        0x8042d3e0,
        410.0,
        330.0,
        Type::u8,
        false,
        false,
        None,
    ));
    items.push(Watch::new(
        0x803a66b3,
        410.0,
        330.0,
        Type::i8,
        false,
        false,
        None,
    ));
    items.push(Watch::new(
        0x8040afc9,
        410.0,
        330.0,
        Type::u8,
        true,
        false,
        None,
    ));
    items.push(Watch::new(
        0x80450c98,
        410.0,
        330.0,
        Type::u8,
        false,
        false,
        None,
    ));
    items.push(Watch::new(
        0x8000_0000,
        410.0,
        330.0,
        Type::ToD,
        false,
        false,
        None,
    ));
    unsafe {
        let cheats = cheats_mut();
        cheats.iter_mut().for_each(|c| c.active = false);
    }
}

pub fn render() {
    const CARD_SAVE: usize = 0;
    const CARD_LOAD: usize = 1;
    const DEFAULTS: usize = 2;
    const DROP_SHADOW: usize = 4;

    let state = unsafe { get_state() };
    let lines = state.menu.lines_mut();
    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        transition(MenuState::MainMenu);
        return;
    }

    let contents = ["Save Card", "Load Card", "Restore Defaults", "", "Drop Shadow"];

    move_cursor(contents.len(), unsafe { &mut cursor });

    if pressed_a {
        unsafe {
            report!("Hello");
            match cursor {
                CARD_SAVE => {
                    let cheats = cheats();
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
                        Ok(_) => report!("Read mem card"),
                        Err(e) => report!("Failed to read mem card: {:?}", e),
                    },
                    Err(e) => report!("Failed to open mem card: {:?}", e),
                },
                DEFAULTS => {
                    defaults();
                }
                DROP_SHADOW => {
                    state.settings.drop_shadow = !state.settings.drop_shadow;
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }

    for (index, (line, &content)) in lines.iter_mut().zip(&contents).enumerate() {
        let _ = match index {
            DROP_SHADOW => write!(line.begin(), "{}: {}", content, state.settings.drop_shadow),
            _ => write!(line.begin(), "{}", content)
        };
        line.selected = index == unsafe { cursor };
    }
}
