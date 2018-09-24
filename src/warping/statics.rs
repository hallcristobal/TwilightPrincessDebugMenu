#![allow(dead_code)]

pub struct Room<'a>(pub &'a str, pub u8, pub &'a [u8]);

#[derive(PartialEq, Clone)]
pub enum Stage {
	Deleted,
	HyruleCastleCutscenes,
	LightArrowsCutscene,
	KakarikoGraveyard,
	StarGame,
	CastleTown,
	HiddenVillage,
	FishingPond,
	TelmasBar,
	DeathMountain,
	KakarikoVillage,
	FaronWoods,
	HyruleCastleSewers,
	OrdonVillage,
	HiddenSkill,
	UpperZorasRiver,
	MirrorChamber,
	GerudoDesert,
	Bublin2,
	OutsideCastleTown,
	HyruleField,
	BublinCamp,
	SacredGrove,
	LakeHylia,
	Snowpeak,
	ZorasDomain,
	ZorasRiver,
	OrdonSpring,
	TitleScreen,
	OrdonRanch,
	FaronWoodsCave,
	Grotto5,
	Grotto4,
	Grotto3,
	Grotto2,
	Grotto1,
	EldinGoronStockcave,
	LakeHyliaLongCave,
	EldinLongCave,
	CaveOfOrdeals,
	LanayruIcePuzzleCave,
	Darkhammer,
	Blizzeta,
	SnowpeakRuins,
	DeathSword,
	Stallord,
	ArbitersGrounds,
	GanondorfDefeated,
	GanondorfField,
	GanondorfCastle,
	HyruleCastle,
	ZantFight,
	PhantomZant2,
	PhantomZant1,
	ZantMainRoom,
	PalaceOfTwilight,
	Aeralfos,
	Argorok,
	CityInTheSky,
	Darknut,
	Armogohma,
	TempleOfTime,
	Ook,
	Diababa,
	ForestTemple,
	Dangoro,
	Fyrus,
	GoronMines,
	DekuToad,
	Morpheel,
	LakebedTemple,
}

impl Stage {
	pub fn get_id(self) -> &'static str {
		match self {
			Stage::Deleted => "S_MV000",
			Stage::HyruleCastleCutscenes => "R_SP301",
			Stage::LightArrowsCutscene => "R_SP300",
			Stage::KakarikoGraveyard => "R_SP209",
			Stage::StarGame => "R_SP161",
			Stage::CastleTown => "R_SP160",
			Stage::HiddenVillage => "R_SP128",
			Stage::FishingPond => "R_SP127",
			Stage::TelmasBar => "R_SP116",
			Stage::DeathMountain => "R_SP110",
			Stage::KakarikoVillage => "R_SP109",
			Stage::FaronWoods => "R_SP108",
			Stage::HyruleCastleSewers => "R_SP107",
			Stage::OrdonVillage => "R_SP01",
			Stage::HiddenSkill => "F_SP200",
			Stage::UpperZorasRiver => "F_SP126",
			Stage::MirrorChamber => "F_SP125",
			Stage::GerudoDesert => "F_SP124",
			Stage::Bublin2 => "F_SP123",
			Stage::OutsideCastleTown => "F_SP122",
			Stage::HyruleField => "F_SP121",
			Stage::BublinCamp => "F_SP118",
			Stage::SacredGrove => "F_SP117",
			Stage::LakeHylia => "F_SP115",
			Stage::Snowpeak => "F_SP114",
			Stage::ZorasDomain => "F_SP113",
			Stage::ZorasRiver => "F_SP112",
			Stage::OrdonSpring => "F_SP104",
			Stage::TitleScreen => "F_SP102",
			Stage::OrdonRanch => "F_SP00",
			Stage::FaronWoodsCave => "D_SB10",
			Stage::Grotto5 => "D_SB09",
			Stage::Grotto4 => "D_SB08",
			Stage::Grotto3 => "D_SB07",
			Stage::Grotto2 => "D_SB06",
			Stage::Grotto1 => "D_SB05",
			Stage::EldinGoronStockcave => "D_SB04",
			Stage::LakeHyliaLongCave => "D_SB03",
			Stage::EldinLongCave => "D_SB02",
			Stage::CaveOfOrdeals => "D_SB01",
			Stage::LanayruIcePuzzleCave => "D_SB00",
			Stage::Darkhammer => "D_MN11B",
			Stage::Blizzeta => "D_MN11A",
			Stage::SnowpeakRuins => "D_MN11",
			Stage::DeathSword => "D_MN10B",
			Stage::Stallord => "D_MN10A",
			Stage::ArbitersGrounds => "D_MN10",
			Stage::GanondorfDefeated => "D_MN09C",
			Stage::GanondorfField => "D_MN09B",
			Stage::GanondorfCastle => "D_MN09A",
			Stage::HyruleCastle => "D_MN09",
			Stage::ZantFight => "D_MN08D",
			Stage::PhantomZant2 => "D_MN08C",
			Stage::PhantomZant1 => "D_MN08B",
			Stage::ZantMainRoom => "D_MN08A",
			Stage::PalaceOfTwilight => "D_MN08",
			Stage::Aeralfos => "D_MN07B",
			Stage::Argorok => "D_MN07A",
			Stage::CityInTheSky => "D_MN07",
			Stage::Darknut => "D_MN06B",
			Stage::Armogohma => "D_MN06A",
			Stage::TempleOfTime => "D_MN06",
			Stage::Ook => "D_MN05B",
			Stage::Diababa => "D_MN05A",
			Stage::ForestTemple => "D_MN05",
			Stage::Dangoro => "D_MN04B",
			Stage::Fyrus => "D_MN04A",
			Stage::GoronMines => "D_MN04",
			Stage::DekuToad => "D_MN01B",
			Stage::Morpheel => "D_MN01A",
			Stage::LakebedTemple => "D_MN01",
		}
	}

	pub fn get_rooms(self) -> &'static [Room<'static>] {
        match self {
			Stage::Deleted => &DELETED,
			Stage::HyruleCastleCutscenes => &HYRULE_CASTLE_CUTSCENES,
			Stage::LightArrowsCutscene => &LIGHT_ARROWS_CUTSCENE,
			Stage::KakarikoGraveyard => &KAKARIKO_GRAVEYARD,
			Stage::StarGame => &STAR_GAME,
			Stage::CastleTown => &CASTLE_TOWN,
			Stage::HiddenVillage => &HIDDEN_VILLAGE,
			Stage::FishingPond => &FISHING_POND,
			Stage::TelmasBar => &TELMAS_BAR,
			Stage::DeathMountain => &DEATH_MOUNTAIN,
			Stage::KakarikoVillage => &KAKARIKO_VILLAGE,
			Stage::FaronWoods => &FARON_WOODS,
			Stage::HyruleCastleSewers => &HYRULE_CASTLE_SEWERS,
			Stage::OrdonVillage => &ORDON_VILLAGE,
			Stage::HiddenSkill => &HIDDEN_SKILL,
			Stage::UpperZorasRiver => &UPPER_ZORAS_RIVER,
			Stage::MirrorChamber => &MIRROR_CHAMBER,
			Stage::GerudoDesert => &GERUDO_DESERT,
			Stage::Bublin2 => &BUBLIN_2,
			Stage::OutsideCastleTown => &OUTSIDE_CASTLE_TOWN,
			Stage::HyruleField => &HYRULE_FIELD,
			Stage::BublinCamp => &BUBLIN_CAMP,
			Stage::SacredGrove => &SACRED_GROVE,
			Stage::LakeHylia => &LAKE_HYLIA,
			Stage::Snowpeak => &SNOWPEAK,
			Stage::ZorasDomain => &ZORAS_DOMAIN,
			Stage::ZorasRiver => &ZORAS_RIVER,
			Stage::OrdonSpring => &ORDON_SPRING,
			Stage::TitleScreen => &TITLE_SCREEN,
			Stage::OrdonRanch => &ORDON_RANCH,
			Stage::FaronWoodsCave => &FARON_WOODS_CAVE,
			Stage::Grotto5 => &GROTTO_5,
			Stage::Grotto4 => &GROTTO_4,
			Stage::Grotto3 => &GROTTO_3,
			Stage::Grotto2 => &GROTTO_2,
			Stage::Grotto1 => &GROTTO_1,
			Stage::EldinGoronStockcave => &ELDIN_GORON_STOCKCAVE,
			Stage::LakeHyliaLongCave => &LAKE_HYLIA_LONG_CAVE,
			Stage::EldinLongCave => &ELDIN_LONG_CAVE,
			Stage::CaveOfOrdeals => &CAVE_OF_ORDEALS,
			Stage::LanayruIcePuzzleCave => &LANAYRU_ICE_PUZZLE_CAVE,
			Stage::Darkhammer => &DARKHAMMER,
			Stage::Blizzeta => &BLIZZETA,
			Stage::SnowpeakRuins => &SNOWPEAK_RUINS,
			Stage::DeathSword => &DEATH_SWORD,
			Stage::Stallord => &STALLORD,
			Stage::ArbitersGrounds => &ARBITERS_GROUNDS,
			Stage::GanondorfDefeated => &GANONDORF_DEFEATED,
			Stage::GanondorfField => &GANONDORF_FIELD,
			Stage::GanondorfCastle => &GANONDORF_CASTLE,
			Stage::HyruleCastle => &HYRULE_CASTLE,
			Stage::ZantFight => &ZANT_FIGHT,
			Stage::PhantomZant2 => &PHANTOM_ZANT_2,
			Stage::PhantomZant1 => &PHANTOM_ZANT_1,
			Stage::ZantMainRoom => &ZANT_MAIN_ROOM,
			Stage::PalaceOfTwilight => &PALACE_OF_TWILIGHT,
			Stage::Aeralfos => &AERALFOS,
			Stage::Argorok => &ARGOROK,
			Stage::CityInTheSky => &CITY_IN_THE_SKY,
			Stage::Darknut => &DARKNUT,
			Stage::Armogohma => &ARMOGOHMA,
			Stage::TempleOfTime => &TEMPLE_OF_TIME,
			Stage::Ook => &OOK,
			Stage::Diababa => &DIABABA,
			Stage::ForestTemple => &FOREST_TEMPLE,
			Stage::Dangoro => &DANGORO,
			Stage::Fyrus => &FYRUS,
			Stage::GoronMines => &GORON_MINES,
			Stage::DekuToad => &DEKU_TOAD,
			Stage::Morpheel => &MORPHEEL,
			Stage::LakebedTemple => &LAKEBED_TEMPLE,
		}
	}
}
pub static STAGES: [(Stage, &str); 71] = [
	(Stage::Deleted, "Deleted"),
	(Stage::HyruleCastleCutscenes, "Hyrule Castle Cutscenes"),
	(Stage::LightArrowsCutscene, "Light Arrows Cutscene"),
	(Stage::KakarikoGraveyard, "Kakariko Graveyard"),
	(Stage::StarGame, "Star Game"),
	(Stage::CastleTown, "Castle Town"),
	(Stage::HiddenVillage, "Hidden Village"),
	(Stage::FishingPond, "Fishing Pond"),
	(Stage::TelmasBar, "Telmas Bar"),
	(Stage::DeathMountain, "Death Mountain"),
	(Stage::KakarikoVillage, "Kakariko Village"),
	(Stage::FaronWoods, "Faron Woods"),
	(Stage::HyruleCastleSewers, "Hyrule Castle Sewers"),
	(Stage::OrdonVillage, "Ordon Village"),
	(Stage::HiddenSkill, "Hidden Skill"),
	(Stage::UpperZorasRiver, "Upper Zoras River"),
	(Stage::MirrorChamber, "Mirror Chamber"),
	(Stage::GerudoDesert, "Gerudo Desert"),
	(Stage::Bublin2, "Bublin 2"),
	(Stage::OutsideCastleTown, "Outside Castle Town"),
	(Stage::HyruleField, "Hyrule Field"),
	(Stage::BublinCamp, "Bublin Camp"),
	(Stage::SacredGrove, "Sacred Grove"),
	(Stage::LakeHylia, "Lake Hylia"),
	(Stage::Snowpeak, "Snowpeak"),
	(Stage::ZorasDomain, "Zoras Domain"),
	(Stage::ZorasRiver, "Zoras River"),
	(Stage::OrdonSpring, "Ordon Spring"),
	(Stage::TitleScreen, "Title Screen"),
	(Stage::OrdonRanch, "Ordon Ranch"),
	(Stage::FaronWoodsCave, "Faron Woods Cave"),
	(Stage::Grotto5, "Grotto 5"),
	(Stage::Grotto4, "Grotto 4"),
	(Stage::Grotto3, "Grotto 3"),
	(Stage::Grotto2, "Grotto 2"),
	(Stage::Grotto1, "Grotto 1"),
	(Stage::EldinGoronStockcave, "Eldin Goron Stockcave"),
	(Stage::LakeHyliaLongCave, "Lake Hylia Long Cave"),
	(Stage::EldinLongCave, "Eldin Long Cave"),
	(Stage::CaveOfOrdeals, "Cave of Ordeals"),
	(Stage::LanayruIcePuzzleCave, "Lanayru Ice Puzzle Cave"),
	(Stage::Darkhammer, "Darkhammer"),
	(Stage::Blizzeta, "Blizzeta"),
	(Stage::SnowpeakRuins, "Snowpeak Ruins"),
	(Stage::DeathSword, "Death Sword"),
	(Stage::Stallord, "Stallord"),
	(Stage::ArbitersGrounds, "Arbiters Grounds"),
	(Stage::GanondorfDefeated, "Ganondorf Defeated"),
	(Stage::GanondorfField, "Ganondorf Field"),
	(Stage::GanondorfCastle, "Ganondorf Castle"),
	(Stage::HyruleCastle, "Hyrule Castle"),
	(Stage::ZantFight, "Zant Fight"),
	(Stage::PhantomZant2, "Phantom Zant 2"),
	(Stage::PhantomZant1, "Phantom Zant 1"),
	(Stage::ZantMainRoom, "Zant Main Room"),
	(Stage::PalaceOfTwilight, "Palace of Twilight"),
	(Stage::Aeralfos, "Aeralfos"),
	(Stage::Argorok, "Argorok"),
	(Stage::CityInTheSky, "City in the Sky"),
	(Stage::Darknut, "Darknut"),
	(Stage::Armogohma, "Armogohma"),
	(Stage::TempleOfTime, "Temple of Time"),
	(Stage::Ook, "Ook"),
	(Stage::Diababa, "Diababa"),
	(Stage::ForestTemple, "Forest Temple"),
	(Stage::Dangoro, "Dangoro"),
	(Stage::Fyrus, "Fyrus"),
	(Stage::GoronMines, "Goron Mines"),
	(Stage::DekuToad, "Deku Toad"),
	(Stage::Morpheel, "Morpheel"),
	(Stage::LakebedTemple, "Lakebed Temple"),
];
pub static DELETED: [Room; 1] = [
	Room("Deleted", 0, &[0, 1]),
];
pub static HYRULE_CASTLE_CUTSCENES: [Room; 1] = [
	Room("Hyrule Castle Cutscenes", 0, &[0, 20, 100]),
];
pub static LIGHT_ARROWS_CUTSCENE: [Room; 1] = [
	Room("Light Arrows Cutscene", 0, &[0, 20, 120]),
];
pub static KAKARIKO_GRAVEYARD: [Room; 2] = [
	Room("Sanctuary Cave (Sky Cannon)", 7, &[1, 0, 2]),
	Room("Kakariko Graveyard", 0, &[0, 1, 2, 3, 4, 5, 6, 111]),
];
pub static STAR_GAME: [Room; 1] = [
	Room("Star Game", 7, &[0, 1, 2, 3, 4]),
];
pub static CASTLE_TOWN: [Room; 11] = [
	Room("Malos Shop", 0, &[0, 1, 2]),
	Room("Fortune Teller House", 1, &[0, 1, 2]),
	Room("Doctor House", 2, &[0, 1, 2]),
	Room("Agitha House", 3, &[1, 2, 0]),
	Room("Goron Shops", 4, &[0, 1, 2]),
	Room("Jovani House", 5, &[0, 1, 2, 3, 4]),
	Room("Central", 0, &[99, 11, 3, 4, 5, 6, 20, 0, 12, 13, 14, 15, 16, 100, 50]),
	Room("North", 1, &[0, 1, 30, 111, 100, 50, 40]),
	Room("West", 2, &[2, 0, 1, 3, 4]),
	Room("South", 3, &[2, 0, 3, 1, 4, 5, 7, 8, 9, 30, 10, 12, 13]),
	Room("East", 4, &[2, 0, 3, 4, 5, 6]),
];
pub static HIDDEN_VILLAGE: [Room; 2] = [
	Room("Impaz House", 0, &[0]),
	Room("Hidden Village", 0, &[0, 1, 2, 100, 3, 4, 5]),
];
pub static FISHING_POND: [Room; 2] = [
	Room("Henas House", 0, &[0, 1]),
	Room("Fishing Pond", 0, &[0, 1, 2, 3, 4, 5, 100]),
];
pub static TELMAS_BAR: [Room; 2] = [
	Room("Telmas Bar", 5, &[0, 1, 2, 3, 20, 4, 30, 5, 6]),
	Room("Jovani-Sewers Transition", 6, &[11, 10, 20, 21, 12]),
];
pub static DEATH_MOUNTAIN: [Room; 5] = [
	Room("Goron Elder Cave", 0, &[0, 1, 2, 3, 4, 100]),
	Room("Entrance", 0, &[0, 200, 100, 2, 1]),
	Room("Entrance-Mountain Transition 1", 1, &[0]),
	Room("Entrance-Mountain Transition 2", 2, &[0]),
	Room("Mountain", 3, &[0, 1, 2, 3, 4, 5, 6]),
];
pub static KAKARIKO_VILLAGE: [Room; 8] = [
	Room("Renardos Sanctuary", 0, &[0, 20, 3, 5, 6, 7, 8, 2, 22, 21, 10]),
	Room("Barnes Shop", 1, &[0, 1, 2, 3]),
	Room("Hotel", 2, &[0, 1, 2, 3]),
	Room("Malos Shop", 3, &[0, 1]),
	Room("Top House", 4, &[0, 1, 2]),
	Room("Bomb House", 5, &[0, 1]),
	Room("Bug House", 6, &[0, 5, 1]),
	Room("Kakariko Village", 0, &[0, 1, 2, 3, 20, 4, 14, 6, 8, 7, 9, 10, 11, 21, 22, 23, 12, 13, 30, 32, 33, 34, 35, 31, 5, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 60, 36, 37, 38, 39, 52, 53, 15, 54, 55, 56, 57, 58, 59, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 24, 16, 17, 18, 19, 100, 71, 101, 25]),
];
pub static FARON_WOODS: [Room; 11] = [
	Room("Coros House", 0, &[0, 1]),
	Room("South Faron Entrance", 0, &[0, 3, 4, 20, 21, 22, 23, 24, 254, 25, 100]),
	Room("Faron Spring", 1, &[6, 3, 2, 20, 21, 0, 1, 100]),
	Room("Transition Spring-Coro", 2, &[0]),
	Room("Gate before Coro", 3, &[0, 99, 5]),
	Room("Coro Area", 4, &[7, 9, 8, 0, 1, 23, 2, 100]),
	Room("Mist Area", 5, &[4, 10, 0, 1, 2, 3, 24, 25, 6, 98, 7, 50, 60, 100]),
	Room("North Faron", 6, &[1, 0, 254, 2, 3, 50, 100, 150, 10, 200]),
	Room("Coro Shortcut to Mist", 8, &[0, 1, 2, 3]),
	Room("Transition Mist-North Faron", 11, &[0]),
	Room("Small Key Cave", 14, &[0]),
];
pub static HYRULE_CASTLE_SEWERS: [Room; 4] = [
	Room("Prison", 0, &[0, 1, 2, 21, 22, 23, 24, 3, 25]),
	Room("Sewers", 1, &[0, 1, 2, 3, 4, 5, 6, 7]),
	Room("Rooftops", 2, &[0, 1, 20, 2, 2]),
	Room("Zeldas Tower", 3, &[0, 20, 21, 1, 22, 24, 23]),
];
pub static ORDON_VILLAGE: [Room; 8] = [
	Room("Bo House", 0, &[0, 1, 2, 3]),
	Room("Shop House", 1, &[0]),
	Room("Shield House", 2, &[0, 1, 2, 3]),
	Room("Links House", 4, &[0, 1, 2, 3, 4]),
	Room("Sword House", 5, &[1, 0, 2]),
	Room("Links House Storage", 7, &[0]),
	Room("Main Village", 0, &[0, 1, 2, 4, 5, 6, 7, 9, 11, 20, 21, 22, 23, 24, 25, 13, 14, 15, 26, 99, 27, 30, 100, 101, 102, 103]),
	Room("Outside Links House", 1, &[0, 2, 3, 20, 21, 6, 23, 7, 24, 25, 26, 99, 8, 27, 30, 1, 9, 10, 5, 100, 4, 11]),
];
pub static HIDDEN_SKILL: [Room; 1] = [
	Room("Hidden Skill", 0, &[2, 1, 3, 4, 5, 6, 7, 0]),
];
pub static UPPER_ZORAS_RIVER: [Room; 1] = [
	Room("Upper Zoras River", 0, &[99, 1, 2, 3, 100, 5, 6, 7, 8, 9, 0, 4, 101, 10, 200, 11]),
];
pub static MIRROR_CHAMBER: [Room; 1] = [
	Room("Mirror Chamber", 4, &[0, 1, 51, 52, 54, 55, 2, 3, 4, 56, 57, 5, 58, 7, 6, 8]),
];
pub static GERUDO_DESERT: [Room; 1] = [
	Room("Gerudo Desert", 0, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 50, 51, 52, 53, 111]),
];
pub static BUBLIN_2: [Room; 1] = [
	Room("Bublin 2", 13, &[0]),
];
pub static OUTSIDE_CASTLE_TOWN: [Room; 3] = [
	Room("West Field", 8, &[0, 1, 2, 254, 3, 4, 100, 101, 111, 5, 6, 7, 200, 76]),
	Room("South Field", 16, &[0, 1, 2, 111, 3, 4]),
	Room("East Field", 17, &[0, 1, 4]),
];
pub static HYRULE_FIELD: [Room; 15] = [
	Room("Eldin Main Field", 0, &[0, 1, 2, 3, 4, 5, 50, 6, 7, 8, 9, 10, 11, 12, 13, 20, 21, 14, 15, 16, 17, 18]),
	Room("Faron-Eldin Transition ", 1, &[0, 20, 21, 22]),
	Room("Eldin Entrance", 2, &[0, 10, 20, 1, 30]),
	Room("Eldin Gorge", 3, &[1, 99, 2, 10, 20, 3, 0, 21, 5, 88, 15, 4, 14, 6, 22, 16, 17]),
	Room("Eldin Gorge-Main Transition 1", 4, &[0, 1]),
	Room("Eldin Gorge-Main Transition 2", 5, &[0]),
	Room("Faron Main Field", 6, &[0, 1, 10, 21, 11, 2, 3, 100, 101, 12]),
	Room("Eldin-Lanayru Transition", 7, &[0, 22, 6, 1, 2, 14]),
	Room("Lanayru Entrance", 9, &[0, 1, 10, 2]),
	Room("Lanayru Main Field", 10, &[0, 1, 2, 3, 4, 20, 21, 22, 5, 23, 15, 6, 7, 8, 14, 16]),
	Room("Lanayru Main-Bridge Transition 1", 11, &[0, 1]),
	Room("Lanayru Main-Bridge Transition 2", 12, &[0, 1, 2, 21, 20, 3]),
	Room("Lanayru Lake Hylia Bridge", 13, &[0, 1, 99, 2, 20, 98, 21, 22, 23, 3, 4, 14]),
	Room("Faron-Lanayru Transition", 14, &[0, 1]),
	Room("Faron-Lanayru Gate", 15, &[0, 99, 20, 21, 22]),
];
pub static BUBLIN_CAMP: [Room; 4] = [
	Room("Camp Geometry (dont use!)", 0, &[]),
	Room("Main Camp", 1, &[0, 1, 2, 6]),
	Room("Beta Camp", 2, &[0]),
	Room("Before Arbiters Grounds", 3, &[0, 2, 3, 4, 5, 7]),
];
pub static SACRED_GROVE: [Room; 3] = [
	Room("Master Sword", 1, &[1, 254, 5, 6, 20, 21, 3, 99, 10, 150, 100, 102, 4, 50, 51, 200]),
	Room("Temple of Time", 2, &[0, 1, 101, 3, 102, 52]),
	Room("Lost Woods", 3, &[0, 1, 2, 3, 4, 5, 6]),
];
pub static LAKE_HYLIA: [Room; 2] = [
	Room("Lake", 0, &[0, 1, 2, 3, 5, 7, 254, 10, 11, 6, 12, 13, 14, 8, 15, 100, 50, 33, 9, 16, 20, 70, 25, 40, 101, 150, 55, 77, 78, 29, 30, 31, 32, 133, 4, 134, 17, 200, 34, 99, 76, 75]),
	Room("Fountain", 1, &[1, 0, 20, 21, 22, 23, 100]),
];
pub static SNOWPEAK: [Room; 3] = [
	Room("First Half", 0, &[0, 4, 7, 15, 14, 13, 100, 5, 1, 2, 6, 10]),
	Room("Second Half", 1, &[1, 2, 3, 5, 6, 9, 10, 11, 20, 12, 21, 13, 22, 100]),
	Room("Transition Cave", 2, &[8, 12, 13]),
];
pub static ZORAS_DOMAIN: [Room; 2] = [
	Room("Inside", 0, &[0, 1, 3, 4, 5, 254, 50, 7, 97, 8, 99, 10]),
	Room("Outside", 1, &[9, 5, 1, 6, 7, 3, 0, 4, 100, 8, 10, 11, 98, 12, 13, 14, 20, 101, 15, 30, 34]),
];
pub static ZORAS_RIVER: [Room; 1] = [
	Room("Zoras River", 1, &[1, 11, 2, 15, 4, 5, 10, 6, 0, 12, 13, 14, 16, 17, 3, 7]),
];
pub static ORDON_SPRING: [Room; 1] = [
	Room("Ordon Spring", 1, &[0, 1, 2, 20, 21, 3, 22, 26, 5, 23, 24, 25, 99, 254, 200, 6, 10, 111, 4, 30, 100]),
];
pub static TITLE_SCREEN: [Room; 1] = [
	Room("Title Screen", 0, &[0, 1, 2, 20, 3, 53, 4, 5, 100, 101]),
];
pub static ORDON_RANCH: [Room; 1] = [
	Room("Ordon Ranch", 0, &[0, 1, 2, 3, 20, 4, 99, 30, 5, 6, 7, 127]),
];
pub static FARON_WOODS_CAVE: [Room; 1] = [
	Room("Faron Woods Cave", 0, &[0, 21, 20, 1]),
];
pub static GROTTO_5: [Room; 1] = [
	Room("Grotto 5", 4, &[0, 1]),
];
pub static GROTTO_4: [Room; 1] = [
	Room("Grotto 4", 3, &[0, 1]),
];
pub static GROTTO_3: [Room; 1] = [
	Room("Grotto 3", 2, &[0, 1]),
];
pub static GROTTO_2: [Room; 1] = [
	Room("Grotto 2", 1, &[0, 1]),
];
pub static GROTTO_1: [Room; 1] = [
	Room("Grotto 1", 0, &[0, 1]),
];
pub static ELDIN_GORON_STOCKCAVE: [Room; 1] = [
	Room("Eldin Goron Stockcave", 10, &[0, 1]),
];
pub static LAKE_HYLIA_LONG_CAVE: [Room; 1] = [
	Room("Lake Hylia Long Cave", 0, &[0, 1]),
];
pub static ELDIN_LONG_CAVE: [Room; 1] = [
	Room("Eldin Long Cave", 0, &[0]),
];
pub static CAVE_OF_ORDEALS: [Room; 50] = [
	Room("Room 1", 0, &[0]),
	Room("Room 2", 1, &[0]),
	Room("Room 3", 2, &[0]),
	Room("Room 4", 3, &[0]),
	Room("Room 5", 4, &[0]),
	Room("Room 6", 5, &[0]),
	Room("Room 7", 6, &[0]),
	Room("Room 8", 7, &[0]),
	Room("Room 9", 8, &[0, 1]),
	Room("Room 10", 9, &[0]),
	Room("Room 11", 10, &[0]),
	Room("Room 12", 11, &[0]),
	Room("Room 13", 12, &[0]),
	Room("Room 14", 13, &[0]),
	Room("Room 15", 14, &[0]),
	Room("Room 16", 15, &[0]),
	Room("Room 17", 16, &[0]),
	Room("Room 18", 17, &[0]),
	Room("Room 19", 18, &[0, 1]),
	Room("Room 20", 19, &[0]),
	Room("Room 21", 20, &[0]),
	Room("Room 22", 21, &[0]),
	Room("Room 23", 22, &[0]),
	Room("Room 24", 23, &[0]),
	Room("Room 25", 24, &[0]),
	Room("Room 26", 25, &[0]),
	Room("Room 27", 26, &[0]),
	Room("Room 28", 27, &[0]),
	Room("Room 29", 28, &[0, 1]),
	Room("Room 30", 29, &[0]),
	Room("Room 31", 30, &[0]),
	Room("Room 32", 31, &[0]),
	Room("Room 33", 32, &[0]),
	Room("Room 34", 33, &[0]),
	Room("Room 35", 34, &[0]),
	Room("Room 36", 35, &[0]),
	Room("Room 37", 36, &[0]),
	Room("Room 38", 37, &[0]),
	Room("Room 39", 38, &[0, 1]),
	Room("Room 40", 39, &[0]),
	Room("Room 41", 40, &[0]),
	Room("Room 42", 41, &[0]),
	Room("Room 43", 42, &[0]),
	Room("Room 44", 43, &[0]),
	Room("Room 45", 44, &[0]),
	Room("Room 46", 45, &[0]),
	Room("Room 47", 46, &[0]),
	Room("Room 48", 47, &[0]),
	Room("Room 49", 48, &[0, 1]),
	Room("Room 50", 49, &[0]),
];
pub static LANAYRU_ICE_PUZZLE_CAVE: [Room; 1] = [
	Room("Lanayru Ice Puzzle Cave", 0, &[0]),
];
pub static DARKHAMMER: [Room; 2] = [
	Room("Beta Room", 49, &[0, 1, 2]),
	Room("Darkhammer", 51, &[0, 1, 2, 3]),
];
pub static BLIZZETA: [Room; 1] = [
	Room("Blizzeta", 50, &[0, 1, 2, 3]),
];
pub static SNOWPEAK_RUINS: [Room; 12] = [
	Room("Entrance", 0, &[0, 1, 2, 3]),
	Room("Yeta", 1, &[0]),
	Room("Yeto", 2, &[0, 1, 2]),
	Room("Ice Puzzle", 3, &[0, 1, 4, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12]),
	Room("Courtyard", 4, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
	Room("Ordon Pumpkin", 5, &[0, 1, 2, 3, 4]),
	Room("Third Poe", 6, &[0, 1, 2]),
	Room("Double Freezard", 7, &[0, 10]),
	Room("Double LJA", 8, &[0]),
	Room("Ice Cannon Room", 9, &[0]),
	Room("Boss Key", 11, &[0]),
	Room("Before Ordon Pumpkin", 13, &[0]),
];
pub static DEATH_SWORD: [Room; 1] = [
	Room("Death Sword", 51, &[0, 1, 2]),
];
pub static STALLORD: [Room; 1] = [
	Room("Stallord", 50, &[0, 1, 2, 3]),
];
pub static ARBITERS_GROUNDS: [Room; 17] = [
	Room("Entrance", 0, &[2, 1, 0, 3]),
	Room("Before Main Room", 1, &[0]),
	Room("Main Poe Room", 2, &[0, 2, 1, 3]),
	Room("2nd Poe", 3, &[0]),
	Room("Before 3rd Poe", 4, &[0, 1, 2, 3]),
	Room("3rd Poe", 5, &[0]),
	Room("Left Wing 1", 6, &[0, 1]),
	Room("Left Wing 2", 7, &[0]),
	Room("4th Poe", 8, &[0]),
	Room("Boss Key", 9, &[0, 1, 2]),
	Room("Ooccoo", 10, &[0]),
	Room("Before Death Sword", 11, &[0, 1, 2, 3]),
	Room("Before 4th Poe", 12, &[0]),
	Room("Epic Spinner Room", 13, &[0, 1]),
	Room("After 3rd Poe", 14, &[0]),
	Room("Right Wing", 15, &[0, 1]),
	Room("Big Turning Room", 16, &[0]),
];
pub static GANONDORF_DEFEATED: [Room; 1] = [
	Room("Ganondorf Defeated", 0, &[0, 20, 21, 23, 22]),
];
pub static GANONDORF_FIELD: [Room; 1] = [
	Room("Ganondorf Field", 0, &[0, 1]),
];
pub static GANONDORF_CASTLE: [Room; 2] = [
	Room("Fight Inside", 50, &[0, 1, 2, 20, 21, 22, 120, 121, 122, 10]),
	Room("Intro Outside", 51, &[0, 1, 2, 20, 21, 22, 120, 121, 122, 10]),
];
pub static HYRULE_CASTLE: [Room; 13] = [
	Room("Inside Main Hall", 1, &[0, 1, 1, 2, 3]),
	Room("Inside Darknut 1", 2, &[0, 2, 3]),
	Room("Inside Left Wing 1", 3, &[0]),
	Room("Inside Left Wing 2", 4, &[1, 0, 2]),
	Room("Inside Right Wing 1", 5, &[0]),
	Room("Inside Right Wing 2", 6, &[0, 1]),
	Room("Treasure Room", 8, &[0]),
	Room("Graveyard", 9, &[0, 1, 2]),
	Room("Entrance", 11, &[1, 2, 3, 0, 5]),
	Room("Inside Final Ascension", 12, &[0, 1, 2, 3, 4, 5, 6, 7, 8]),
	Room("Outside Left Wing", 13, &[0]),
	Room("Outside Right Wing", 14, &[1, 2, 3, 0, 4, 5]),
	Room("Outside Boss Key", 15, &[1, 0, 3, 2, 4, 5, 6, 7]),
];
pub static ZANT_FIGHT: [Room; 7] = [
	Room("Intro", 50, &[20, 0]),
	Room("Diababa Phase", 53, &[0]),
	Room("Goron Mines Phase", 54, &[0]),
	Room("Lakebed Phase", 55, &[0, 1]),
	Room("Ook Phase", 56, &[0]),
	Room("Blizzeta Phase", 57, &[0]),
	Room("Final Hyrule Phase", 60, &[0]),
];
pub static PHANTOM_ZANT_2: [Room; 1] = [
	Room("Phantom Zant 2", 52, &[0, 2]),
];
pub static PHANTOM_ZANT_1: [Room; 1] = [
	Room("Phantom Zant 1", 51, &[0, 1, 2, 3]),
];
pub static ZANT_MAIN_ROOM: [Room; 1] = [
	Room("Zant Main Room", 10, &[0, 1, 21, 23, 24, 25]),
];
pub static PALACE_OF_TWILIGHT: [Room; 10] = [
	Room("Entrance", 0, &[0, 1, 2, 3, 10, 20, 21, 22, 4]),
	Room("Left Wing 1", 1, &[0, 1]),
	Room("Left Wing 2", 2, &[0, 1]),
	Room("Right Wing 1", 4, &[0, 1]),
	Room("Right Wing 2", 5, &[0, 1, 2]),
	Room("Double Sol", 7, &[0, 1]),
	Room("Early Platform", 8, &[0]),
	Room("Messengers before Zant", 9, &[0, 1, 20, 21]),
	Room("Beta Zant Room", 10, &[0, 1]),
	Room("Boss Key", 11, &[0, 1, 20, 22, 21]),
];
pub static AERALFOS: [Room; 1] = [
	Room("Aeralfos", 51, &[0, 1, 2]),
];
pub static ARGOROK: [Room; 1] = [
	Room("Argorok", 50, &[1, 0, 2, 3]),
];
pub static CITY_IN_THE_SKY: [Room; 16] = [
	Room("Entrance", 0, &[2, 1, 0, 3, 4, 5]),
	Room("Before Main Room", 1, &[0]),
	Room("Main Room", 2, &[0, 1, 2, 3, 4]),
	Room("Right Wing Outside", 3, &[1, 0, 2, 3]),
	Room("Right Wing Inside 1", 4, &[0, 1, 2]),
	Room("Before Aeralfos", 5, &[0, 1, 2]),
	Room("Left Wing Outside ", 6, &[0, 1, 2, 3, 4, 5, 6, 7, 8]),
	Room("Right Wing Inside 2", 7, &[0]),
	Room("Right Wing Inside 3", 8, &[0]),
	Room("Left Wing Inside", 10, &[0, 1, 3]),
	Room("Big Baba Room", 11, &[0, 1]),
	Room("After Big Baba Outside", 12, &[0, 1, 2, 3]),
	Room("Before Boss Key Outside", 13, &[0, 1]),
	Room("North Wing Outside", 14, &[0, 1, 3]),
	Room("North Wing Inside", 15, &[0, 1, 3, 4]),
	Room("Oocca Shop", 16, &[0, 1, 2]),
];
pub static DARKNUT: [Room; 1] = [
	Room("Darknut", 51, &[0]),
];
pub static ARMOGOHMA: [Room; 1] = [
	Room("Armogohma", 50, &[0, 1]),
];
pub static TEMPLE_OF_TIME: [Room; 9] = [
	Room("Entrance", 0, &[0, 1]),
	Room("First Staircase", 1, &[0]),
	Room("Turning Platform", 2, &[0, 1, 2]),
	Room("Statue Throws", 3, &[0, 1]),
	Room("Second Staircase", 4, &[0, 1]),
	Room("Scale Room", 5, &[0]),
	Room("Boss Key", 6, &[0]),
	Room("Third Staircase", 7, &[0, 1, 2]),
	Room("Before Gohma", 8, &[0, 1, 2]),
];
pub static OOK: [Room; 1] = [
	Room("Ook", 51, &[1, 0, 2]),
];
pub static DIABABA: [Room; 1] = [
	Room("Diababa", 50, &[0, 1]),
];
pub static FOREST_TEMPLE: [Room; 13] = [
	Room("Main Room", 0, &[0]),
	Room("Right Wing Boss Key", 1, &[0]),
	Room("2nd Monkey", 2, &[0]),
	Room("Left Wing", 3, &[0, 1]),
	Room("Outside", 4, &[1, 0, 2]),
	Room("3rd Monkey", 5, &[0, 1]),
	Room("4th Monkey", 7, &[0]),
	Room("North Wing Turning Bridge", 9, &[0]),
	Room("Final Monkey", 10, &[0]),
	Room("6th Monkey", 11, &[0]),
	Room("Before Diababa", 12, &[0, 1]),
	Room("7th Monkey", 19, &[0]),
	Room("Entrance", 22, &[0]),
];
pub static DANGORO: [Room; 1] = [
	Room("Dangoro", 51, &[2, 1, 0, 3]),
];
pub static FYRUS: [Room; 1] = [
	Room("Fyrus", 50, &[0, 1]),
];
pub static GORON_MINES: [Room; 13] = [
	Room("Entrance", 1, &[1, 0]),
	Room("Magnet Room", 3, &[0]),
	Room("Roll Clipping", 4, &[0, 1]),
	Room("Before 1st Elder", 5, &[0]),
	Room("Clawshot Switch", 6, &[0, 1]),
	Room("Outside", 7, &[0, 1]),
	Room("Before Dangoro", 9, &[0, 1, 2, 3]),
	Room("Bow", 11, &[1, 0]),
	Room("Before Fyrus", 12, &[0, 1]),
	Room("Bow-Magnet Shortcut Room", 13, &[0]),
	Room("1st Elder", 14, &[0, 1]),
	Room("3rd Elder", 16, &[0]),
	Room("2nd Elder", 17, &[0, 1]),
];
pub static DEKU_TOAD: [Room; 1] = [
	Room("Deku Toad", 51, &[1, 2, 0, 3]),
];
pub static MORPHEEL: [Room; 1] = [
	Room("Morpheel", 50, &[0, 1, 2, 3]),
];
pub static LAKEBED_TEMPLE: [Room; 13] = [
	Room("Entrance", 0, &[0, 1, 2]),
	Room("Stalactite Room", 1, &[0]),
	Room("Central Room Outside", 2, &[0]),
	Room("Central Room", 3, &[0, 1, 2]),
	Room("Before Boss Key", 5, &[0, 1, 2]),
	Room("Boss Key", 6, &[0, 1, 2]),
	Room("Right Wing Upper", 7, &[0]),
	Room("Right Wing Lower", 8, &[0, 2]),
	Room("Before Deku Toad", 9, &[0, 1, 2, 3, 4]),
	Room("Right Wing Water Supply", 10, &[0, 1]),
	Room("Left Wing Upper", 11, &[0]),
	Room("Left Wing Lower", 12, &[0, 1, 2]),
	Room("Left Wing Water Supply", 13, &[0]),
];
