use {controller, get_state, utils::*};
use super::{inv_menu_state, InventoryMenu};
use libtp::link::{Link, Inventory};
use core::fmt::Write;

static mut cursor: usize = 0;
static mut scroll_offset: usize = 0;

const ORDON_SWORD_INDEX: usize = 0;
const MASTER_SWORD_INDEX: usize = 1;
const WOODEN_SHIELD_INDEX: usize = 2;
const HYLIAN_SHIELD_INDEX: usize = 3;
const HEROS_CLOTHES_INDEX: usize = 4;
const ZORA_ARMOR_INDEX: usize = 5;
const MAGIC_ARMOR_INDEX: usize = 6;
const BOMB_UPGRADE_INDEX: usize = 7;
const WALLET_INDEX: usize = 8;
const QUIVER_INDEX: usize = 9;

const ITEMS: [&str; 10] = [
	"Ordon Sword",
	"Master Sword",
	"Wooden Shield",
    "Hylian Shield",
	"Hero's Clothes",
	"Zora Armor",
	"Magic Armor",
    "Bomb Upgrade",
	"Wallet",
	"Quiver",
];

pub fn transition_into() {}

fn handle_item_switch() {
    let dpad_left = controller::DPAD_LEFT.is_pressed();
    let dpad_right = controller::DPAD_RIGHT.is_pressed();
    let inventory = Inventory::get_inventory();
    let link = Link::get_link();
    if dpad_left {
        match unsafe {cursor} {
            ORDON_SWORD_INDEX => {
                    inventory.ordon_sword_flag().deactivate();
            }
            MASTER_SWORD_INDEX => {
                let light = inventory.light_sword_flag();
                let master = inventory.master_sword_flag();
                if light.is_active() && master.is_active() {
                    light.deactivate();
                } else if !light.is_active() && master.is_active() {
                    master.deactivate();
                }
            }
            WOODEN_SHIELD_INDEX => {
                let ordon = inventory.ordon_shield_flag();
                let wooden = inventory.wooden_shield_flag();
                if !ordon.is_active() && wooden.is_active() {
                    wooden.deactivate();
                    ordon.activate();
                } else if ordon.is_active() && !wooden.is_active() {
                    ordon.deactivate();
                }
            }
            HYLIAN_SHIELD_INDEX => {
                    inventory.hylian_shield_flag().deactivate()
            }
            HEROS_CLOTHES_INDEX => {
                    inventory.heros_clothes_flag().deactivate();
            }
            ZORA_ARMOR_INDEX => {
                    inventory.zora_armor_flag().deactivate();
            }
            MAGIC_ARMOR_INDEX => {
                    inventory.magic_armor_flag().deactivate();
            }
            BOMB_UPGRADE_INDEX => {
                    inventory.bomb_capacity_flag().deactivate();
            }
            WALLET_INDEX => {
                if link.wallet_upgrade == 2 {
                    link.wallet_upgrade = 1;
                } else if link.wallet_upgrade == 1 {
                    link.wallet_upgrade = 0;
                }
            }
            QUIVER_INDEX => {
                if inventory.arrow_capacity == 100 {
                    inventory.arrow_capacity = 60;
                } else if inventory.arrow_capacity == 60 {
                    inventory.arrow_capacity = 30;
                }
             }
            _ => unreachable!()
        };
    } else if dpad_right {
        match unsafe {cursor} {
            ORDON_SWORD_INDEX => {
                    inventory.ordon_sword_flag().activate();
            }
            MASTER_SWORD_INDEX => {
                let light = inventory.light_sword_flag();
                let master = inventory.master_sword_flag();
                if !light.is_active() && !master.is_active() {
                    master.activate();
                } else if !light.is_active() && master.is_active() {
                    light.activate();
                }
            }
            WOODEN_SHIELD_INDEX => {
                let ordon = inventory.ordon_shield_flag();
                let wooden = inventory.wooden_shield_flag();
                if !ordon.is_active() && !wooden.is_active() {
                    ordon.activate();
                } else if ordon.is_active() && !wooden.is_active() {
                    wooden.activate();
                    ordon.deactivate();
                }
            }
            HYLIAN_SHIELD_INDEX => {
                    inventory.hylian_shield_flag().activate()
            }
            HEROS_CLOTHES_INDEX => {
                    inventory.heros_clothes_flag().activate();
            }
            ZORA_ARMOR_INDEX => {
                    inventory.zora_armor_flag().activate();
            }
            MAGIC_ARMOR_INDEX => {
                    inventory.magic_armor_flag().activate();
            }
            BOMB_UPGRADE_INDEX => {
                    inventory.bomb_capacity_flag().activate();
            }
            WALLET_INDEX => {
                if link.wallet_upgrade == 0 {
                    link.wallet_upgrade = 1;
                } else if link.wallet_upgrade == 1 {
                    link.wallet_upgrade = 2;
                }
            }
            QUIVER_INDEX => {
                if inventory.arrow_capacity == 30 {
                    inventory.arrow_capacity = 60;
                } else if inventory.arrow_capacity == 60 {
                    inventory.arrow_capacity = 100;
                }
             }
            _ => unreachable!()
        };
    }
}

pub fn render() {
    let state = unsafe { get_state() };
    let menu = &mut state.menu;
    let inventory = Inventory::get_inventory();
    let link = Link::get_link();
    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        unsafe {
            inv_menu_state = InventoryMenu::Main;
        }
        transition(MenuState::InventoryMenu);
        return;
    }

    unsafe {
        scroll_move_cursor(ITEMS.len(), &mut cursor, &mut scroll_offset);
    }

    handle_item_switch();

    let lines = menu.lines_mut();
    for (index, (line, &text)) in lines
        .into_iter()
        .zip(ITEMS.iter().skip(unsafe { scroll_offset }))
        .enumerate()
        .take(state.settings.max_lines)
    {
        let index = index + unsafe { scroll_offset };
        let _ = write!(line.begin(), "{}", text);
        let _ = match index {
            ORDON_SWORD_INDEX => {
                write!(line.begin(), "Ordon Sword: {}", if inventory.ordon_sword_flag().is_active() {
                    "Ordon Sword"
                } else {
                    ""
                })
            }
            MASTER_SWORD_INDEX => {
                write!(line.begin(), "Master Sword: {}", if inventory.light_sword_flag().is_active() {
                    "Light Sword"
                } else if inventory.master_sword_flag().is_active() {
                    "Master Sword"
                } else {
                    ""
                })
            }
            WOODEN_SHIELD_INDEX => {
                write!(line.begin(), "Wooden Shield: {}", if inventory.wooden_shield_flag().is_active() {
                    "Wooden Shield"
                } else if inventory.ordon_shield_flag().is_active() {
                    "Ordon Shield"
                } else {
                    ""
                })
            }
            HYLIAN_SHIELD_INDEX => {
                write!(line.begin(), "Hylian Shield: {}", if inventory.hylian_shield_flag().is_active() {
                    "Hylian Shield"
                } else {
                    ""
                })
            }
            HEROS_CLOTHES_INDEX => {
                write!(line.begin(), "Hero's Clothes: {}", if inventory.heros_clothes_flag().is_active() {
                    "Hero's Clothes"
                } else {
                    ""
                })
            }
            ZORA_ARMOR_INDEX => {
                write!(line.begin(), "Zora Armor: {}", if inventory.zora_armor_flag().is_active() {
                    "Zora Armor"
                } else {
                    ""
                })
            }
            MAGIC_ARMOR_INDEX => {
                write!(line.begin(), "Magic Armor: {}", if inventory.magic_armor_flag().is_active() {
                    "Magic Armor"
                } else {
                    ""
                })
            }
            BOMB_UPGRADE_INDEX => {
                write!(line.begin(), "Bomb Capacity: {}", if inventory.bomb_capacity_flag().is_active() {
                    "60/30/20"
                } else {
                    "30/15/10"
                })
            }
            WALLET_INDEX => {
                write!(line.begin(), "Wallet Upgrade: {}", if link.wallet_upgrade == 0 {
                    "300 Rupee Wallet"
                } else if link.wallet_upgrade == 1 {
                    "600 Rupee Wallet"
                } else {
                    "1000 Rupee Wallet"
                })
            }
            QUIVER_INDEX => {
                write!(line.begin(), "Arrow Capacity: {} Arrows", inventory.arrow_capacity)
            }
            _ => unreachable!()
        };
		if unsafe { index  == cursor } {
			line.selected = true;
		}
    }
}
