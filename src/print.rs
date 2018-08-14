use super::{get_state, State};

pub unsafe fn setup_draw() {
    let state = get_state();
    state.font.setup_rendering();
}

pub unsafe fn printf(s: &str, x: f32, y: f32, top_color: u32) {
    let State { font, settings, .. } = get_state();
    if settings.drop_shadow {
        font.render_chars(s.chars(), x + 2.0, y + 2.0, 0x00_00_00_FF);
    }
    font.render_chars(s.chars(), x, y, top_color);
}

pub unsafe fn printf_select(
    s: &str,
    word_index: usize,
    char_index: Option<usize>,
    mut x: f32,
    y: f32,
    top_color: u32,
) {
    let State { font, settings, .. } = get_state();
    let split = s.split(' ');
    for (i, s) in split.into_iter().enumerate() {
        for (c_i, c) in s.chars().enumerate() {
            if settings.drop_shadow {
                font.render_char(c, x + 2.0, y + 2.0, 0x00_00_00_FF);
            }
            x = font.render_char(
                c,
                x,
                y,
                if let Some(char_index) = char_index {
                    if i == word_index && char_index == c_i {
                        0x00_00_FF_FF
                    } else {
                        top_color
                    }
                } else if i == word_index {
                    0x00_00_FF_FF
                } else {
                    top_color
                },
            );
        }
        x = font.render_char(' ', x, y, top_color);
    }
}
