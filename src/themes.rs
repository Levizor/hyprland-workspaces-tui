use clap::ValueEnum;
use ratatui::style::Color;

macro_rules! col {
    ($hex:expr) => {
        Color::from_u32($hex)
    };
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub button_fg: Color,     // foreground for workspace button text
    pub button_bg: Color,     // background for workspace button
    pub active_bg: Color,     // background for active workspace button
    pub focused_bg: Color,    // background for focused workspace button
    pub button_border: Color, // border around workspace button block
    pub active_fg: Color,     // foreground for active workspace button text
    pub focused_fg: Color,    // foreground for focused workspace button text
    pub transparent_bg: bool,
}

#[derive(ValueEnum, Debug, Clone, Default)]
pub enum Themes {
    #[default]
    TokyoNight,
    Nord,
    Gruvbox,
    Dracula,
    SolarizedDark,
    SolarizedLight,
    Monokai,
    OneDark,
    Catppuccin,
    AyuDark,
    TomorrowNight,
    Horizon,
}

impl Themes {
    pub fn get_theme(theme: &Option<Themes>, transparent_bg: bool) -> Theme {
        let theme = match theme.clone().unwrap_or_default() {
            Themes::Nord => Theme {
                button_fg: col!(0xD8DEE9),
                button_bg: col!(0x5E81AC),
                active_bg: col!(0x88C0D0),
                focused_bg: col!(0x81A1C1),
                button_border: col!(0x4C566A),
                active_fg: col!(0xECEFF4),
                focused_fg: col!(0xD8DEE9),
                transparent_bg,
            },
            Themes::Gruvbox => Theme {
                button_fg: col!(0xD5C4A1),
                button_bg: col!(0x8EC07C),
                active_bg: col!(0xB8A6A3),
                focused_bg: col!(0x7C6F3A),
                button_border: col!(0x7C6F3A),
                active_fg: col!(0xFBF1C7),
                focused_fg: col!(0xD5C4A1),
                transparent_bg,
            },
            Themes::Dracula => Theme {
                button_fg: col!(0xF8F8F2),
                button_bg: col!(0x6272A4),
                active_bg: col!(0x50FA7B),
                focused_bg: col!(0xFF79C6),
                button_border: col!(0x44475A),
                active_fg: col!(0x282A36),
                focused_fg: col!(0xF8F8F2),
                transparent_bg,
            },
            Themes::SolarizedDark => Theme {
                button_fg: col!(0x93A1A1),
                button_bg: col!(0x268BD2),
                active_bg: col!(0xB58900),
                focused_bg: col!(0x2AA198),
                button_border: col!(0x073642),
                active_fg: col!(0xFDF6E3),
                focused_fg: col!(0xFDF6E3),
                transparent_bg,
            },
            Themes::SolarizedLight => Theme {
                button_fg: col!(0x657B83),
                button_bg: col!(0x268BD2),
                active_bg: col!(0xB58900),
                focused_bg: col!(0x2AA198),
                button_border: col!(0xEEE8D5),
                active_fg: col!(0x002B36),
                focused_fg: col!(0x002B36),
                transparent_bg,
            },
            Themes::Monokai => Theme {
                button_fg: col!(0xF8F8F2),
                button_bg: col!(0xF92672),
                active_bg: col!(0xA6E22E),
                focused_bg: col!(0xF8F8F2),
                button_border: col!(0x66D9EF),
                active_fg: col!(0x272822),
                focused_fg: col!(0x272822),
                transparent_bg,
            },
            Themes::OneDark => Theme {
                button_fg: col!(0xABB2BF),
                button_bg: col!(0x61AFEF),
                active_bg: col!(0xE06C75),
                focused_bg: col!(0x98C379),
                button_border: col!(0x56B6C2),
                active_fg: col!(0x282C34),
                focused_fg: col!(0x282C34),
                transparent_bg,
            },
            Themes::Catppuccin => Theme {
                button_fg: col!(0xD9E0EE),
                button_bg: col!(0x89B4FA),
                active_bg: col!(0xF28FAD),
                focused_bg: col!(0xF8F8F2),
                button_border: col!(0xA6E3A1),
                active_fg: col!(0x1E1E2E),
                focused_fg: col!(0x1E1E2E),
                transparent_bg,
            },
            Themes::TokyoNight => Theme {
                button_fg: col!(0xC0CAF5),
                button_bg: col!(0x7AA2F7),
                active_bg: col!(0x9ECE6A),
                focused_bg: col!(0x7AA2F7),
                button_border: col!(0x414868),
                active_fg: col!(0x1A1B26),
                focused_fg: col!(0x1A1B26),
                transparent_bg,
            },
            Themes::AyuDark => Theme {
                button_fg: col!(0xBFBDB6),
                button_bg: col!(0x39BAE6),
                active_bg: col!(0xFFD580),
                focused_bg: col!(0x8FBF8F),
                button_border: col!(0x273B42),
                active_fg: col!(0xF8F8F2),
                focused_fg: col!(0xF8F8F2),
                transparent_bg,
            },
            Themes::TomorrowNight => Theme {
                button_fg: col!(0xC5C8C6),
                button_bg: col!(0x81A2BE),
                active_bg: col!(0xB5BD68),
                focused_bg: col!(0xB5BD68),
                button_border: col!(0x373B41),
                active_fg: col!(0x1D1F21),
                focused_fg: col!(0x1D1F21),
                transparent_bg,
            },
            Themes::Horizon => Theme {
                button_fg: col!(0xE4A8A1),
                button_bg: col!(0xF8CD7F),
                active_bg: col!(0xA9DC76),
                focused_bg: col!(0xF8CD7F),
                button_border: col!(0x9CAACF),
                active_fg: col!(0x1C1E26),
                focused_fg: col!(0x1C1E26),
                transparent_bg,
            },
        };
        return theme;
    }
}
