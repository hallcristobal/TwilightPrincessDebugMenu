use arrayvec::ArrayVec;
use core::i32;
use core::result;
use core::str;
use bincode;

const FILE_NAME_MAX: usize = 32;
const CARD_ICON_MAX: usize = 8;
#[allow(dead_code)]
const CARD_ICON_WIDTH: usize = 32;
#[allow(dead_code)]
const CARD_ICON_HEIGHT: usize = 32;
#[allow(dead_code)]
const CARD_BANNER_WIDTH: usize = 96;
#[allow(dead_code)]
const CARD_BANNER_HEIGHT: usize = 32;
const FILE_NAME: &str = "tpgz01";
const MIN_SAVE_AMNT: usize = 0x2000;

type Result<T> = result::Result<T, CardResult>;
use memory::{Watch, MAX_WATCH};
use cheat_menu::{CHEAT_AMNT};
type CheatsLayout = (ArrayVec<[Watch; MAX_WATCH]>, ArrayVec<[bool; CHEAT_AMNT]>);

#[derive(PartialEq, Debug)]
pub enum CardResult {
    Unlocked = 1,
    Ready = 0,
    Busy = -1,
    Wrongdevice = -2,
    Nocard = -3,
    Nofile = -4,
    IOerror = -5,
    Broken = -6,
    Exist = -7,
    Noent = -8,
    Insspace = -9,
    Noperm = -10,
    Limit = -11,
    Nametoolong = -12,
    Encoding = -13,
    Canceled = -14,
    FatalError = -128,
}

impl From<i32> for CardResult {
    fn from(i: i32) -> CardResult {
        match i {
            1 => CardResult::Unlocked,
            0 => CardResult::Ready,
            -1 => CardResult::Busy,
            -2 => CardResult::Wrongdevice,
            -3 => CardResult::Nocard,
            -4 => CardResult::Nofile,
            -5 => CardResult::IOerror,
            -6 => CardResult::Broken,
            -7 => CardResult::Exist,
            -8 => CardResult::Noent,
            -9 => CardResult::Insspace,
            -10 => CardResult::Noperm,
            -11 => CardResult::Limit,
            -12 => CardResult::Nametoolong,
            -13 => CardResult::Encoding,
            -14 => CardResult::Canceled,
            -128 => CardResult::FatalError,
            _ => unreachable!(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Default)]
struct CardInfo {
    chan: i32,
    file_num: i32,
    offset: i32,
    length: i32,
    i_block: u16,
}

#[repr(C)]
#[derive(Debug, Default)]
struct CardStat {
    // Read-only (Set by CARDGetStatus)
    file_name: [u8; FILE_NAME_MAX],
    length: u32,
    game_name: [u8; 4],
    company: [u8; 2],

    // Read/Write (Set by CARDGetStatus/CardSetStatus)
    banner_format: u8,
    icon_addr: u32,
    icon_format: u16,
    icon_speed: u16,
    commend_addr: u32,

    // Read-only (Set by CARDGetStatus)
    offset_banner: u32,
    offset_banner_tlut: u32,
    offset_icon: [u32; CARD_ICON_MAX],
    offset_icon_tlut: u32,
    offset_data: u32,
}

pub struct Card {
    card_info: CardInfo,
    card_stat: CardStat,
    sector_size: i32,
    card_size: i32,
    save_size: i32,
}

macro_rules! card_try {
    ($e:expr, $id:expr) => {
        let result: CardResult = unsafe { $e.into() };
        if result == CardResult::Ready {
            report!("Card {}: {:#?}", $id, result);
        } else {
            report!("Card {} failed. -> {:?}", $id, result);
            return Err(result);
        }
    };
}

impl Card {
    pub fn init() -> Result<Card> {
        let (sector_size, card_size) = {
            let mut sector_size = i32::MAX;
            let mut card_size = i32::MAX;
            card_try!(
                card_probe_ex(0, &mut card_size, &mut sector_size),
                "Card Probe Ex"
            );
            report!(
                "Sector Size: {} (0x{:X}) Card Size: {} (0x{:X})",
                sector_size,
                sector_size,
                card_size,
                card_size
            );
            (sector_size, card_size)
        };

        //make sure we take enough sectors of the sector size that we need, and are aligned properly

        Ok(Card {
            card_info: CardInfo::default(),
            card_stat: CardStat::default(),
            sector_size,
            card_size,
            save_size: sector_size,
        })
    }

    pub fn create(&mut self) -> Result<CardResult> {
        let result: CardResult = unsafe {
            card_create(
                0,
                FILE_NAME.as_ptr(),
                self.sector_size as u32,
                &mut self.card_info,
            ).into()
        };
        if result == CardResult::Ready || result == CardResult::Exist {
            report!("Card Info: {:?}", self.card_info);
            card_try!(
                card_get_status(0, self.card_info.file_num, &mut self.card_stat),
                "Card Get Status"
            );
            Ok(result)
        } else {
            report!("Card Create Failed. -> {:?}", result);
            Err(result)
        }
    }

    pub fn write(&mut self) -> Result<()> {
        let mut bytes = ArrayVec::<[u8; MIN_SAVE_AMNT]>::new();
        {
            let cheats = unsafe { ::cheat_menu::get_cheats() };
            let items = ::memory::ITEMS.borrow();
            let active_c = cheats.iter().map(|c| c.active).collect::<ArrayVec<[_; CHEAT_AMNT]>>();
            bincode::serialize_into(&mut bytes, &(&*items, active_c)).map_err(|e| {
                report!("Failed Serializing for memorycard: {:?}", e);
                CardResult::Encoding
            })?;
        }

        card_try!(
            card_open(0, FILE_NAME.as_ptr(), &mut self.card_info),
            "Card Open"
        );
        card_try!(
            card_write(&mut self.card_info, bytes.as_ptr(), MIN_SAVE_AMNT as i32, 0),
            "Card Write"
        );
        card_try!(card_close(&mut self.card_info), "Card Close");

        Ok(())
    }
}

extern "C" {
    #[link_name = "CARDOpen"]
    fn card_open(chan: i32, name: *const u8, card_info: *mut CardInfo) -> i32;
    #[link_name = "CARDClose"]
    fn card_close(card_info: *mut CardInfo) -> i32;
    #[link_name = "CARDCreate"]
    fn card_create(chan: i32, name: *const u8, size: u32, card_info: *mut CardInfo) -> i32;
    #[link_name = "CARDProbeEx"]
    fn card_probe_ex(chan: i32, card_size: *mut i32, sector_size: *mut i32) -> i32;
    #[link_name = "CARDGetStatus"]
    fn card_get_status(chan: i32, file_no: i32, card_stat: *mut CardStat) -> i32;
    #[link_name = "CARDSetStatus"]
    fn card_set_status(chan: i32, file_no: i32, card_stat: *mut CardStat) -> i32;
    #[link_name = "CARDWrite"]
    fn card_write(card_info: *mut CardInfo, data: *const u8, length: i32, offset: i32) -> i32;
}

/*
    why appending save not overwritng
    reading & deseralizing

*/
