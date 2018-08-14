pub struct Settings {
    pub drop_shadow: bool,
    pub max_lines: usize,
}

pub const fn default_settings() -> Settings {
    Settings {
        drop_shadow: true,
        max_lines: 16,
    }
}
