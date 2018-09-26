// pub mod main;
pub mod statics;
use self::statics::Stage;
use super::get_state;

pub static mut warp_menu_state: WarpMenu = WarpMenu::Main;
pub static mut stage_type: StageType = StageType::Cave;

pub fn transition_into() {
    match unsafe { warp_menu_state } {
        // WarpMenu::RoomSelection => room_selection::transition_into(),
        // WarpMenu::StageSelection => stage_selection::transition_into(),
        // WarpMenu::BrowseTop => browse_top::transition_into(),
        // WarpMenu::Main => main::transition_into(),
        _ => {}
    }
}

pub fn render() {
    match unsafe { warp_menu_state } {
        // WarpMenu::RoomSelection => room_selection::render(),
        // WarpMenu::StageSelection => stage_selection::render(),
        // WarpMenu::BrowseTop => browse_top::render(),
        // WarpMenu::Main => main::render(),
        _ => {}
    }
}

#[derive(Copy, Clone)]
pub enum WarpMenu {
    RoomSelection,
    StageSelection,
    BrowseTop,
    Main,
}

#[derive(Copy, Clone)]
pub enum StageType {
    Cave,
    Dungeon,
    Interior,
    Overworld,
}

impl<'a> StageType {
    pub fn str(&self) -> &'a str {
        match *self {
            StageType::Cave => "Cave",
            StageType::Dungeon => "Dungeon",
            StageType::Interior => "Interior",
            StageType::Overworld => "Overworld",
        }
    }

    // pub fn areas(&self) -> &[(&Stage, &str)] {
    //     match *self {
    //         StageType::Cave => &statics::cave::STAGES,
    //         StageType::Dungeon => &statics::dungeon::STAGES,
    //         StageType::Interior => &statics::interior::STAGES,
    //         StageType::Overworld => &statics::overworld::STAGES,
    //     }
    // }
}
