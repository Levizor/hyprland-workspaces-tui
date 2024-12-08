use clap::ValueEnum;
use ratatui::style::Color;

macro_rules! col {
    ($hex:expr) => {
        Color::from_u32($hex)
    };
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub text: Color,
    pub button: Color,
    pub active: Color,
    pub urgent: Color,
    pub background: Color,
    pub border: Color,
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
    pub fn get_theme(theme: &Option<Themes>) -> Theme {
        match theme.clone().unwrap_or_default() {
            Themes::Nord => Theme {
                text: col!(0xD6DCE7),
                button: col!(0x81A1C1),
                active: col!(0xEBCB8B),
                urgent: col!(0xBF616A),
                background: col!(0x2E3440),
                border: col!(0x81A1C1),
            },
            Themes::Gruvbox => Theme {
                text: col!(0xEBDBB2),
                button: col!(0xD79921),
                active: col!(0xB57614),
                urgent: col!(0xBF616A),
                background: col!(0x282828),
                border: col!(0x928374),
            },
            Themes::Dracula => Theme {
                text: col!(0xF8F8F2),
                button: col!(0x6272A4),
                active: col!(0x50FA7B),
                urgent: col!(0xFF5555),
                background: col!(0x282A36),
                border: col!(0x44475A),
            },
            Themes::SolarizedDark => Theme {
                text: col!(0x839496),
                button: col!(0x268BD2),
                active: col!(0xB58900),
                urgent: col!(0xDC322F),
                background: col!(0x002B36),
                border: col!(0x073642),
            },
            Themes::SolarizedLight => Theme {
                text: col!(0x657B83),
                button: col!(0x268BD2),
                active: col!(0xB58900),
                urgent: col!(0xDC322F),
                background: col!(0xFDF6E3),
                border: col!(0xEEE8D5),
            },
            Themes::Monokai => Theme {
                text: col!(0xF8F8F2),
                button: col!(0xF92672),
                active: col!(0xA6E22E),
                urgent: col!(0xFD971F),
                background: col!(0x272822),
                border: col!(0x66D9EF),
            },
            Themes::OneDark => Theme {
                text: col!(0xABB2BF),
                button: col!(0x61AFEF),
                active: col!(0xE06C75),
                urgent: col!(0xBE5046),
                background: col!(0x282C34),
                border: col!(0x56B6C2),
            },
            Themes::Catppuccin => Theme {
                text: col!(0xD9E0EE),
                button: col!(0x89B4FA),
                active: col!(0xF28FAD),
                urgent: col!(0xF38BA8),
                background: col!(0x1E1E2E),
                border: col!(0xA6E3A1),
            },
            Themes::TokyoNight => Theme {
                text: col!(0xC0CAF5),
                button: col!(0x7AA2F7),
                active: col!(0x9ECE6A),
                urgent: col!(0xF7768E),
                background: col!(0x1A1B26),
                border: col!(0x414868),
            },
            Themes::AyuDark => Theme {
                text: col!(0xBFBDB6),
                button: col!(0x39BAE6),
                active: col!(0xFFD580),
                urgent: col!(0xF07178),
                background: col!(0x0F1419),
                border: col!(0x273B42),
            },
            Themes::TomorrowNight => Theme {
                text: col!(0xC5C8C6),
                button: col!(0x81A2BE),
                active: col!(0xB5BD68),
                urgent: col!(0xCC6666),
                background: col!(0x1D1F21),
                border: col!(0x373B41),
            },
            Themes::Horizon => Theme {
                text: col!(0xE4A8A1),
                button: col!(0xF8CD7F),
                active: col!(0xA9DC76),
                urgent: col!(0xE95678),
                background: col!(0x1C1E26),
                border: col!(0x9CAACF),
            },
        }
    }
}
