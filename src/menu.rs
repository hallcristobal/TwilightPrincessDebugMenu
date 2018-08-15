use arrayvec::ArrayString;
use core::fmt::Error;
use core::fmt::Write;
use print::*;

#[derive(Copy, Clone)]
pub struct Menu {
    x: f32,
    y: f32,
    offset: f32,
    selected_color: u32,
    color: u32,
    lines: [Line; 32],
    pub selected_word: Option<usize>,
    pub selected_char: Option<usize>,
}

#[derive(Copy, Clone)]
pub struct Line {
    pub text: ArrayString<[u8; 128]>,
    pub selected: bool,
}

impl Menu {
    pub fn new(x: f32, y: f32) -> Self {
        Menu {
            x,
            y,
            ..Self::default()
        }
    }

    pub fn lines(&self) -> &[Line] {
        &self.lines
    }

    pub fn lines_mut(&mut self) -> &mut [Line] {
        &mut self.lines
    }

    pub fn line(&mut self, index: usize) -> &Line {
        &self.lines[index]
    }

    pub fn line_mut(&mut self, index: usize) -> &mut Line {
        &mut self.lines[index]
    }

    pub fn clear(&mut self) {
        self.lines.iter_mut().for_each(|line| line.clear());
    }

    pub fn draw(&self) {
        let state = unsafe { super::get_state() };
        for (index, line) in self.lines.iter().enumerate() {
            let y = self.y + ((state.font.font.size * self.offset) * index as f32);
            let color = if line.selected {
                self.selected_color
            } else {
                self.color
            };

            if line.is_empty() {
                continue;
            }

            unsafe {
                if line.selected && self.selected_word.is_some() {
                    printf_select(
                        line.text.as_str(),
                        self.selected_word.unwrap(),
                        self.selected_char,
                        self.x,
                        y,
                        self.color,
                    );
                } else {
                    printf(line.text.as_str(), self.x, y, color);
                }
            }
        }
    }
}

impl Default for Menu {
    fn default() -> Self {
        Menu {
            x: 20.0,
            y: 20.0,
            offset: 1.2,
            selected_color: 0x00_00_FF_FF,
            color: 0xFF_FF_FF_FF,
            lines: [Line::new(); 32],
            selected_word: None,
            selected_char: None,
        }
    }
}

impl Line {
    fn new() -> Self {
        Line {
            text: ArrayString::default(),
            selected: false,
        }
    }

    pub fn begin(&mut self) -> LineWriter {
        self.clear();
        LineWriter {
            line: self,
            position: 0,
        }
    }

    pub fn append(&mut self) -> LineWriter {
        let len = self.len();
        LineWriter {
            line: self,
            position: len,
        }
    }

    pub fn write_at(&mut self, pos: usize) -> LineWriter {
        LineWriter {
            line: self,
            position: pos,
        }
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn push_str(&mut self, s: &str) {
        self.text.push_str(s);
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.selected = false;
    }
}

pub struct LineWriter<'a> {
    line: &'a mut Line,
    position: usize,
}

impl<'a> Write for LineWriter<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        if s.len() > 128 {
            self.line.push_str(&s[..128]);
        } else {
            self.line.push_str(s);
        }
        self.position += s.len();
        Ok(())
    }
}
