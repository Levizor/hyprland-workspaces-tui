use crate::{
    config::Config,
    elements::Workspace,
    themes::{Theme, Themes},
};
use serde_json::Value;
use std::{
    error::{self, Error},
    process::{exit, Stdio},
    vec,
};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, ChildStdout, Command};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub theme: Theme,
    pub stream: tokio::io::Lines<BufReader<ChildStdout>>,
    pub workspaces: Vec<Workspace>,
    show_special: bool,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(config: Config) -> Self {
        Self {
            running: true,
            stream: App::init_reader(),
            theme: Themes::get_theme(config.theme.clone()),
            workspaces: Vec::new(),
            show_special: config.show_special.unwrap_or_default(),
        }
    }

    pub fn get_state(&mut self, state: String) -> Result<Value, ()> {
        let json = serde_json::from_str(&state);
        match json {
            Ok(result) => Ok(result),
            Err(e) => {
                log::error!("Error: {}", e);
                Err(())
            }
        }
    }

    pub fn update(&mut self, state: Value) -> Result<(), ()> {
        if let Some(workspaces) = state.as_array() {
            self.workspaces.clear();
            workspaces
                .into_iter()
                .filter(|ws| {
                    ws["name"]
                        .to_string()
                        .starts_with("\"special")
                        .then(|| self.show_special)
                        .unwrap_or(true)
                })
                .map(|ws| Workspace::new(ws.clone(), self.theme.clone()))
                .for_each(|ws| self.workspaces.push(ws));
            return Ok(());
        }
        Err(())
    }

    fn init_reader() -> tokio::io::Lines<BufReader<ChildStdout>> {
        let cmd = Command::new("hyprland-workspaces")
            .arg("ALL")
            .stdout(Stdio::piped())
            .spawn();
        let Ok(mut cmd) = cmd else {
            eprintln!("Couldn't run hyprland-workspaces");
            exit(1);
        };
        let stdout = cmd
            .stdout
            .take()
            .expect("Couldn't take stdout of the hyprland-workspaces");
        BufReader::new(stdout).lines()
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
