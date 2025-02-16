use clap::{CommandFactory, Parser};
use clap_complete::generate;
use cli::Commands;
use config::Config;
use handler::handle_mouse_event;
use simplelog::{CombinedLogger, Config as Conf, LevelFilter, WriteLogger};
use std::fs::File;
use std::io;

use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::{App, AppResult},
    cli::Cli,
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

pub mod app;
pub mod cli;
pub mod config;
pub mod elements;
pub mod event;
pub mod handler;
pub mod plain_text_mode;
pub mod tui;
pub mod ui;

#[tokio::main]
async fn main() -> AppResult<()> {
    let cli = Cli::parse();
    let mut config = Config::new(&cli.config_path)?;

    if let Some(shell) = cli.completions {
        let mut cmd = Cli::command();
        let mut out = io::stdout();
        generate(shell, &mut cmd, "hyprland-workspaces-tui", &mut out);
        return Ok(());
    }

    if cli.debug {
        CombinedLogger::init(vec![WriteLogger::new(
            LevelFilter::Debug,
            Conf::default(),
            File::create("app.log").unwrap(),
        )])
        .unwrap();
    }

    config.merge_with_cli(&cli);

    let mut app = App::new(config);

    if let Some(Commands::Plain { .. }) = cli.command {
        log::info!("Entering plain text mode");
        return plain_text_mode::main(&mut app).await;
    }

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new();
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        tui.draw(&mut app)?;
        tokio::select! {
            Ok(Some(line)) = app.stream.next_line() => {
                app.feed(line);
            }

            Ok(event) = tui.events.next() => {
                match event {
                    Event::Key(key_event) => handle_key_events(key_event, &mut app).await?,
                    Event::Mouse(mouse_event) => {
                        handle_mouse_event(mouse_event, &mut app)?
                    },
                    Event::Resize(_, _) => {tui.draw(&mut app).unwrap()}
                }
            }
        }
    }

    tui.exit()?;
    Ok(())
}
