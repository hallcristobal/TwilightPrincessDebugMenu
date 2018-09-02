use arrayvec::ArrayString;
use arrayvec::ArrayVec;
use core::fmt::Write;
use core::fmt::{Debug, Display, Error, Formatter};
use core::mem::size_of;
use libtp::system::memory;
use libtp::Addr;

use controller;
use core::cell::RefCell;
use libtp::system::mutex::Mutex;
use print;
use utils::*;

pub const MAX_WATCH: usize = 32;
pub const WATCH_SIZE: usize = size_of::<Watch>();

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Type {
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    f32,
    String,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Type::u8 => write!(f, "u8"),
            Type::i8 => write!(f, "i8"),
            Type::u16 => write!(f, "u16"),
            Type::i16 => write!(f, "i16"),
            Type::u32 => write!(f, "u32"),
            Type::i32 => write!(f, "i32"),
            Type::f32 => write!(f, "f32"),
            Type::String => write!(f, "String"),
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Watch {
    addr: Addr,
    x: f32,
    y: f32,
    t: Type,
    offset: Option<u16>,
    hex: bool,
    visible: bool,
    val_addr: Addr,
}

impl Watch {
    pub fn new(
        addr: Addr,
        x: f32,
        y: f32,
        t: Type,
        hex: bool,
        visible: bool,
        offset: Option<u16>,
    ) -> Self {
        Watch {
            addr,
            x,
            y,
            t,
            offset,
            hex,
            visible,
            val_addr: 0x8000000,
        }
    }
    fn update(&mut self) {
        self.val_addr = if let Some(offset) = self.offset {
            memory::read::<Addr>(self.addr) + offset as usize
        } else {
            self.addr
        }
    }

    fn write_val(&self, f: &mut Formatter) -> Result<(), Error> {
        if self.val_addr < 0x80000000 || self.val_addr > 0x8FFFFFFF {
            return write!(f, "N/A");
        }
        match self.t {
            Type::u8 => {
                let value = memory::read::<u8>(self.val_addr);
                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::i8 => {
                let value = memory::read::<i8>(self.val_addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::u16 => {
                let value = memory::read::<u16>(self.val_addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::i16 => {
                let value = memory::read::<i16>(self.val_addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::u32 => {
                let value = memory::read::<u32>(self.val_addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::i32 => {
                let value = memory::read::<i32>(self.val_addr);

                if self.hex {
                    write!(f, "{:X}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Type::f32 => {
                if self.hex {
                    let value = memory::read::<u32>(self.val_addr);
                    write!(f, "{:X}", value)
                } else {
                    let value = memory::read::<f32>(self.val_addr);
                    write!(f, "{:.*}", 5, value)
                }
            }
            Type::String => {
                let value = memory::read_str(memory::ptr(self.val_addr));
                write!(f, "{}", value)
            }
        }
    }
}

impl Default for Watch {
    fn default() -> Self {
        Watch {
            addr: 0x80000000,
            x: 100.0,
            y: 100.0,
            t: Type::String,
            offset: None,
            hex: true,
            visible: false,
            val_addr: 0x8000000,
        }
    }
}

impl Display for Watch {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.write_val(f)
    }
}

impl Debug for Watch {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut string = ArrayString::<[u8; 10]>::new();
        write!(
            f,
            "{:#08X} {} {} {} {} {} [{}] ",
            self.addr,
            self.x,
            self.y,
            self.hex,
            self.t,
            if let Some(offset) = self.offset {
                let _ = write!(string, "{:#06X}", offset);
                string.as_str()
            } else {
                "None"
            },
            if self.visible { "x" } else { " " },
        )?;
        self.write_val(f)
    }
}

lazy_static! {
    pub static ref ITEMS: Mutex<ArrayVec<[Watch; MAX_WATCH]>> =
        Mutex(RefCell::new(ArrayVec::new()));
}

pub fn transition_into() {}

pub fn render_watches() {
    ITEMS.borrow_mut().iter_mut().for_each(|item| {
        if item.visible {
            item.update();
            let mut s = ArrayString::<[u8; 64]>::new();
            let _ = write!(s, "{}", item);
            unsafe {
                print::printf(s.as_str(), item.x as f32, item.y as f32, 0xFF_FF_FF_FF);
            }
        }
    });
}

#[derive(Debug)]
enum SelectedPhase {
    Base,
    Word,
    Char,
}

static mut PHASE: SelectedPhase = SelectedPhase::Base;
static mut cursor: usize = 0;
static mut word_cursor: usize = 0;
static mut char_cursor: usize = 2;

pub fn render() {
    let state = unsafe { super::get_state() };
    let menu = &mut state.menu;
    menu.clear();

    let pressed_b = controller::B.is_pressed();
    let pressed_a = controller::A.is_pressed();
    let pressed_x = controller::X.is_pressed();
    let pressed_y = controller::Y.is_pressed();
    let pressed_u = controller::DPAD_UP.is_pressed();
    let pressed_d = controller::DPAD_DOWN.is_pressed();
    let pressed_l = controller::DPAD_LEFT.is_pressed();
    let pressed_r = controller::DPAD_RIGHT.is_pressed();

    unsafe {
        match &PHASE {
            SelectedPhase::Base => {
                if pressed_b {
                    transition(MenuState::MainMenu);
                    return;
                }
                if pressed_a {
                    if ITEMS.borrow().len() > 0 {
                        PHASE = SelectedPhase::Word;
                    }
                }

                if pressed_x {
                    let mut items = ITEMS.borrow_mut();
                    if items.len() < MAX_WATCH {
                        items.push(Watch::default());
                    }
                }

                if pressed_y {
                    let mut items = ITEMS.borrow_mut();
                    if items.len() > 0 {
                        if cursor < items.len() {
                            items.remove(cursor);
                        }
                    }
                }

                if cursor >= ITEMS.borrow().len() {
                    cursor = ITEMS.borrow().len() - 1;
                }

                if pressed_u && cursor > 0 {
                    cursor -= 1;
                } else if pressed_d && cursor + 1 < ITEMS.borrow().len() {
                    cursor += 1;
                }
            }
            SelectedPhase::Word => {
                let mut current_watch: Watch = ITEMS.borrow_mut().remove(cursor);
                menu.selected_word = Some(word_cursor);
                if pressed_b {
                    PHASE = SelectedPhase::Base;
                    word_cursor = 0;
                    menu.selected_word = None;
                }
                if pressed_a {
                    if word_cursor == 6 {
                        current_watch.visible = !current_watch.visible;
                    } else if word_cursor == 0 {
                        PHASE = SelectedPhase::Char;
                    } else if word_cursor == 5 && current_watch.offset.is_some() {
                        PHASE = SelectedPhase::Char;
                    } else if word_cursor == 3 {
                        current_watch.hex = !current_watch.hex;
                    }
                }
                if pressed_y && word_cursor == 5 {
                    current_watch.offset = Some(0);
                }
                if pressed_x && word_cursor == 5 {
                    current_watch.offset = None;
                }

                if pressed_l && word_cursor > 0 {
                    word_cursor -= 1;
                } else if pressed_r && word_cursor < 6 {
                    word_cursor += 1;
                } else if pressed_u {
                    match word_cursor {
                        1 => {
                            current_watch.x += 5.0;
                        }
                        2 => {
                            current_watch.y += 5.0;
                        }
                        4 => {
                            current_watch.t = match current_watch.t {
                                Type::u8 => Type::i8,
                                Type::i8 => Type::u16,
                                Type::u16 => Type::i16,
                                Type::i16 => Type::u32,
                                Type::u32 => Type::i32,
                                Type::i32 => Type::f32,
                                Type::f32 => Type::String,
                                Type::String => Type::u8,
                            }
                        }
                        _ => {}
                    }
                } else if pressed_d {
                    match word_cursor {
                        1 => {
                            current_watch.x -= 5.0;
                        }
                        2 => {
                            current_watch.y -= 5.0;
                        }
                        4 => {
                            current_watch.t = match current_watch.t {
                                Type::u8 => Type::String,
                                Type::i8 => Type::u8,
                                Type::u16 => Type::i8,
                                Type::i16 => Type::u16,
                                Type::u32 => Type::i16,
                                Type::i32 => Type::u32,
                                Type::f32 => Type::i32,
                                Type::String => Type::f32,
                            }
                        }
                        _ => {}
                    }
                }

                ITEMS.borrow_mut().insert(cursor, current_watch);
            }
            SelectedPhase::Char => {
                let mut current_watch: Watch = ITEMS.borrow_mut().remove(cursor);
                menu.selected_char = Some(char_cursor);
                if pressed_b {
                    PHASE = SelectedPhase::Word;
                    char_cursor = 2;
                    menu.selected_char = None;
                }

                let max_char = match word_cursor {
                    0 => 9,
                    5 => 5,
                    _ => 0,
                };

                if pressed_l && char_cursor > 2 {
                    char_cursor -= 1;
                } else if pressed_r && char_cursor < max_char {
                    char_cursor += 1;
                } else if pressed_u {
                    match word_cursor {
                        0 => match char_cursor {
                            3 => {
                                if current_watch.addr < 0x8F000000 {
                                    current_watch.addr += 0x1000000;
                                }
                            }
                            4 => {
                                if current_watch.addr < 0x8FF00000 {
                                    current_watch.addr += 0x100000;
                                }
                            }
                            5 => {
                                if current_watch.addr < 0x8FFF0000 {
                                    current_watch.addr += 0x10000;
                                }
                            }
                            6 => {
                                if current_watch.addr < 0x8FFFF000 {
                                    current_watch.addr += 0x1000;
                                }
                            }
                            7 => {
                                if current_watch.addr < 0x8FFFFF00 {
                                    current_watch.addr += 0x100;
                                }
                            }
                            8 => {
                                if current_watch.addr < 0x8FFFFFF0 {
                                    current_watch.addr += 0x10;
                                }
                            }
                            9 => {
                                if current_watch.addr < 0x8FFFFFFF {
                                    current_watch.addr += 0x1;
                                }
                            }
                            _ => {}
                        },
                        5 => {
                            if let Some(offset) = current_watch.offset {
                                match char_cursor {
                                    2 => {
                                        if offset < 0xEFFF {
                                            current_watch.offset = offset.checked_add(0x1000);
                                        }
                                    }
                                    3 => {
                                        if offset < 0xFEFF {
                                            current_watch.offset = offset.checked_add(0x100);
                                        }
                                    }
                                    4 => {
                                        if offset < 0xFFEF {
                                            current_watch.offset = offset.checked_add(0x10);
                                        }
                                    }
                                    5 => {
                                        if offset < 0xFFFE {
                                            current_watch.offset = offset.checked_add(0x1);
                                        }
                                    }
                                    _ => {}
                                }
                            } else {
                                current_watch.offset = Some(1);
                            }
                        }
                        _ => {}
                    }
                } else if pressed_d {
                    match word_cursor {
                        0 => match char_cursor {
                            3 => {
                                if current_watch.addr > 0x80FFFFFF {
                                    current_watch.addr -= 0x1000000;
                                }
                            }
                            4 => {
                                if current_watch.addr > 0x800FFFFF {
                                    current_watch.addr -= 0x100000;
                                }
                            }
                            5 => {
                                if current_watch.addr > 0x8000FFFF {
                                    current_watch.addr -= 0x10000;
                                }
                            }
                            6 => {
                                if current_watch.addr > 0x80000FFF {
                                    current_watch.addr -= 0x1000;
                                }
                            }
                            7 => {
                                if current_watch.addr > 0x800000FF {
                                    current_watch.addr -= 0x100;
                                }
                            }
                            8 => {
                                if current_watch.addr > 0x8000000F {
                                    current_watch.addr -= 0x10;
                                }
                            }
                            9 => {
                                if current_watch.addr > 0x80000000 {
                                    current_watch.addr -= 0x1;
                                }
                            }
                            _ => {}
                        },
                        5 => {
                            if let Some(offset) = current_watch.offset {
                                match char_cursor {
                                    2 => {
                                        if offset > 0x0FFF {
                                            if let Some(a) = (offset).checked_sub(0x1000) {
                                                current_watch.offset = Some(a);
                                            } else {
                                                current_watch.offset = None;
                                            }
                                        }
                                    }
                                    3 => {
                                        if offset > 0x00FF {
                                            if let Some(a) = (offset).checked_sub(0x100) {
                                                current_watch.offset = Some(a);
                                            } else {
                                                current_watch.offset = None;
                                            }
                                        }
                                    }
                                    4 => {
                                        if offset > 0x000F {
                                            if let Some(a) = (offset).checked_sub(0x10) {
                                                current_watch.offset = Some(a);
                                            } else {
                                                current_watch.offset = None;
                                            }
                                        }
                                    }
                                    5 => {
                                        if offset > 0x0000 {
                                            if let Some(a) = (offset).checked_sub(0x1) {
                                                current_watch.offset = Some(a);
                                            } else {
                                                current_watch.offset = None;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }

                ITEMS.borrow_mut().insert(cursor, current_watch);
            }
        }
    }
    let lines = menu.lines_mut();
    let _ = write!(
        lines[0].begin(),
        "Address  X    Y    Hex   Type  Offset  Show"
    );
    for (index, (line, content)) in lines
        .into_iter()
        .skip(1)
        .zip(ITEMS.borrow_mut().iter_mut())
        .enumerate()
    {
        content.update();
        let _ = write!(line.begin(), "{:?}", content);
        line.selected = index == unsafe { cursor };
    }
}
