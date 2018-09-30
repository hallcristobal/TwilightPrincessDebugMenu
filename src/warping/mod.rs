pub mod main;
pub mod statics;
use super::get_state;
use libtp::warping::Entrance;

pub static mut warp_menu_state: WarpMenu = WarpMenu::Main;
pub static mut stage_type: StageType = StageType::Cave;
pub static mut saved_warp: Option<Entrance> = None;

pub fn transition_into() {
    match unsafe { warp_menu_state } {
        // WarpMenu::RoomSelection => room_selection::transition_into(),
        // WarpMenu::StageSelection => stage_selection::transition_into(),
        // WarpMenu::BrowseTop => browse_top::transition_into(),
        WarpMenu::Main => main::transition_into(),
        _ => {}
    }
}

pub fn render() {
    match unsafe { warp_menu_state } {
        // WarpMenu::RoomSelection => room_selection::render(),
        // WarpMenu::StageSelection => stage_selection::render(),
        // WarpMenu::BrowseTop => browse_top::render(),
        WarpMenu::Main => main::render(),
        _ => {}
    }
}

pub fn load_saved_warp() {
    unsafe {
        stage_type = StageType::SavedEntrance;
    }
    main::do_warp();
}

#[derive(Copy, Clone)]
pub enum WarpMenu {
    RoomSelection,
    StageSelection,
    BrowseTop,
    Main,
}

#[derive(Copy, Clone, PartialEq)]
pub enum StageType {
    Cave,
    Dungeon,
    Interior,
    Overworld,
    LastEntrance,
    SavedEntrance,
}

impl<'a> StageType {
    pub fn str(&self) -> &'a str {
        match *self {
            StageType::Cave => "Cave",
            StageType::Dungeon => "Dungeon",
            StageType::Interior => "Interior",
            StageType::Overworld => "Overworld",
            StageType::LastEntrance => "Last Entrance",
            StageType::SavedEntrance => "Saved Entrance",
        }
    }
}
