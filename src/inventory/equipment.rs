use super::super::get_state;
use core::fmt::Write;
use libtp::link::inventory::Inventory;
use libtp::link::item::*;
use super::super::utils::{MenuState, transition};

use super::{inv_menu_state, InventoryMenu};
use controller;

static mut cursor: usize = 0;

pub fn scroll_move_cursor() {
    if controller::DPAD_UP.is_pressed() && unsafe { cursor } > 0 {
        unsafe {
            cursor -= 1;
            if cursor >= 4 && cursor - 4 < scroll_offset {
                scroll_offset = cursor - 4;
            }
        }
    } else if controller::DPAD_DOWN.is_pressed() && unsafe { cursor + 1 } < ITEM_SLOTS.len() {
        unsafe {
            cursor += 1;
            if cursor + 4 < ITEM_SLOTS.len() && cursor > scroll_offset + 20 {
                scroll_offset = cursor - 20;
            }
        }
    }
}

#[derive(Copy, Clone)]
enum ItemType {
    SingleItem(u8),
    Bottle,
    SkyBook,
    IliaQuest,
    FishingRod,
    Letter,
    Ooccoo,
    Bombs,
}

use self::ItemType::*;

const ITEM_SLOTS: [(&str, ItemType); 23] = [
    ("Double Clawshots:", SingleItem(DOUBLE_CLAWSHOTS)),
    ("Dominion Rod:", SingleItem(DOMINION_ROD)),
    ("Ball Chain:", SingleItem(BALL_CHAIN)),
    ("Spinner:", SingleItem(SPINNER)),
    ("Heros Bow:", SingleItem(HEROS_BOW)),
    ("Iron Boots:", SingleItem(IRON_BOOTS)),
    ("Boomerang:", SingleItem(BOOMERANG)),
    ("Lantern:", SingleItem(LANTERN)),
    ("Slingshot:", SingleItem(SLINGSHOT)),
    ("Clawshot:", SingleItem(CLAWSHOT)),
    ("Fishing Rod:", FishingRod),
    ("Hawkeye:", SingleItem(HAWKEYE)),
    ("Bomb Bag1 1:", Bombs),
    ("Bomb Bag1 2:", Bombs),
    ("Bomb Bag1 3:", Bombs),
    ("Bottle 1:", Bottle),
    ("Bottle 2:", Bottle),
    ("Bottle 3:", Bottle),
    ("Bottle 4:", Bottle),
    ("Sky Book: ", SkyBook),
    ("Ilia Quest:", IliaQuest),
    ("Ooccoo: ", Ooccoo),
    ("Letter: ", Letter),
];

static mut scroll_offset: usize = 0;

pub fn transition_into() {}

fn handle_item_switch() {
    let dpad_left = controller::DPAD_LEFT.is_pressed();
    let dpad_right = controller::DPAD_RIGHT.is_pressed();
    let item_slot = unsafe { cursor };
    let inventory = Inventory::get_inventory();
    let item_id = inventory.get_by_slot_id(item_slot);
    let (_, item_type) = ITEM_SLOTS[item_slot];
    if dpad_left {
        let new_item_id = match (item_type, item_id) {
            (Bottle, RED_POTION) => EMPTY_BOTTLE,
            (Bottle, GREEN_POTION) => RED_POTION,
            (Bottle, BLUE_POTION) => GREEN_POTION,
            (Bottle, MILK) => BLUE_POTION,
            (Bottle, MILK_HALF) => MILK,
            (Bottle, LANTERN_OIL) => MILK_HALF,
            (Bottle, WATER) => LANTERN_OIL,
            (Bottle, NASTY_SOUP) => WATER,
            (Bottle, HOT_SPRINGWATER) => NASTY_SOUP,
            (Bottle, FAIRY) => HOT_SPRINGWATER,
            (Bottle, FAIRY_TEARS) => FAIRY,
            (Bottle, WORM) => FAIRY_TEARS,
            (Bottle, BEE_LARVA) => WORM,
            (Bottle, RARE_CHU_JELLY) => BEE_LARVA,
            (Bottle, RED_CHU_JELLY) => RARE_CHU_JELLY,
            (Bottle, BLUE_CHU_JELLY) => RED_CHU_JELLY,
            (Bottle, GREEN_CHU_JELLY) => BLUE_CHU_JELLY,
            (Bottle, YELLOW_CHU_JELLY) => GREEN_CHU_JELLY,
            (Bottle, PURPLE_CHU_JELLY) => YELLOW_CHU_JELLY,
            (Bottle, SIMPLE_SOUP) => PURPLE_CHU_JELLY,
            (Bottle, GOOD_SOUP) => SIMPLE_SOUP,
            (Bottle, SUPERB_SOUP) => GOOD_SOUP,
            (Bottle, BLACK_CHU_JELLY) => SUPERB_SOUP,
            (Bombs, REGULAR_BOMBS) => EMPTY_BOMB_BAG,
            (Bombs, WATER_BOMBS) => REGULAR_BOMBS,
            (Bombs, BOMBLINGS) => WATER_BOMBS,
            (Ooccoo, OOCCOO_JR) => OOCCOO,
            (Ooccoo, OOCCOOS_NOTE) => OOCCOO_JR,
            (Letter, ASHEIS_SKETCH) => AURUS_MEMO,
            (FishingRod, FISHING_ROD_BEE_LARVA) => FISHING_ROD,
            (FishingRod, FISHING_ROD_CORAL_EARRING) => FISHING_ROD_BEE_LARVA,
            (FishingRod, FISHING_ROD_WORM) => FISHING_ROD_CORAL_EARRING,
            (FishingRod, FISHING_ROD_EARRING_BEE_LARVA) => FISHING_ROD_WORM,
            (FishingRod, FISHING_ROD_EARRING_WORM) => FISHING_ROD_EARRING_BEE_LARVA,
            (IliaQuest, INVOICE) => RENARDOS_LETTER,
            (IliaQuest, WOODEN_STATUE) => INVOICE,
            (IliaQuest, ILIAS_CHARM) => WOODEN_STATUE,
            (IliaQuest, HORSE_CALL) => ILIAS_CHARM,
            (SkyBook, SKY_BOOK_PARTLY_FILLED) => SKY_BOOK_EMPTY,
            (SkyBook, SKY_BOOK_FILLED) => SKY_BOOK_PARTLY_FILLED,
            _ => EMPTY,
        };
        if new_item_id == 0xFF {
            report!("Setting: {} to 0xFF", item_slot);
        }
        inventory.set_by_slot_id(item_slot, new_item_id);
    } else if dpad_right {
        let new_item_id = match (item_type, item_id) {
            (Bottle, EMPTY_BOTTLE) => RED_POTION,
            (Bottle, RED_POTION) => GREEN_POTION,
            (Bottle, GREEN_POTION) => MILK,
            (Bottle, MILK) => MILK_HALF,
            (Bottle, MILK_HALF) => LANTERN_OIL,
            (Bottle, LANTERN_OIL) => WATER,
            (Bottle, WATER) => NASTY_SOUP,
            (Bottle, NASTY_SOUP) => HOT_SPRINGWATER,
            (Bottle, HOT_SPRINGWATER) => FAIRY,
            (Bottle, FAIRY) => FAIRY_TEARS,
            (Bottle, FAIRY_TEARS) => WORM,
            (Bottle, WORM) => BEE_LARVA,
            (Bottle, BEE_LARVA) => RARE_CHU_JELLY,
            (Bottle, RARE_CHU_JELLY) => RED_CHU_JELLY,
            (Bottle, RED_CHU_JELLY) => BLUE_CHU_JELLY,
            (Bottle, BLUE_CHU_JELLY) => GREEN_CHU_JELLY,
            (Bottle, GREEN_CHU_JELLY) => YELLOW_CHU_JELLY,
            (Bottle, YELLOW_CHU_JELLY) => PURPLE_CHU_JELLY,
            (Bottle, PURPLE_CHU_JELLY) => SIMPLE_SOUP,
            (Bottle, SIMPLE_SOUP) => GOOD_SOUP,
            (Bottle, GOOD_SOUP) => SUPERB_SOUP,
            (Bottle, SUPERB_SOUP) => BLACK_CHU_JELLY,
            (Bombs, EMPTY_BOMB_BAG) => REGULAR_BOMBS,
            (Bombs, REGULAR_BOMBS) => WATER_BOMBS,
            (Bombs, WATER_BOMBS) => BOMBLINGS,
            (Ooccoo, OOCCOO) => OOCCOO_JR,
            (Ooccoo, OOCCOO_JR) => OOCCOOS_NOTE,
            (Letter, AURUS_MEMO) => ASHEIS_SKETCH,
            (FishingRod, FISHING_ROD) => FISHING_ROD_BEE_LARVA,
            (FishingRod, FISHING_ROD_BEE_LARVA) => FISHING_ROD_CORAL_EARRING,
            (FishingRod, FISHING_ROD_CORAL_EARRING) => FISHING_ROD_WORM,
            (FishingRod, FISHING_ROD_WORM) => FISHING_ROD_EARRING_BEE_LARVA,
            (FishingRod, FISHING_ROD_EARRING_BEE_LARVA) => FISHING_ROD_EARRING_WORM,
            (IliaQuest, RENARDOS_LETTER) => INVOICE,
            (IliaQuest, INVOICE) => WOODEN_STATUE,
            (IliaQuest, WOODEN_STATUE) => ILIAS_CHARM,
            (IliaQuest, ILIAS_CHARM) => HORSE_CALL,
            (SkyBook, SKY_BOOK_EMPTY) => SKY_BOOK_PARTLY_FILLED,
            (SkyBook, SKY_BOOK_PARTLY_FILLED) => SKY_BOOK_FILLED,
            (SingleItem(x), _) => x,
            (_, x) => x,
        };
        if new_item_id == 0xFF {
            report!("Setting: {} to 0xFF", item_slot);
        }
        inventory.set_by_slot_id(item_slot, new_item_id);
    }
}

pub fn render() {
    let state = unsafe { get_state() };
    let menu = &mut state.menu;
    let inventory = Inventory::get_inventory();
    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        unsafe {
            inv_menu_state = InventoryMenu::Main;
        }
        transition(MenuState::InventoryMenu);
        return;
    }

    scroll_move_cursor();

    handle_item_switch();

    let lines = menu.lines_mut();
    for (index, (line, &(text, _))) in lines
        .into_iter()
        .zip(ITEM_SLOTS.iter().skip(unsafe { scroll_offset }))
        .enumerate()
        .take(25)
    {
        let index = index + unsafe { scroll_offset };
        let item_id = inventory.get_by_slot_id(index);
        let item_text = item_id_to_str(item_id);
        let _ = write!(line.begin(), "{} {}", text, item_text);
		if unsafe { index - scroll_offset == cursor } {
			line.selected = true;
		}
    }
}
