use super::super::get_state;
use super::super::utils::{scroll_move_cursor, transition, MenuState};
use core::fmt::Write;
use libtp::link::inventory::Inventory;
use libtp::link::item::*;

use super::{inv_menu_state, InventoryMenu};
use controller;

static mut cursor: usize = 0;

#[derive(Copy, Clone, Debug)]
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
use libtp::link::inventory::*;

const ITEM_SLOTS: [(&str, ItemType, usize); 23] = [
    (
        "Double Clawshots:",
        SingleItem(DOUBLE_CLAWSHOTS),
        DOUBLE_CLAWSHOT_ID_VALUE,
    ),
    (
        "Dominion Rod:",
        SingleItem(DOMINION_ROD),
        DOMINION_ROD_ID_VALUE,
    ),
    (
        "Ball Chain:",
        SingleItem(BALL_CHAIN),
        BALL_AND_CHAIN_ID_VALUE,
    ),
    ("Spinner:", SingleItem(SPINNER), SPINNER_ID_VALUE),
    ("Heros Bow:", SingleItem(HEROS_BOW), HEROS_BOW_ID_VALUE),
    ("Iron Boots:", SingleItem(IRON_BOOTS), IRON_BOOTS_ID_VALUE),
    ("Boomerang:", SingleItem(BOOMERANG), GALE_BOOMERANG_ID_VALUE),
    ("Lantern:", SingleItem(LANTERN), LANTERN_ID_VALUE),
    ("Slingshot:", SingleItem(SLINGSHOT), SLINGSHOT_ID_VALUE),
    ("Clawshot:", SingleItem(CLAWSHOT), CLAWSHOT_ID_VALUE),
    ("Fishing Rod:", FishingRod, FISHING_ROD_ID_VALUE),
    ("Hawkeye:", SingleItem(HAWKEYE), HAWKEYE_ID_VALUE),
    ("Bomb Bag1 1:", Bombs, BOMB_BAG_1_ID_VALUE),
    ("Bomb Bag1 2:", Bombs, BOMB_BAG_2_ID_VALUE),
    ("Bomb Bag1 3:", Bombs, BOMB_BAG_3_ID_VALUE),
    ("Bottle 1:", Bottle, BOTTLE_1_ID_VALUE),
    ("Bottle 2:", Bottle, BOTTLE_2_ID_VALUE),
    ("Bottle 3:", Bottle, BOTTLE_3_ID_VALUE),
    ("Bottle 4:", Bottle, BOTTLE_4_ID_VALUE),
    ("Sky Book: ", SkyBook, SKY_BOOK_ID_VALUE),
    ("Ilia Quest:", IliaQuest, ILIA_QUEST_ID_VALUE),
    ("Ooccoo: ", Ooccoo, OOCCOO_ID_VALUE),
    ("Letter: ", Letter, LETTER_ID_VALUE),
];

static mut scroll_offset: usize = 0;

pub fn transition_into() {}

fn handle_item_switch() {
    let dpad_left = controller::DPAD_LEFT.is_pressed();
    let dpad_right = controller::DPAD_RIGHT.is_pressed();
    let inventory = Inventory::get_inventory();
    let (_, item_type, item_slot) = ITEM_SLOTS[unsafe { cursor }];
    let item_id = inventory.get_by_slot_id(item_slot);
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
        inventory.set_by_slot_id(item_slot, new_item_id);
    } else if dpad_right {
        let new_item_id = match (item_type, item_id) {
            (Bottle, EMPTY) => EMPTY_BOTTLE,
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
            (Bombs, EMPTY) => EMPTY_BOMB_BAG,
            (Bombs, EMPTY_BOMB_BAG) => REGULAR_BOMBS,
            (Bombs, REGULAR_BOMBS) => WATER_BOMBS,
            (Bombs, WATER_BOMBS) => BOMBLINGS,
            (Ooccoo, EMPTY) => OOCCOO,
            (Ooccoo, OOCCOO) => OOCCOO_JR,
            (Ooccoo, OOCCOO_JR) => OOCCOOS_NOTE,
            (Letter, EMPTY) => AURUS_MEMO,
            (Letter, AURUS_MEMO) => ASHEIS_SKETCH,
            (FishingRod, EMPTY) => FISHING_ROD,
            (FishingRod, FISHING_ROD) => FISHING_ROD_BEE_LARVA,
            (FishingRod, FISHING_ROD_BEE_LARVA) => FISHING_ROD_CORAL_EARRING,
            (FishingRod, FISHING_ROD_CORAL_EARRING) => FISHING_ROD_WORM,
            (FishingRod, FISHING_ROD_WORM) => FISHING_ROD_EARRING_BEE_LARVA,
            (FishingRod, FISHING_ROD_EARRING_BEE_LARVA) => FISHING_ROD_EARRING_WORM,
            (IliaQuest, EMPTY) => RENARDOS_LETTER,
            (IliaQuest, RENARDOS_LETTER) => INVOICE,
            (IliaQuest, INVOICE) => WOODEN_STATUE,
            (IliaQuest, WOODEN_STATUE) => ILIAS_CHARM,
            (IliaQuest, ILIAS_CHARM) => HORSE_CALL,
            (SkyBook, EMPTY) => SKY_BOOK_EMPTY,
            (SkyBook, SKY_BOOK_EMPTY) => SKY_BOOK_PARTLY_FILLED,
            (SkyBook, SKY_BOOK_PARTLY_FILLED) => SKY_BOOK_FILLED,
            (SingleItem(x), _) => x,
            (_, x) => x,
        };
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
    unsafe {
        scroll_move_cursor(ITEM_SLOTS.len(), &mut cursor, &mut scroll_offset);
    }

    handle_item_switch();

    let lines = menu.lines_mut();
    for (index, (line, &(text, _, slot_index))) in lines
        .into_iter()
        .zip(ITEM_SLOTS.iter().skip(unsafe { scroll_offset }))
        .enumerate()
        .take(state.settings.max_lines)
    {
        let index = index + unsafe { scroll_offset };
        let item_id = inventory.get_by_slot_id(slot_index);
        let item_text = item_id_to_str(item_id);
        let _ = write!(line.begin(), "{} {}", text, item_text);
        if unsafe { index == cursor } {
            line.selected = true;
        }
    }
}
