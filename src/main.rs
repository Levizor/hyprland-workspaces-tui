use clap::Parser;
use simplelog::{CombinedLogger, Config as Conf, LevelFilter, WriteLogger};
use std::io;
use std::{fs::File, time::Duration};
use tokio::time::sleep;

use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::{App, AppResult},
    config::Config,
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

pub mod app;
pub mod config;
pub mod elements;
pub mod event;
pub mod handler;
pub mod themes;
pub mod tui;
pub mod ui;

#[tokio::main]
async fn main() -> AppResult<()> {
    let conf = Config::parse();
    if let Some(d) = conf.debug {
        d.then(|| {
            CombinedLogger::init(vec![WriteLogger::new(
                LevelFilter::Debug,
                Conf::default(),
                File::create("app.log").unwrap(),
            )])
            .unwrap();
        });
    }

    let mut app = App::new(conf);

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
                if let Ok(state) = app.get_state(line){
                    let _ = app.update(state);
                }
            }

            Ok(event) = tui.events.next() => {
                match event {
                    Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
                    Event::Mouse(_) => {}
                    Event::Resize(_, _) => {}
                }
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    println!("hey");
    Ok(())
}
