#![allow(dead_code)]
use super::*;

#[derive(PartialEq, Clone)]
pub enum InteriorStage {
    OrdonVillage,
    HyruleCastleSewers,
    FaronWoods,
    KakarikoVillage,
    DeathMountain,
    TelmasBar,
    FishingPond,
    HiddenVillage,
    CastleTown,
    StarGame,
    KakarikoGraveyard,
    LightArrowsCutscene,
    HyruleCastleCutscenes,
}

impl Stage for InteriorStage {
    fn get_id(&self) -> &'static str {
        match *self {
            InteriorStage::OrdonVillage => "R_SP01",
            InteriorStage::HyruleCastleSewers => "R_SP107",
            InteriorStage::FaronWoods => "R_SP108",
            InteriorStage::KakarikoVillage => "R_SP109",
            InteriorStage::DeathMountain => "R_SP110",
            InteriorStage::TelmasBar => "R_SP116",
            InteriorStage::FishingPond => "R_SP127",
            InteriorStage::HiddenVillage => "R_SP128",
            InteriorStage::CastleTown => "R_SP160",
            InteriorStage::StarGame => "R_SP161",
            InteriorStage::KakarikoGraveyard => "R_SP209",
            InteriorStage::LightArrowsCutscene => "R_SP300",
            InteriorStage::HyruleCastleCutscenes => "R_SP301",
        }
    }

    fn get_rooms(&self) -> &'static [Room<'static>] {
        match *self {
            InteriorStage::OrdonVillage => &ORDON_VILLAGE,
            InteriorStage::HyruleCastleSewers => &HYRULE_CASTLE_SEWERS,
            InteriorStage::FaronWoods => &FARON_WOODS,
            InteriorStage::KakarikoVillage => &KAKARIKO_VILLAGE,
            InteriorStage::DeathMountain => &DEATH_MOUNTAIN,
            InteriorStage::TelmasBar => &TELMAS_BAR,
            InteriorStage::FishingPond => &FISHING_POND,
            InteriorStage::HiddenVillage => &HIDDEN_VILLAGE,
            InteriorStage::CastleTown => &CASTLE_TOWN,
            InteriorStage::StarGame => &STAR_GAME,
            InteriorStage::KakarikoGraveyard => &KAKARIKO_GRAVEYARD,
            InteriorStage::LightArrowsCutscene => &LIGHT_ARROWS_CUTSCENE,
            InteriorStage::HyruleCastleCutscenes => &HYRULE_CASTLE_CUTSCENES,
        }
    }
}

pub static STAGES: [(&Stage, &str); 13] = [
    (&InteriorStage::OrdonVillage, "Ordon Village"),
    (&InteriorStage::HyruleCastleSewers, "Hyrule Castle Sewers"),
    (&InteriorStage::FaronWoods, "Faron Woods"),
    (&InteriorStage::KakarikoVillage, "Kakariko Village"),
    (&InteriorStage::DeathMountain, "Death Mountain"),
    (&InteriorStage::TelmasBar, "Telmas Bar"),
    (&InteriorStage::FishingPond, "Fishing Pond"),
    (&InteriorStage::HiddenVillage, "Hidden Village"),
    (&InteriorStage::CastleTown, "Castle Town"),
    (&InteriorStage::StarGame, "Star Game"),
    (&InteriorStage::KakarikoGraveyard, "Kakariko Graveyard"),
    (&InteriorStage::LightArrowsCutscene, "Light Arrows Cutscene"),
    (
        &InteriorStage::HyruleCastleCutscenes,
        "Hyrule Castle Cutscenes",
    ),
];

pub static ORDON_VILLAGE: [Room; 6] = [
    Room::new("Bo House", 0, &[0, 1, 2, 3]),
    Room::new("Shop House", 1, &[0]),
    Room::new("Shield House", 2, &[0, 1, 2, 3]),
    Room::new("Links House", 4, &[0, 1, 2, 3, 4]),
    Room::new("Sword House", 5, &[1, 0, 2]),
    Room::new("Links House Storage", 7, &[0]),
];
pub static HYRULE_CASTLE_SEWERS: [Room; 4] = [
    Room::new("Prison", 0, &[0, 1, 2, 21, 22, 23, 24, 3, 25]),
    Room::new("Sewers", 1, &[0, 1, 2, 3, 4, 5, 6, 7]),
    Room::new("Rooftops", 2, &[0, 1, 20, 2, 2]),
    Room::new("Zeldas Tower", 3, &[0, 20, 21, 1, 22, 24, 23]),
];
pub static FARON_WOODS: [Room; 1] = [Room::new("Coros House", 0, &[0, 1])];
pub static KAKARIKO_VILLAGE: [Room; 7] = [
    Room::new(
        "Renardos Sanctuary",
        0,
        &[0, 20, 3, 5, 6, 7, 8, 2, 22, 21, 10],
    ),
    Room::new("Barnes Shop", 1, &[0, 1, 2, 3]),
    Room::new("Hotel", 2, &[0, 1, 2, 3]),
    Room::new("Malos Shop", 3, &[0, 1]),
    Room::new("Top House", 4, &[0, 1, 2]),
    Room::new("Bomb House", 5, &[0, 1]),
    Room::new("Bug House", 6, &[0, 5, 1]),
];
pub static DEATH_MOUNTAIN: [Room; 1] = [Room::new("Goron Elder Cave", 0, &[0, 1, 2, 3, 4, 100])];
pub static TELMAS_BAR: [Room; 2] = [
    Room::new("Telmas Bar", 5, &[0, 1, 2, 3, 20, 4, 30, 5, 6]),
    Room::new("Jovani-Sewers Transition", 6, &[11, 10, 20, 21, 12]),
];
pub static FISHING_POND: [Room; 1] = [Room::new("Henas House", 0, &[0, 1])];
pub static HIDDEN_VILLAGE: [Room; 1] = [Room::new("Impaz House", 0, &[0])];
pub static CASTLE_TOWN: [Room; 6] = [
    Room::new("Malos Shop", 0, &[0, 1, 2]),
    Room::new("Fortune Teller House", 1, &[0, 1, 2]),
    Room::new("Doctor House", 2, &[0, 1, 2]),
    Room::new("Agitha House", 3, &[1, 2, 0]),
    Room::new("Goron Shops", 4, &[0, 1, 2]),
    Room::new("Jovani House", 5, &[0, 1, 2, 3, 4]),
];
pub static STAR_GAME: [Room; 1] = [Room::new("Star Game", 7, &[0, 1, 2, 3, 4])];
pub static KAKARIKO_GRAVEYARD: [Room; 1] =
    [Room::new("Sanctuary Cave (Sky Cannon)", 7, &[1, 0, 2])];
pub static LIGHT_ARROWS_CUTSCENE: [Room; 1] =
    [Room::new("Light Arrows Cutscene", 0, &[0, 20, 120])];
pub static HYRULE_CASTLE_CUTSCENES: [Room; 1] =
    [Room::new("Hyrule Castle Cutscenes", 0, &[0, 20, 100])];
