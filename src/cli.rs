use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Cli {
    /// Monitor name passed to hyprland-workspaces. ALL by default.
    #[arg(short, long, value_name = "MONITOR", default_value = "ALL")]
    pub monitor: String,

    #[arg(short, long, value_name = "CONFIG_PATH")]
    pub config_path: Option<String>,

    /// Show special workspaces
    #[arg(short, long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub show_special: bool,

    /// Use big text
    #[arg(short, long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub big_text: bool,

    /// Generates completion scripts for the specified shell
    #[arg(long, value_name = "SHELL", value_enum)]
    pub completions: Option<Shell>,

    /// Places workspaces vertically
    #[arg(short, long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub vertical: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Prints debug information to app.log file
    #[arg(short, long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Just print workspaces to stdout
    Plain {
        /// Separator between workspaces
        #[arg(short, long, default_value_t = String::from(" "))]
        separator: String,

        /// String to add around active workspacse
        #[arg(short, long, default_value_t = String::from("|"))]
        active: String,

        /// Use carriage return to override update line
        #[arg(short, long, action = clap::ArgAction::SetTrue, default_value_t = false)]
        carriage_return: bool,

        #[arg(short, long, action = clap::ArgAction::SetTrue, default_value_t = false)]
        print_once: bool,
    },
}
