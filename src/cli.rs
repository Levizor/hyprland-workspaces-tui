use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Cli {
    /// Monitor name passed to hyprland-workspaces. ALL by default.
    #[arg(short, long, value_name = "MONITOR")]
    pub monitor: Option<String>,

    #[arg(short, long, value_name = "CONFIG_PATH")]
    pub config_path: Option<String>,

    /// Show special workspaces
    #[arg(short, long)]
    pub show_special: Option<bool>,

    /// Use big text
    #[arg(short, long)]
    pub big_text: Option<bool>,

    /// Generates completion scripts for the specified shell
    #[arg(long, value_name = "SHELL", value_enum)]
    pub completions: Option<Shell>,

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
        /// Use carriage return to override update line
        #[arg(short, long)]
        carriage_return: Option<bool>,

        #[arg(short, long)]
        print_once: Option<bool>,
    },
}
