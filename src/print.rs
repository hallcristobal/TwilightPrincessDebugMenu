use super::{get_state, State};

pub unsafe fn setup_draw() {
    let state = get_state();
    state.font.setup_rendering();
}

pub unsafe fn printf(s: &str, x: f32, y: f32, top_color: u32) {
    let State { font, .. } = get_state();
    font.render_chars(s.chars(), x, y, top_color);
}
