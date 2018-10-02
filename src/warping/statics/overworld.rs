use super::*;
#[derive(PartialEq, Clone)]
pub enum OverworldStage {
    OrdonRanch,
    TitleScreen,
    OrdonVillage,
    OrdonSpring,
    FaronWoods,
    KakarikoVillage,
    DeathMountain,
    KakarikoGraveyard,
    ZorasRiver,
    ZorasDomain,
    Snowpeak,
    LakeHylia,
    CastleTown,
    SacredGrove,
    BublinCamp,
    HyruleField,
    OutsideCastleTown,
    Bublin2,
    GerudoDesert,
    MirrorChamber,
    UpperZorasRiver,
    FishingPond,
    HiddenVillage,
    HiddenSkill,
}

impl OverworldStage {
    pub fn get_id(&self) -> &'static str {
        match *self {
            OverworldStage::OrdonRanch => "F_SP00",
            OverworldStage::TitleScreen => "F_SP102",
            OverworldStage::OrdonVillage => "F_SP103",
            OverworldStage::OrdonSpring => "F_SP104",
            OverworldStage::FaronWoods => "F_SP108",
            OverworldStage::KakarikoVillage => "F_SP109",
            OverworldStage::DeathMountain => "F_SP110",
            OverworldStage::KakarikoGraveyard => "F_SP111",
            OverworldStage::ZorasRiver => "F_SP112",
            OverworldStage::ZorasDomain => "F_SP113",
            OverworldStage::Snowpeak => "F_SP114",
            OverworldStage::LakeHylia => "F_SP115",
            OverworldStage::CastleTown => "F_SP116",
            OverworldStage::SacredGrove => "F_SP117",
            OverworldStage::BublinCamp => "F_SP118",
            OverworldStage::HyruleField => "F_SP121",
            OverworldStage::OutsideCastleTown => "F_SP122",
            OverworldStage::Bublin2 => "F_SP123",
            OverworldStage::GerudoDesert => "F_SP124",
            OverworldStage::MirrorChamber => "F_SP125",
            OverworldStage::UpperZorasRiver => "F_SP126",
            OverworldStage::FishingPond => "F_SP127",
            OverworldStage::HiddenVillage => "F_SP128",
            OverworldStage::HiddenSkill => "F_SP200",
        }
    }

    pub fn get_rooms(&self) -> &'static [Room<'static>] {
        match *self {
            OverworldStage::OrdonRanch => &ORDON_RANCH,
            OverworldStage::TitleScreen => &TITLE_SCREEN,
            OverworldStage::OrdonVillage => &ORDON_VILLAGE,
            OverworldStage::OrdonSpring => &ORDON_SPRING,
            OverworldStage::FaronWoods => &FARON_WOODS,
            OverworldStage::KakarikoVillage => &KAKARIKO_VILLAGE,
            OverworldStage::DeathMountain => &DEATH_MOUNTAIN,
            OverworldStage::KakarikoGraveyard => &KAKARIKO_GRAVEYARD,
            OverworldStage::ZorasRiver => &ZORAS_RIVER,
            OverworldStage::ZorasDomain => &ZORAS_DOMAIN,
            OverworldStage::Snowpeak => &SNOWPEAK,
            OverworldStage::LakeHylia => &LAKE_HYLIA,
            OverworldStage::CastleTown => &CASTLE_TOWN,
            OverworldStage::SacredGrove => &SACRED_GROVE,
            OverworldStage::BublinCamp => &BUBLIN_CAMP,
            OverworldStage::HyruleField => &HYRULE_FIELD,
            OverworldStage::OutsideCastleTown => &OUTSIDE_CASTLE_TOWN,
            OverworldStage::Bublin2 => &BUBLIN_2,
            OverworldStage::GerudoDesert => &GERUDO_DESERT,
            OverworldStage::MirrorChamber => &MIRROR_CHAMBER,
            OverworldStage::UpperZorasRiver => &UPPER_ZORAS_RIVER,
            OverworldStage::FishingPond => &FISHING_POND,
            OverworldStage::HiddenVillage => &HIDDEN_VILLAGE,
            OverworldStage::HiddenSkill => &HIDDEN_SKILL,
        }
    }

    pub fn len(&self) -> usize {
        24
    }
}

impl From<usize> for OverworldStage {
    fn from(i: usize) -> Self {
        match i {
            0 => OverworldStage::OrdonRanch,
            1 => OverworldStage::TitleScreen,
            2 => OverworldStage::OrdonVillage,
            3 => OverworldStage::OrdonSpring,
            4 => OverworldStage::FaronWoods,
            5 => OverworldStage::KakarikoVillage,
            6 => OverworldStage::DeathMountain,
            7 => OverworldStage::KakarikoGraveyard,
            8 => OverworldStage::ZorasRiver,
            9 => OverworldStage::ZorasDomain,
            10 => OverworldStage::Snowpeak,
            11 => OverworldStage::LakeHylia,
            12 => OverworldStage::CastleTown,
            13 => OverworldStage::SacredGrove,
            14 => OverworldStage::BublinCamp,
            15 => OverworldStage::HyruleField,
            16 => OverworldStage::OutsideCastleTown,
            17 => OverworldStage::Bublin2,
            18 => OverworldStage::GerudoDesert,
            19 => OverworldStage::MirrorChamber,
            20 => OverworldStage::UpperZorasRiver,
            21 => OverworldStage::FishingPond,
            22 => OverworldStage::HiddenVillage,
            23 => OverworldStage::HiddenSkill,
            _ => unreachable!(),
        }
    }
}

pub static STAGES: [&str; 24] = [
    "Ordon Ranch",
    "Title Screen",
    "Ordon Village",
    "Ordon Spring",
    "Faron Woods",
    "Kakariko Village",
    "Death Mountain",
    "Kakariko Graveyard",
    "Zoras River",
    "Zoras Domain",
    "Snowpeak",
    "Lake Hylia",
    "Castle Town",
    "Sacred Grove",
    "Bublin Camp",
    "Hyrule Field",
    "Outside Castle Town",
    "Bublin 2",
    "Gerudo Desert",
    "Mirror Chamber",
    "Upper Zoras River",
    "Fishing Pond",
    "Hidden Village",
    "Hidden Skill",
];

pub static ORDON_RANCH: [Room; 1] = [Room::new(
    "Ordon Ranch",
    0,
    &[0, 1, 2, 3, 20, 4, 99, 30, 5, 6, 7, 127],
)];
pub static TITLE_SCREEN: [Room; 1] = [Room::new(
    "Title Screen",
    0,
    &[0, 1, 2, 20, 3, 53, 4, 5, 100, 101],
)];
pub static ORDON_VILLAGE: [Room; 2] = [
    Room::new(
        "Main Village",
        0,
        &[
            0, 1, 2, 4, 5, 6, 7, 9, 11, 20, 21, 22, 23, 24, 25, 13, 14, 15, 26, 99, 27, 30, 100,
            101, 102, 103,
        ],
    ),
    Room::new(
        "Outside Links House",
        1,
        &[
            0, 2, 3, 20, 21, 6, 23, 7, 24, 25, 26, 99, 8, 27, 30, 1, 9, 10, 5, 100, 4, 11,
        ],
    ),
];
pub static ORDON_SPRING: [Room; 1] = [Room::new(
    "Ordon Spring",
    1,
    &[
        0, 1, 2, 20, 21, 3, 22, 26, 5, 23, 24, 25, 99, 254, 200, 6, 10, 111, 4, 30, 100,
    ],
)];
pub static FARON_WOODS: [Room; 10] = [
    Room::new(
        "South Faron Entrance",
        0,
        &[0, 3, 4, 20, 21, 22, 23, 24, 254, 25, 100],
    ),
    Room::new("Faron Spring", 1, &[6, 3, 2, 20, 21, 0, 1, 100]),
    Room::new("Transition Spring-Coro", 2, &[0]),
    Room::new("Gate before Coro", 3, &[0, 99, 5]),
    Room::new("Coro Area", 4, &[7, 9, 8, 0, 1, 23, 2, 100]),
    Room::new(
        "Mist Area",
        5,
        &[4, 10, 0, 1, 2, 3, 24, 25, 6, 98, 7, 50, 60, 100],
    ),
    Room::new("North Faron", 6, &[1, 0, 254, 2, 3, 50, 100, 150, 10, 200]),
    Room::new("Coro Shortcut to Mist", 8, &[0, 1, 2, 3]),
    Room::new("Transition Mist-North Faron", 11, &[0]),
    Room::new("Small Key Cave", 14, &[0]),
];
pub static KAKARIKO_VILLAGE: [Room; 1] = [Room::new(
    "Kakariko Village",
    0,
    &[
        0, 1, 2, 3, 20, 4, 14, 6, 8, 7, 9, 10, 11, 21, 22, 23, 12, 13, 30, 32, 33, 34, 35, 31, 5,
        40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 60, 36, 37, 38, 39, 52, 53, 15, 54, 55, 56,
        57, 58, 59, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 24, 16, 17, 18, 19, 100, 71, 101, 25,
    ],
)];
pub static DEATH_MOUNTAIN: [Room; 4] = [
    Room::new("Entrance", 0, &[0, 200, 100, 2, 1]),
    Room::new("Entrance-Mountain Transition 1", 1, &[0]),
    Room::new("Entrance-Mountain Transition 2", 2, &[0]),
    Room::new("Mountain", 3, &[0, 1, 2, 3, 4, 5, 6]),
];
pub static KAKARIKO_GRAVEYARD: [Room; 1] = [Room::new(
    "Kakariko Graveyard",
    0,
    &[0, 1, 2, 3, 4, 5, 6, 111],
)];
pub static ZORAS_RIVER: [Room; 1] = [Room::new(
    "Zoras River",
    1,
    &[1, 11, 2, 15, 4, 5, 10, 6, 0, 12, 13, 14, 16, 17, 3, 7],
)];
pub static ZORAS_DOMAIN: [Room; 2] = [
    Room::new("Inside", 0, &[0, 1, 3, 4, 5, 254, 50, 7, 97, 8, 99, 10]),
    Room::new(
        "Outside",
        1,
        &[
            9, 5, 1, 6, 7, 3, 0, 4, 100, 8, 10, 11, 98, 12, 13, 14, 20, 101, 15, 30, 34,
        ],
    ),
];
pub static SNOWPEAK: [Room; 3] = [
    Room::new("First Half", 0, &[0, 4, 7, 15, 14, 13, 100, 5, 1, 2, 6, 10]),
    Room::new(
        "Second Half",
        1,
        &[1, 2, 3, 5, 6, 9, 10, 11, 20, 12, 21, 13, 22, 100],
    ),
    Room::new("Transition Cave", 2, &[8, 12, 13]),
];
pub static LAKE_HYLIA: [Room; 2] = [
    Room::new(
        "Lake",
        0,
        &[
            0, 1, 2, 3, 5, 7, 254, 10, 11, 6, 12, 13, 14, 8, 15, 100, 50, 33, 9, 16, 20, 70, 25,
            40, 101, 150, 55, 77, 78, 29, 30, 31, 32, 133, 4, 134, 17, 200, 34, 99, 76, 75,
        ],
    ),
    Room::new("Fountain", 1, &[1, 0, 20, 21, 22, 23, 100]),
];
pub static CASTLE_TOWN: [Room; 5] = [
    Room::new(
        "Central",
        0,
        &[99, 11, 3, 4, 5, 6, 20, 0, 12, 13, 14, 15, 16, 100, 50],
    ),
    Room::new("North", 1, &[0, 1, 30, 111, 100, 50, 40]),
    Room::new("West", 2, &[2, 0, 1, 3, 4]),
    Room::new("South", 3, &[2, 0, 3, 1, 4, 5, 7, 8, 9, 30, 10, 12, 13]),
    Room::new("East", 4, &[2, 0, 3, 4, 5, 6]),
];
pub static SACRED_GROVE: [Room; 3] = [
    Room::new(
        "Master Sword",
        1,
        &[
            1, 254, 5, 6, 20, 21, 3, 99, 10, 150, 100, 102, 4, 50, 51, 200,
        ],
    ),
    Room::new("Temple of Time", 2, &[0, 1, 101, 3, 102, 52]),
    Room::new("Lost Woods", 3, &[0, 1, 2, 3, 4, 5, 6]),
];
pub static BUBLIN_CAMP: [Room; 3] = [
    Room::new("Main Camp", 1, &[0, 1, 2, 6]),
    Room::new("Beta Camp", 2, &[0]),
    Room::new("Before Arbiters Grounds", 3, &[0, 2, 3, 4, 5, 7]),
];
pub static HYRULE_FIELD: [Room; 15] = [
    Room::new(
        "Eldin Main Field",
        0,
        &[
            0, 1, 2, 3, 4, 5, 50, 6, 7, 8, 9, 10, 11, 12, 13, 20, 21, 14, 15, 16, 17, 18,
        ],
    ),
    Room::new("Faron-Eldin Transition ", 1, &[0, 20, 21, 22]),
    Room::new("Eldin Entrance", 2, &[0, 10, 20, 1, 30]),
    Room::new(
        "Eldin Gorge",
        3,
        &[1, 99, 2, 10, 20, 3, 0, 21, 5, 88, 15, 4, 14, 6, 22, 16, 17],
    ),
    Room::new("Eldin Gorge-Main Transition 1", 4, &[0, 1]),
    Room::new("Eldin Gorge-Main Transition 2", 5, &[0]),
    Room::new(
        "Faron Main Field",
        6,
        &[0, 1, 10, 21, 11, 2, 3, 100, 101, 12],
    ),
    Room::new("Eldin-Lanayru Transition", 7, &[0, 22, 6, 1, 2, 14]),
    Room::new("Lanayru Entrance", 9, &[0, 1, 10, 2]),
    Room::new(
        "Lanayru Main Field",
        10,
        &[0, 1, 2, 3, 4, 20, 21, 22, 5, 23, 15, 6, 7, 8, 14, 16],
    ),
    Room::new("Lanayru Main-Bridge Transition 1", 11, &[0, 1]),
    Room::new(
        "Lanayru Main-Bridge Transition 2",
        12,
        &[0, 1, 2, 21, 20, 3],
    ),
    Room::new(
        "Lanayru Lake Hylia Bridge",
        13,
        &[0, 1, 99, 2, 20, 98, 21, 22, 23, 3, 4, 14],
    ),
    Room::new("Faron-Lanayru Transition", 14, &[0, 1]),
    Room::new("Faron-Lanayru Gate", 15, &[0, 99, 20, 21, 22]),
];
pub static OUTSIDE_CASTLE_TOWN: [Room; 3] = [
    Room::new(
        "West Field",
        8,
        &[0, 1, 2, 254, 3, 4, 100, 101, 111, 5, 6, 7, 200, 76],
    ),
    Room::new("South Field", 16, &[0, 1, 2, 111, 3, 4]),
    Room::new("East Field", 17, &[0, 1, 4]),
];
pub static BUBLIN_2: [Room; 1] = [Room::new("Bublin 2", 13, &[0])];
pub static GERUDO_DESERT: [Room; 1] = [Room::new(
    "Gerudo Desert",
    0,
    &[
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 50, 51, 52, 53, 111,
    ],
)];
pub static MIRROR_CHAMBER: [Room; 1] = [Room::new(
    "Mirror Chamber",
    4,
    &[0, 1, 51, 52, 54, 55, 2, 3, 4, 56, 57, 5, 58, 7, 6, 8],
)];
pub static UPPER_ZORAS_RIVER: [Room; 1] = [Room::new(
    "Upper Zoras River",
    0,
    &[99, 1, 2, 3, 100, 5, 6, 7, 8, 9, 0, 4, 101, 10, 200, 11],
)];
pub static FISHING_POND: [Room; 1] = [Room::new("Fishing Pond", 0, &[0, 1, 2, 3, 4, 5, 100])];
pub static HIDDEN_VILLAGE: [Room; 1] = [Room::new("Hidden Village", 0, &[0, 1, 2, 100, 3, 4, 5])];
pub static HIDDEN_SKILL: [Room; 1] = [Room::new("Hidden Skill", 0, &[2, 1, 3, 4, 5, 6, 7, 0])];
