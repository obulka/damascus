pub mod renderer;
pub mod style;
pub mod widget;


pub struct Config {
    pub font_size: u16,
    pub tab_bar_height: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            font_size: 13,
            tab_bar_height: 36,
        }
    }
}
