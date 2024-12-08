use crate::themes::Themes;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Config {
    /// Sets a theme
    #[arg(short, long, value_name = "THEME_NAME")]
    pub theme: Option<Themes>,
    /// Prints debug information to app.log file
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub debug: Option<bool>,

    /// Show special workspaces
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub show_special: Option<bool>,

    /// Use big text
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub big_text: Option<bool>,
}
