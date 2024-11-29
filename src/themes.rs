use clap::ValueEnum;
use ratatui::style::{Color, Style};

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

#[derive(ValueEnum, Debug, Clone)]
pub enum Themes {
    Nord,
    Gruvbox,
}

impl Themes {
    pub const fn get_theme(theme: Themes) -> Theme {
        match theme {
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
        }
    }
}
