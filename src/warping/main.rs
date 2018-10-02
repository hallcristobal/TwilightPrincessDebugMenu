use core::fmt::Write;
use core::str;
use libtp::warping::{Entrance, Warp};
use visible;

use super::*;
use controller;
use utils::*;

static mut cursor: usize = 0;
static mut area_index: usize = 0;
static mut room_index: usize = 0;
static mut spawn_index: usize = 0;
static mut state_override: u8 = 0xFF;
static mut last_entrance: Entrance = Entrance::default();

const TYPE: usize = 0;
const AREA: usize = 1;
const ROOM: usize = 2;
const SPAWN_POINT: usize = 3;
const STATE: usize = 4;
const EXECUTE: usize = 6;
const SAVE_WARP: usize = 7;
const LAST_AREA: usize = 10;
const LAST_ROOM: usize = 11;
const LAST_SPAWN_POINT: usize = 12;
const LAST_STATE: usize = 13;

pub fn transition_into() {
    unsafe {
        last_entrance = Warp::last_warp().entrance;
    }
}

pub fn do_warp() -> bool {
    let (stage_id, room_id, spawn_id) = unsafe {
        match stage_type {
            StageType::Cave => {
                let area = statics::cave::CaveStage::from(area_index);
                let area_id = area.get_id();
                let rooms = area.get_rooms();
                let spawn_points = rooms[room_index].spawn_points;
                (area_id, rooms[room_index].id, spawn_points[spawn_index])
            }
            StageType::Dungeon => {
                let area = statics::dungeon::DungeonStage::from(area_index);
                let area_id = area.get_id();
                let rooms = area.get_rooms();
                let spawn_points = rooms[room_index].spawn_points;
                (area_id, rooms[room_index].id, spawn_points[spawn_index])
            }
            StageType::Interior => {
                let area = statics::interior::InteriorStage::from(area_index);
                let area_id = area.get_id();
                let rooms = area.get_rooms();
                let spawn_points = rooms[room_index].spawn_points;
                (area_id, rooms[room_index].id, spawn_points[spawn_index])
            }
            StageType::Overworld => {
                let area = statics::overworld::OverworldStage::from(area_index);
                let area_id = area.get_id();
                let rooms = area.get_rooms();
                let spawn_points = rooms[room_index].spawn_points;
                (area_id, rooms[room_index].id, spawn_points[spawn_index])
            }
            StageType::LastEntrance => {
                let mut warp = Warp::new("", 0xFF, 0xFF, 0xFF);
                warp.entrance = last_entrance.clone();
                warp.execute();
                return true;
            }
            StageType::SavedEntrance => {
                if let Some(ref saved) = saved_warp {
                    let mut warp = Warp::new("", 0xFF, 0xFF, 0xFF);
                    warp.entrance = saved.clone();
                    warp.execute();
                    return true;
                } else {
                    return false;
                }
            }
        }
    };

    Warp::new(stage_id, room_id, spawn_id, unsafe { state_override }).execute();
    return true;
}

fn handle_input() -> bool {
    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();
    let dpad_left = controller::DPAD_LEFT.is_pressed();
    let dpad_right = controller::DPAD_RIGHT.is_pressed();

    let (area_count, room_count, spawn_point_count) = unsafe {
        match stage_type {
            StageType::Cave => {
                let areas = statics::cave::CaveStage::from(area_index);
                let rooms = areas.get_rooms();
                let spawn_points = rooms[room_index].spawn_points;
                (areas.len(), rooms.len(), spawn_points.len())
            }
            StageType::Dungeon => {
                let areas = statics::dungeon::DungeonStage::from(area_index);
                let rooms = areas.get_rooms();
                let spawn_points = rooms[room_index].spawn_points;
                (areas.len(), rooms.len(), spawn_points.len())
            }
            StageType::Interior => {
                let areas = statics::interior::InteriorStage::from(area_index);
                let rooms = areas.get_rooms();
                let spawn_points = rooms[room_index].spawn_points;
                (areas.len(), rooms.len(), spawn_points.len())
            }
            StageType::Overworld => {
                let areas = statics::overworld::OverworldStage::from(area_index);
                let rooms = areas.get_rooms();
                let spawn_points = rooms[room_index].spawn_points;
                (areas.len(), rooms.len(), spawn_points.len())
            }
            StageType::LastEntrance | StageType::SavedEntrance => (0, 0, 0),
        }
    };

    if pressed_b {
        return true;
    } else if pressed_a {
        if unsafe { cursor } == EXECUTE {
            if do_warp() {
                unsafe {
                    visible = false;
                }
                return true;
            }
        } else if unsafe { cursor } == SAVE_WARP {
            unsafe {
                saved_warp = Some(last_entrance.clone());
            }
        }
    } else if dpad_left {
        unsafe {
            match cursor {
                TYPE => {
                    stage_type = match stage_type {
                        StageType::Dungeon => StageType::Cave,
                        StageType::Interior => StageType::Dungeon,
                        StageType::Overworld => StageType::Interior,
                        StageType::LastEntrance => StageType::Overworld,
                        StageType::SavedEntrance => StageType::LastEntrance,
                        StageType::Cave => StageType::SavedEntrance,
                    };
                    area_index = 0;
                    room_index = 0;
                    spawn_index = 0;
                    state_override = 0xFF;
                }
                AREA => {
                    if area_index > 0 {
                        area_index -= 1;
                    }
                    room_index = 0;
                    spawn_index = 0;
                    state_override = 0xFF;
                }
                ROOM => {
                    if room_index > 0 {
                        room_index -= 1;
                    }
                    spawn_index = 0;
                    state_override = 0xFF;
                }
                SPAWN_POINT => {
                    if spawn_index > 0 {
                        spawn_index -= 1;
                    }
                    state_override = 0xFF;
                }
                STATE => {
                    if stage_type != StageType::SavedEntrance
                        && stage_type != StageType::LastEntrance
                    {
                        if state_override > 0 && state_override < 0xFF {
                            state_override -= 1;
                        } else if state_override == 0 {
                            state_override = 0xFF
                        }
                    }
                }
                _ => {}
            }
        }
    } else if dpad_right {
        unsafe {
            match cursor {
                TYPE => {
                    stage_type = match stage_type {
                        StageType::SavedEntrance => StageType::Cave,
                        StageType::Cave => StageType::Dungeon,
                        StageType::Dungeon => StageType::Interior,
                        StageType::Interior => StageType::Overworld,
                        StageType::Overworld => StageType::LastEntrance,
                        StageType::LastEntrance => StageType::SavedEntrance,
                    };
                    area_index = 0;
                    room_index = 0;
                    spawn_index = 0;
                    state_override = 0xFF;
                }
                AREA => {
                    if area_index < area_count - 1 {
                        area_index += 1;
                    }
                    room_index = 0;
                    spawn_index = 0;
                    state_override = 0xFF;
                }
                ROOM => {
                    if room_index < room_count - 1 {
                        room_index += 1;
                    }
                    spawn_index = 0;
                    state_override = 0xFF;
                }
                SPAWN_POINT => {
                    if spawn_index < spawn_point_count - 1 {
                        spawn_index += 1;
                    }
                    state_override = 0xFF;
                }
                STATE => {
                    if stage_type != StageType::SavedEntrance
                        && stage_type != StageType::LastEntrance
                    {
                        if state_override < 0xF {
                            state_override += 1;
                        } else if state_override == 0xFF {
                            state_override = 0;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    return false;
}

pub fn render() {
    let state = unsafe { super::get_state() };
    let lines = state.menu.lines_mut();

    let contents = [
        "Type",
        "Area",
        "Room",
        "Spawn Point",
        "State",
        "",
        "Execute",
        "Save Entrance",
        "",
        "Last Entrance:",
        "Area",
        "Room",
        "Spawn Point",
        "State",
    ];

    if handle_input() {
        unsafe {
            warp_menu_state = WarpMenu::Main;
        }
        transition(MenuState::MainMenu);
        return;
    }
    move_cursor(contents.len() - 6, unsafe { &mut cursor });

    let (area_name, room_name, spawn_point) = unsafe {
        match stage_type {
            StageType::Cave => {
                let areas = statics::cave::STAGES;
                let area_name = areas[area_index];
                let area = statics::cave::CaveStage::from(area_index);
                let rooms = area.get_rooms();
                let spawn_points = rooms[room_index].spawn_points;
                (area_name, rooms[room_index].name, spawn_points[spawn_index])
            }
            StageType::Dungeon => {
                let areas = statics::dungeon::STAGES;
                let area_name = areas[area_index];
                let area = statics::dungeon::DungeonStage::from(area_index);
                let rooms = area.get_rooms();
                let spawn_points = rooms[room_index].spawn_points;
                (area_name, rooms[room_index].name, spawn_points[spawn_index])
            }
            StageType::Interior => {
                let areas = statics::interior::STAGES;
                let area_name = areas[area_index];
                let area = statics::interior::InteriorStage::from(area_index);
                let rooms = area.get_rooms();
                let spawn_points = rooms[room_index].spawn_points;
                (area_name, rooms[room_index].name, spawn_points[spawn_index])
            }
            StageType::Overworld => {
                let areas = statics::overworld::STAGES;
                let area_name = areas[area_index];
                let area = statics::overworld::OverworldStage::from(area_index);
                let rooms = area.get_rooms();
                let spawn_points = rooms[room_index].spawn_points;
                (area_name, rooms[room_index].name, spawn_points[spawn_index])
            }
            StageType::LastEntrance => (
                str::from_utf8(&last_entrance.stage).unwrap_or(""),
                "",
                last_entrance.spawn,
            ),
            StageType::SavedEntrance => {
                if let Some(ref _saved_warp) = saved_warp {
                    (
                        str::from_utf8(&_saved_warp.stage).unwrap_or(""),
                        "",
                        _saved_warp.spawn,
                    )
                } else {
                    ("No Saved Warp", "", 0xFF)
                }
            }
        }
    };

    let type_string = unsafe { stage_type.str() };

    let last = unsafe { &last_entrance };
    let last_stage = str::from_utf8(&last.stage).unwrap_or("");

    for (index, (line, &content)) in lines.iter_mut().zip(&contents).enumerate() {
        let _ = match index {
            TYPE => write!(line.begin(), "{}: {}", content, type_string),
            AREA => write!(line.begin(), "{}: {}", content, area_name),
            ROOM => if unsafe { stage_type == StageType::LastEntrance } {
                write!(line.begin(), "{}: {}", content, unsafe {
                    last_entrance.room
                })
            } else if unsafe { stage_type == StageType::SavedEntrance } {
                if let Some(_saved_warp) = unsafe { saved_warp.clone() } {
                    write!(line.begin(), "{}: {}", content, _saved_warp.room)
                } else {
                    write!(line.begin(), "{}: N/A", content)
                }
            } else {
                write!(line.begin(), "{}: {}", content, room_name)
            },
            SPAWN_POINT => write!(line.begin(), "{}: {}", content, spawn_point),
            STATE => write!(line.begin(), "{}: {:X}", content, unsafe { state_override }),
            LAST_AREA => write!(line.begin(), "{}: {}", content, last_stage),
            LAST_ROOM => write!(line.begin(), "{}: {}", content, last.room),
            LAST_SPAWN_POINT => write!(line.begin(), "{}: {}", content, last.spawn),
            LAST_STATE => write!(line.begin(), "{}: {:X}", content, last.state),
            _ => write!(line.begin(), "{}", content),
        };

        line.selected = index == unsafe { cursor };
    }
}
