use crate::themes::Themes;
use clap::Parser;
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Config {
    /// Monitor name passed to hyprland-workspaces. ALL by default.
    #[arg(short, long, value_name = "MONITOR", default_value = "ALL")]
    pub monitor: String,

    /// Sets a theme
    #[arg(short, long, value_name = "THEME_NAME")]
    pub theme: Option<Themes>,

    /// Prints debug information to app.log file
    #[arg(short, long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub debug: bool,

    /// Show special workspaces
    #[arg(short, long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub show_special: bool,

    /// Use big text
    #[arg(short, long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub big_text: bool,

    /// Generates completion scripts for the specified shell
    #[arg(long, value_name = "SHELL", value_enum)]
    pub completions: Option<Shell>,
}
