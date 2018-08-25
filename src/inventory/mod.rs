// mod amounts;
mod equipment;
mod main;
// mod map;
// mod progression;
// mod quest;

static mut inv_menu_state: InventoryMenu = InventoryMenu::Main;

pub fn transition_into() {
    match unsafe { inv_menu_state } {
        InventoryMenu::Main => main::transition_into(),
        InventoryMenu::Equipment => equipment::transition_into(),
        // InventoryMenu::Amounts => amounts::transition_into(),
        // InventoryMenu::Quest => quest::transition_into(),
        // InventoryMenu::Progression => progression::transition_into(),
    }
}

pub fn render() {
    match unsafe { inv_menu_state } {
        InventoryMenu::Main => main::render(),
        InventoryMenu::Equipment => equipment::render(),
        // InventoryMenu::Amounts => amounts::render(),
        // InventoryMenu::Quest => quest::render(),
        // InventoryMenu::Progression => progression::render(),
    }
}

#[derive(Clone, Copy)]
enum InventoryMenu {
    Main,
    Equipment,
    // Amounts,
    // Quest,
    // Progression,
}
