use ::config;
use dirs::config_dir;
use ratatui::style::Color;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

fn get_default_config_path() -> Result<PathBuf, Box<dyn Error>> {
    match dirs::config_dir() {
        Some(conf_dir) => Ok(conf_dir.join("hyprland-workspaces-tui/config.toml")),
        None => Err(Box::<dyn Error>::from(
            "Couldn't determine default config directory.",
        )),
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "colors")]
    pub colors: Colors,

    #[serde(default = "plain_text_mode")]
    pub plain_text_mode: PlainTextMode,
}

fn colors() -> Colors {
    Colors::default()
}

fn plain_text_mode() -> PlainTextMode {
    PlainTextMode::default()
}

impl Config {
    pub fn new(path: &Option<String>) -> Result<Self, Box<dyn Error>> {
        let builder = config::Config::builder();
        let builder = match path {
            Some(path) => builder.add_source(
                config::File::from(PathBuf::from_str(&path).expect("Infallible"))
                    .format(config::FileFormat::Toml),
            ),
            None => {
                let path = get_default_config_path()?;
                if !path.exists() {
                    log::info!("Config file not found. Using default configuration.");
                    return Ok(Self::default());
                }
                builder.add_source(
                    config::File::from(get_default_config_path().expect("Infallible"))
                        .format(config::FileFormat::Toml),
                )
            }
        };

        let config = builder.build()?.try_deserialize::<Config>()?;
        Ok(config)
    }

    //pub fn parse(path: &Option<String>) -> Result<Config, Box<dyn Error>> {
    //    let path = match path {
    //        Some(path) => PathBuf::from_str(&path).expect("Infallible"),
    //        None => get_default_config_path()?,
    //    };
    //
    //    let text = fs::read_to_string(path)?;
    //    toml::from_str(&text).map_err(|e| Box::new(e) as Box<dyn Error>)
    //}
}

impl Default for Config {
    fn default() -> Self {
        Self {
            colors: Colors::default(),
            plain_text_mode: PlainTextMode::default(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Colors {
    #[serde(default = "black")]
    pub bg: Color,

    #[serde(default = "white")]
    pub fg: Color,

    #[serde(default = "darkgray")]
    pub bg_active: Color,

    #[serde(default = "magenta")]
    pub fg_active: Color,

    #[serde(default = "blue")]
    pub bg_focused: Color,

    #[serde(default = "white")]
    pub fg_focused: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            bg: black(),
            fg: white(),
            bg_active: blue(),
            fg_active: black(),
            bg_focused: darkgray(),
            fg_focused: white(),
        }
    }
}

const fn black() -> Color {
    Color::Black
}

const fn white() -> Color {
    Color::White
}

const fn darkgray() -> Color {
    Color::DarkGray
}

const fn magenta() -> Color {
    Color::Magenta
}

const fn blue() -> Color {
    Color::Blue
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlainTextMode {
    #[serde(default = "separator")]
    pub separator: String,

    #[serde(default = "separator_active")]
    pub separator_active: String,

    #[serde(default = "print_once")]
    pub print_once: bool,

    #[serde(default = "carriage_return")]
    pub carriage_return: bool,
}

impl Default for PlainTextMode {
    fn default() -> Self {
        Self {
            separator: separator(),
            separator_active: separator_active(),
            print_once: carriage_return(),
            carriage_return: carriage_return(),
        }
    }
}

fn separator() -> String {
    String::from(" ")
}

fn separator_active() -> String {
    String::from("|")
}

fn carriage_return() -> bool {
    true
}

fn print_once() -> bool {
    false
}
