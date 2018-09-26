use *;

pub fn move_cursor(len: usize, cursor: &mut usize) {
    let state = unsafe { super::get_state() };
    let lines = state.menu.lines_mut();

    if controller::DPAD_UP.is_pressed() && *cursor > 0 {
        *cursor -= 1;
        while lines[*cursor].len() < 3 {
            *cursor -= 1;
        }
    } else if controller::DPAD_DOWN.is_pressed() && *cursor + 1 < len {
        *cursor += 1;
        while lines[*cursor].len() < 3 {
            *cursor += 1;
        }
    }
}

pub fn scroll_move_cursor(len: usize, cursor: &mut usize, scroll_offset: &mut usize) {
    let state = unsafe { get_state() };
    const padding: usize = 2;
    if controller::DPAD_UP.is_pressed() && *cursor > 0 {
        *cursor -= 1;
        if *cursor >= padding && *cursor - padding < *scroll_offset {
            *scroll_offset = *cursor - padding;
        }
    } else if controller::DPAD_DOWN.is_pressed() && *cursor + 1 < len {
        *cursor += 1;
        if *cursor + padding < len
            && *cursor > *scroll_offset + (state.settings.max_lines - (padding + 1))
        {
            *scroll_offset = *cursor - (state.settings.max_lines - (padding + 1));
        }
    }
}

pub fn clear_menu() {
    let state = unsafe { super::get_state() };
    let lines = state.menu.lines_mut();
    for line in lines.into_iter() {
        line.clear();
    }
}

macro_rules! define_menu {
    ( $( ($name:ident, $m:ident)),* ) => (
        #[derive(Copy, Clone)]
        pub enum MenuState {
            $(
                $name,
            )*
        }

        pub fn transition(state: MenuState) {
            clear_menu();
            unsafe { menu_state = state; }
            match state {
                $(
                    MenuState::$name => $m::transition_into(),
                )*
            }
        }

        pub fn render() {
            match unsafe { menu_state } {
                $(
                    MenuState::$name => $m::render(),
                )*
            }
        }
    )
}

pub static mut menu_state: MenuState = MenuState::MainMenu;

define_menu!(
    (MainMenu, main_menu),
    (Memory, memory),
    (InventoryMenu, inventory),
    (CheatMenu, cheat_menu),
    (Settings, settings)
    // (Warp, warping)
);
