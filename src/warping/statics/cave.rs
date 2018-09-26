#![allow(dead_code)]
use super::*;

#[derive(PartialEq, Clone)]
pub enum CaveStage {
    LanayruIcePuzzleCave,
    CaveOfOrdeals,
    EldinLongCave,
    LakeHyliaLongCave,
    EldinGoronStockcave,
    Grotto1,
    Grotto2,
    Grotto3,
    Grotto4,
    Grotto5,
    FaronWoodsCave,
}

impl Stage for CaveStage {
    fn get_id(&self) -> &'static str {
        match *self {
            CaveStage::LanayruIcePuzzleCave => "D_SB00",
            CaveStage::CaveOfOrdeals => "D_SB01",
            CaveStage::EldinLongCave => "D_SB02",
            CaveStage::LakeHyliaLongCave => "D_SB03",
            CaveStage::EldinGoronStockcave => "D_SB04",
            CaveStage::Grotto1 => "D_SB05",
            CaveStage::Grotto2 => "D_SB06",
            CaveStage::Grotto3 => "D_SB07",
            CaveStage::Grotto4 => "D_SB08",
            CaveStage::Grotto5 => "D_SB09",
            CaveStage::FaronWoodsCave => "D_SB10",
        }
    }

    fn get_rooms(&self) -> &'static [Room<'static>] {
        match *self {
            CaveStage::LanayruIcePuzzleCave => &LANAYRU_ICE_PUZZLE_CAVE,
            CaveStage::CaveOfOrdeals => &CAVE_OF_ORDEALS,
            CaveStage::EldinLongCave => &ELDIN_LONG_CAVE,
            CaveStage::LakeHyliaLongCave => &LAKE_HYLIA_LONG_CAVE,
            CaveStage::EldinGoronStockcave => &ELDIN_GORON_STOCKCAVE,
            CaveStage::Grotto1 => &GROTTO_1,
            CaveStage::Grotto2 => &GROTTO_2,
            CaveStage::Grotto3 => &GROTTO_3,
            CaveStage::Grotto4 => &GROTTO_4,
            CaveStage::Grotto5 => &GROTTO_5,
            CaveStage::FaronWoodsCave => &FARON_WOODS_CAVE,
        }
    }
}

pub static STAGES: [(&Stage, &str); 11] = [
    (&CaveStage::LanayruIcePuzzleCave, "Lanayru Ice Puzzle Cave"),
    (&CaveStage::CaveOfOrdeals, "Cave of Ordeals"),
    (&CaveStage::EldinLongCave, "Eldin Long Cave"),
    (&CaveStage::LakeHyliaLongCave, "Lake Hylia Long Cave"),
    (&CaveStage::EldinGoronStockcave, "Eldin Goron Stockcave"),
    (&CaveStage::Grotto1, "Grotto 1"),
    (&CaveStage::Grotto2, "Grotto 2"),
    (&CaveStage::Grotto3, "Grotto 3"),
    (&CaveStage::Grotto4, "Grotto 4"),
    (&CaveStage::Grotto5, "Grotto 5"),
    (&CaveStage::FaronWoodsCave, "Faron Woods Cave"),
];

pub static LANAYRU_ICE_PUZZLE_CAVE: [Room; 1] = [Room::new("Lanayru Ice Puzzle Cave", 0, &[0])];
pub static CAVE_OF_ORDEALS: [Room; 50] = [
    Room::new("Room 1", 0, &[0]),
    Room::new("Room 2", 1, &[0]),
    Room::new("Room 3", 2, &[0]),
    Room::new("Room 4", 3, &[0]),
    Room::new("Room 5", 4, &[0]),
    Room::new("Room 6", 5, &[0]),
    Room::new("Room 7", 6, &[0]),
    Room::new("Room 8", 7, &[0]),
    Room::new("Room 9", 8, &[0, 1]),
    Room::new("Room 10", 9, &[0]),
    Room::new("Room 11", 10, &[0]),
    Room::new("Room 12", 11, &[0]),
    Room::new("Room 13", 12, &[0]),
    Room::new("Room 14", 13, &[0]),
    Room::new("Room 15", 14, &[0]),
    Room::new("Room 16", 15, &[0]),
    Room::new("Room 17", 16, &[0]),
    Room::new("Room 18", 17, &[0]),
    Room::new("Room 19", 18, &[0, 1]),
    Room::new("Room 20", 19, &[0]),
    Room::new("Room 21", 20, &[0]),
    Room::new("Room 22", 21, &[0]),
    Room::new("Room 23", 22, &[0]),
    Room::new("Room 24", 23, &[0]),
    Room::new("Room 25", 24, &[0]),
    Room::new("Room 26", 25, &[0]),
    Room::new("Room 27", 26, &[0]),
    Room::new("Room 28", 27, &[0]),
    Room::new("Room 29", 28, &[0, 1]),
    Room::new("Room 30", 29, &[0]),
    Room::new("Room 31", 30, &[0]),
    Room::new("Room 32", 31, &[0]),
    Room::new("Room 33", 32, &[0]),
    Room::new("Room 34", 33, &[0]),
    Room::new("Room 35", 34, &[0]),
    Room::new("Room 36", 35, &[0]),
    Room::new("Room 37", 36, &[0]),
    Room::new("Room 38", 37, &[0]),
    Room::new("Room 39", 38, &[0, 1]),
    Room::new("Room 40", 39, &[0]),
    Room::new("Room 41", 40, &[0]),
    Room::new("Room 42", 41, &[0]),
    Room::new("Room 43", 42, &[0]),
    Room::new("Room 44", 43, &[0]),
    Room::new("Room 45", 44, &[0]),
    Room::new("Room 46", 45, &[0]),
    Room::new("Room 47", 46, &[0]),
    Room::new("Room 48", 47, &[0]),
    Room::new("Room 49", 48, &[0, 1]),
    Room::new("Room 50", 49, &[0]),
];
pub static ELDIN_LONG_CAVE: [Room; 1] = [Room::new("Eldin Long Cave", 0, &[0])];
pub static LAKE_HYLIA_LONG_CAVE: [Room; 1] = [Room::new("Lake Hylia Long Cave", 0, &[0, 1])];
pub static ELDIN_GORON_STOCKCAVE: [Room; 1] = [Room::new("Eldin Goron Stockcave", 10, &[0, 1])];
pub static GROTTO_1: [Room; 1] = [Room::new("Grotto 1", 0, &[0, 1])];
pub static GROTTO_2: [Room; 1] = [Room::new("Grotto 2", 1, &[0, 1])];
pub static GROTTO_3: [Room; 1] = [Room::new("Grotto 3", 2, &[0, 1])];
pub static GROTTO_4: [Room; 1] = [Room::new("Grotto 4", 3, &[0, 1])];
pub static GROTTO_5: [Room; 1] = [Room::new("Grotto 5", 4, &[0, 1])];
pub static FARON_WOODS_CAVE: [Room; 1] = [Room::new("Faron Woods Cave", 0, &[0, 21, 20, 1])];
