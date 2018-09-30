#![allow(dead_code)]
use super::*;

#[derive(PartialEq, Clone)]
pub enum DungeonStage {
    LakebedTemple,
    Morpheel,
    DekuToad,
    GoronMines,
    Fyrus,
    Dangoro,
    ForestTemple,
    Diababa,
    Ook,
    TempleOfTime,
    Armogohma,
    Darknut,
    CityInTheSky,
    Argorok,
    Aeralfos,
    PalaceOfTwilight,
    ZantMainRoom,
    PhantomZant1,
    PhantomZant2,
    ZantFight,
    HyruleCastle,
    GanondorfCastle,
    GanondorfField,
    GanondorfDefeated,
    ArbitersGrounds,
    Stallord,
    DeathSword,
    SnowpeakRuins,
    Blizzeta,
    Darkhammer,
}

impl Stage for DungeonStage {
    fn get_id(&self) -> &'static str {
        match *self {
            DungeonStage::LakebedTemple => "D_MN01",
            DungeonStage::Morpheel => "D_MN01A",
            DungeonStage::DekuToad => "D_MN01B",
            DungeonStage::GoronMines => "D_MN04",
            DungeonStage::Fyrus => "D_MN04A",
            DungeonStage::Dangoro => "D_MN04B",
            DungeonStage::ForestTemple => "D_MN05",
            DungeonStage::Diababa => "D_MN05A",
            DungeonStage::Ook => "D_MN05B",
            DungeonStage::TempleOfTime => "D_MN06",
            DungeonStage::Armogohma => "D_MN06A",
            DungeonStage::Darknut => "D_MN06B",
            DungeonStage::CityInTheSky => "D_MN07",
            DungeonStage::Argorok => "D_MN07A",
            DungeonStage::Aeralfos => "D_MN07B",
            DungeonStage::PalaceOfTwilight => "D_MN08",
            DungeonStage::ZantMainRoom => "D_MN08A",
            DungeonStage::PhantomZant1 => "D_MN08B",
            DungeonStage::PhantomZant2 => "D_MN08C",
            DungeonStage::ZantFight => "D_MN08D",
            DungeonStage::HyruleCastle => "D_MN09",
            DungeonStage::GanondorfCastle => "D_MN09A",
            DungeonStage::GanondorfField => "D_MN09B",
            DungeonStage::GanondorfDefeated => "D_MN09C",
            DungeonStage::ArbitersGrounds => "D_MN10",
            DungeonStage::Stallord => "D_MN10A",
            DungeonStage::DeathSword => "D_MN10B",
            DungeonStage::SnowpeakRuins => "D_MN11",
            DungeonStage::Blizzeta => "D_MN11A",
            DungeonStage::Darkhammer => "D_MN11B",
        }
    }

    fn get_rooms(&self) -> &'static [Room<'static>] {
        match *self {
            DungeonStage::LakebedTemple => &LAKEBED_TEMPLE,
            DungeonStage::Morpheel => &MORPHEEL,
            DungeonStage::DekuToad => &DEKU_TOAD,
            DungeonStage::GoronMines => &GORON_MINES,
            DungeonStage::Fyrus => &FYRUS,
            DungeonStage::Dangoro => &DANGORO,
            DungeonStage::ForestTemple => &FOREST_TEMPLE,
            DungeonStage::Diababa => &DIABABA,
            DungeonStage::Ook => &OOK,
            DungeonStage::TempleOfTime => &TEMPLE_OF_TIME,
            DungeonStage::Armogohma => &ARMOGOHMA,
            DungeonStage::Darknut => &DARKNUT,
            DungeonStage::CityInTheSky => &CITY_IN_THE_SKY,
            DungeonStage::Argorok => &ARGOROK,
            DungeonStage::Aeralfos => &AERALFOS,
            DungeonStage::PalaceOfTwilight => &PALACE_OF_TWILIGHT,
            DungeonStage::ZantMainRoom => &ZANT_MAIN_ROOM,
            DungeonStage::PhantomZant1 => &PHANTOM_ZANT_1,
            DungeonStage::PhantomZant2 => &PHANTOM_ZANT_2,
            DungeonStage::ZantFight => &ZANT_FIGHT,
            DungeonStage::HyruleCastle => &HYRULE_CASTLE,
            DungeonStage::GanondorfCastle => &GANONDORF_CASTLE,
            DungeonStage::GanondorfField => &GANONDORF_FIELD,
            DungeonStage::GanondorfDefeated => &GANONDORF_DEFEATED,
            DungeonStage::ArbitersGrounds => &ARBITERS_GROUNDS,
            DungeonStage::Stallord => &STALLORD,
            DungeonStage::DeathSword => &DEATH_SWORD,
            DungeonStage::SnowpeakRuins => &SNOWPEAK_RUINS,
            DungeonStage::Blizzeta => &BLIZZETA,
            DungeonStage::Darkhammer => &DARKHAMMER,
        }
    }
    fn len(&self) -> usize {
        30
    }
}

impl From<usize> for DungeonStage {
    fn from(i: usize) -> Self {
        match i {
            0 => DungeonStage::LakebedTemple,
            1 => DungeonStage::Morpheel,
            2 => DungeonStage::DekuToad,
            3 => DungeonStage::GoronMines,
            4 => DungeonStage::Fyrus,
            5 => DungeonStage::Dangoro,
            6 => DungeonStage::ForestTemple,
            7 => DungeonStage::Diababa,
            8 => DungeonStage::Ook,
            9 => DungeonStage::TempleOfTime,
            10 => DungeonStage::Armogohma,
            11 => DungeonStage::Darknut,
            12 => DungeonStage::CityInTheSky,
            13 => DungeonStage::Argorok,
            14 => DungeonStage::Aeralfos,
            15 => DungeonStage::PalaceOfTwilight,
            16 => DungeonStage::ZantMainRoom,
            17 => DungeonStage::PhantomZant1,
            18 => DungeonStage::PhantomZant2,
            19 => DungeonStage::ZantFight,
            20 => DungeonStage::HyruleCastle,
            21 => DungeonStage::GanondorfCastle,
            22 => DungeonStage::GanondorfField,
            23 => DungeonStage::GanondorfDefeated,
            24 => DungeonStage::ArbitersGrounds,
            25 => DungeonStage::Stallord,
            26 => DungeonStage::DeathSword,
            27 => DungeonStage::SnowpeakRuins,
            28 => DungeonStage::Blizzeta,
            29 => DungeonStage::Darkhammer,
            _ => unreachable!(),
        }
    }
}

pub static STAGES: [&str; 30] = [
    "Lakebed Temple",
    "Morpheel",
    "Deku Toad",
    "Goron Mines",
    "Fyrus",
    "Dangoro",
    "Forest Temple",
    "Diababa",
    "Ook",
    "Temple of Time",
    "Armogohma",
    "Darknut",
    "City in the Sky",
    "Argorok",
    "Aeralfos",
    "Palace of Twilight",
    "Zant Main Room",
    "Phantom Zant 1",
    "Phantom Zant 2",
    "Zant Fight",
    "Hyrule Castle",
    "Ganondorf Castle",
    "Ganondorf Field",
    "Ganondorf Defeated",
    "Arbiters Grounds",
    "Stallord",
    "Death Sword",
    "Snowpeak Ruins",
    "Blizzeta",
    "Darkhammer",
];

pub static LAKEBED_TEMPLE: [Room; 13] = [
    Room::new("Entrance", 0, &[0, 1, 2]),
    Room::new("Stalactite Room", 1, &[0]),
    Room::new("Central Room Outside", 2, &[0]),
    Room::new("Central Room", 3, &[0, 1, 2]),
    Room::new("Before Boss Key", 5, &[0, 1, 2]),
    Room::new("Boss Key", 6, &[0, 1, 2]),
    Room::new("Right Wing Upper", 7, &[0]),
    Room::new("Right Wing Lower", 8, &[0, 2]),
    Room::new("Before Deku Toad", 9, &[0, 1, 2, 3, 4]),
    Room::new("Right Wing Water Supply", 10, &[0, 1]),
    Room::new("Left Wing Upper", 11, &[0]),
    Room::new("Left Wing Lower", 12, &[0, 1, 2]),
    Room::new("Left Wing Water Supply", 13, &[0]),
];
pub static MORPHEEL: [Room; 1] = [Room::new("Morpheel", 50, &[0, 1, 2, 3])];
pub static DEKU_TOAD: [Room; 1] = [Room::new("Deku Toad", 51, &[1, 2, 0, 3])];
pub static GORON_MINES: [Room; 13] = [
    Room::new("Entrance", 1, &[1, 0]),
    Room::new("Magnet Room", 3, &[0]),
    Room::new("Roll Clipping", 4, &[0, 1]),
    Room::new("Before 1st Elder", 5, &[0]),
    Room::new("Clawshot Switch", 6, &[0, 1]),
    Room::new("Outside", 7, &[0, 1]),
    Room::new("Before Dangoro", 9, &[0, 1, 2, 3]),
    Room::new("Bow", 11, &[1, 0]),
    Room::new("Before Fyrus", 12, &[0, 1]),
    Room::new("Bow-Magnet Shortcut Room", 13, &[0]),
    Room::new("1st Elder", 14, &[0, 1]),
    Room::new("3rd Elder", 16, &[0]),
    Room::new("2nd Elder", 17, &[0, 1]),
];
pub static FYRUS: [Room; 1] = [Room::new("Fyrus", 50, &[0, 1])];
pub static DANGORO: [Room; 1] = [Room::new("Dangoro", 51, &[2, 1, 0, 3])];
pub static FOREST_TEMPLE: [Room; 13] = [
    Room::new("Main Room", 0, &[0]),
    Room::new("Right Wing Boss Key", 1, &[0]),
    Room::new("2nd Monkey", 2, &[0]),
    Room::new("Left Wing", 3, &[0, 1]),
    Room::new("Outside", 4, &[1, 0, 2]),
    Room::new("3rd Monkey", 5, &[0, 1]),
    Room::new("4th Monkey", 7, &[0]),
    Room::new("North Wing Turning Bridge", 9, &[0]),
    Room::new("Final Monkey", 10, &[0]),
    Room::new("6th Monkey", 11, &[0]),
    Room::new("Before Diababa", 12, &[0, 1]),
    Room::new("7th Monkey", 19, &[0]),
    Room::new("Entrance", 22, &[0]),
];
pub static DIABABA: [Room; 1] = [Room::new("Diababa", 50, &[0, 1])];
pub static OOK: [Room; 1] = [Room::new("Ook", 51, &[1, 0, 2])];
pub static TEMPLE_OF_TIME: [Room; 9] = [
    Room::new("Entrance", 0, &[0, 1]),
    Room::new("First Staircase", 1, &[0]),
    Room::new("Turning Platform", 2, &[0, 1, 2]),
    Room::new("Statue Throws", 3, &[0, 1]),
    Room::new("Second Staircase", 4, &[0, 1]),
    Room::new("Scale Room", 5, &[0]),
    Room::new("Boss Key", 6, &[0]),
    Room::new("Third Staircase", 7, &[0, 1, 2]),
    Room::new("Before Gohma", 8, &[0, 1, 2]),
];
pub static ARMOGOHMA: [Room; 1] = [Room::new("Armogohma", 50, &[0, 1])];
pub static DARKNUT: [Room; 1] = [Room::new("Darknut", 51, &[0])];
pub static CITY_IN_THE_SKY: [Room; 16] = [
    Room::new("Entrance", 0, &[2, 1, 0, 3, 4, 5]),
    Room::new("Before Main Room", 1, &[0]),
    Room::new("Main Room", 2, &[0, 1, 2, 3, 4]),
    Room::new("Right Wing Outside", 3, &[1, 0, 2, 3]),
    Room::new("Right Wing Inside 1", 4, &[0, 1, 2]),
    Room::new("Before Aeralfos", 5, &[0, 1, 2]),
    Room::new("Left Wing Outside ", 6, &[0, 1, 2, 3, 4, 5, 6, 7, 8]),
    Room::new("Right Wing Inside 2", 7, &[0]),
    Room::new("Right Wing Inside 3", 8, &[0]),
    Room::new("Left Wing Inside", 10, &[0, 1, 3]),
    Room::new("Big Baba Room", 11, &[0, 1]),
    Room::new("After Big Baba Outside", 12, &[0, 1, 2, 3]),
    Room::new("Before Boss Key Outside", 13, &[0, 1]),
    Room::new("North Wing Outside", 14, &[0, 1, 3]),
    Room::new("North Wing Inside", 15, &[0, 1, 3, 4]),
    Room::new("Oocca Shop", 16, &[0, 1, 2]),
];
pub static ARGOROK: [Room; 1] = [Room::new("Argorok", 50, &[1, 0, 2, 3])];
pub static AERALFOS: [Room; 1] = [Room::new("Aeralfos", 51, &[0, 1, 2])];
pub static PALACE_OF_TWILIGHT: [Room; 10] = [
    Room::new("Entrance", 0, &[0, 1, 2, 3, 10, 20, 21, 22, 4]),
    Room::new("Left Wing 1", 1, &[0, 1]),
    Room::new("Left Wing 2", 2, &[0, 1]),
    Room::new("Right Wing 1", 4, &[0, 1]),
    Room::new("Right Wing 2", 5, &[0, 1, 2]),
    Room::new("Double Sol", 7, &[0, 1]),
    Room::new("Early Platform", 8, &[0]),
    Room::new("Messengers before Zant", 9, &[0, 1, 20, 21]),
    Room::new("Beta Zant Room", 10, &[0, 1]),
    Room::new("Boss Key", 11, &[0, 1, 20, 22, 21]),
];
pub static ZANT_MAIN_ROOM: [Room; 1] = [Room::new("Zant Main Room", 10, &[0, 1, 21, 23, 24, 25])];
pub static PHANTOM_ZANT_1: [Room; 1] = [Room::new("Phantom Zant 1", 51, &[0, 1, 2, 3])];
pub static PHANTOM_ZANT_2: [Room; 1] = [Room::new("Phantom Zant 2", 52, &[0, 2])];
pub static ZANT_FIGHT: [Room; 7] = [
    Room::new("Intro", 50, &[20, 0]),
    Room::new("Diababa Phase", 53, &[0]),
    Room::new("Goron Mines Phase", 54, &[0]),
    Room::new("Lakebed Phase", 55, &[0, 1]),
    Room::new("Ook Phase", 56, &[0]),
    Room::new("Blizzeta Phase", 57, &[0]),
    Room::new("Final Hyrule Phase", 60, &[0]),
];
pub static HYRULE_CASTLE: [Room; 13] = [
    Room::new("Inside Main Hall", 1, &[0, 1, 1, 2, 3]),
    Room::new("Inside Darknut 1", 2, &[0, 2, 3]),
    Room::new("Inside Left Wing 1", 3, &[0]),
    Room::new("Inside Left Wing 2", 4, &[1, 0, 2]),
    Room::new("Inside Right Wing 1", 5, &[0]),
    Room::new("Inside Right Wing 2", 6, &[0, 1]),
    Room::new("Treasure Room", 8, &[0]),
    Room::new("Graveyard", 9, &[0, 1, 2]),
    Room::new("Entrance", 11, &[1, 2, 3, 0, 5]),
    Room::new("Inside Final Ascension", 12, &[0, 1, 2, 3, 4, 5, 6, 7, 8]),
    Room::new("Outside Left Wing", 13, &[0]),
    Room::new("Outside Right Wing", 14, &[1, 2, 3, 0, 4, 5]),
    Room::new("Outside Boss Key", 15, &[1, 0, 3, 2, 4, 5, 6, 7]),
];
pub static GANONDORF_CASTLE: [Room; 2] = [
    Room::new(
        "Fight Inside",
        50,
        &[0, 1, 2, 20, 21, 22, 120, 121, 122, 10],
    ),
    Room::new(
        "Intro Outside",
        51,
        &[0, 1, 2, 20, 21, 22, 120, 121, 122, 10],
    ),
];
pub static GANONDORF_FIELD: [Room; 1] = [Room::new("Ganondorf Field", 0, &[0, 1])];
pub static GANONDORF_DEFEATED: [Room; 1] =
    [Room::new("Ganondorf Defeated", 0, &[0, 20, 21, 23, 22])];
pub static ARBITERS_GROUNDS: [Room; 17] = [
    Room::new("Entrance", 0, &[2, 1, 0, 3]),
    Room::new("Before Main Room", 1, &[0]),
    Room::new("Main Poe Room", 2, &[0, 2, 1, 3]),
    Room::new("2nd Poe", 3, &[0]),
    Room::new("Before 3rd Poe", 4, &[0, 1, 2, 3]),
    Room::new("3rd Poe", 5, &[0]),
    Room::new("Left Wing 1", 6, &[0, 1]),
    Room::new("Left Wing 2", 7, &[0]),
    Room::new("4th Poe", 8, &[0]),
    Room::new("Boss Key", 9, &[0, 1, 2]),
    Room::new("Ooccoo", 10, &[0]),
    Room::new("Before Death Sword", 11, &[0, 1, 2, 3]),
    Room::new("Before 4th Poe", 12, &[0]),
    Room::new("Epic Spinner Room", 13, &[0, 1]),
    Room::new("After 3rd Poe", 14, &[0]),
    Room::new("Right Wing", 15, &[0, 1]),
    Room::new("Big Turning Room", 16, &[0]),
];
pub static STALLORD: [Room; 1] = [Room::new("Stallord", 50, &[0, 1, 2, 3])];
pub static DEATH_SWORD: [Room; 1] = [Room::new("Death Sword", 51, &[0, 1, 2])];
pub static SNOWPEAK_RUINS: [Room; 12] = [
    Room::new("Entrance", 0, &[0, 1, 2, 3]),
    Room::new("Yeta", 1, &[0]),
    Room::new("Yeto", 2, &[0, 1, 2]),
    Room::new("Ice Puzzle", 3, &[0, 1, 4, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12]),
    Room::new("Courtyard", 4, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
    Room::new("Ordon Pumpkin", 5, &[0, 1, 2, 3, 4]),
    Room::new("Third Poe", 6, &[0, 1, 2]),
    Room::new("Double Freezard", 7, &[0, 10]),
    Room::new("Double LJA", 8, &[0]),
    Room::new("Ice Cannon Room", 9, &[0]),
    Room::new("Boss Key", 11, &[0]),
    Room::new("Before Ordon Pumpkin", 13, &[0]),
];
pub static BLIZZETA: [Room; 1] = [Room::new("Blizzeta", 50, &[0, 1, 2, 3])];
pub static DARKHAMMER: [Room; 2] = [
    Room::new("Beta Room", 49, &[0, 1, 2]),
    Room::new("Darkhammer", 51, &[0, 1, 2, 3]),
];
