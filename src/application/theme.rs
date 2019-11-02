use tweek::prelude::*;

#[allow(unused_imports)]
use quicksilver::graphics::{Color, Font, FontStyle};

static ROBOTO_REGULAR: &[u8] = include_bytes!("../../static/Roboto-Regular.ttf");
static ROBOTO_BOLD: &[u8] = include_bytes!("../../static/Roboto-Bold.ttf");

pub struct ThemeManager {}

impl ThemeManager {
    /// TODO: Create another theme for nav bars, since white text is not right for body text
    pub fn default_theme() -> Theme {
        let mut theme = Theme::new(ROBOTO_REGULAR);
        theme.font_size = 18.0;
        theme.font_bytes = ROBOTO_REGULAR.into();
        theme.bg_color = Color::from_hex("#FFFFEE");
        theme.fg_color = Color::WHITE;

        let font = Font::from_slice(ROBOTO_BOLD).unwrap();
        theme.title_font = Some(font);
        theme
    }

    pub fn nav_theme() -> Theme {
        let mut theme = Theme::new(ROBOTO_BOLD);
        theme.font_size = 18.0;
        theme.font_bytes = ROBOTO_BOLD.into();
        theme.bg_color = Color::from_hex("#333333");
        theme.fg_color = Color::WHITE;

        let font = Font::from_slice(ROBOTO_BOLD).unwrap();
        theme.title_font = Some(font);
        theme

    }
}
