use crate::{elements::Workspace, themes::Theme};
use serde_json::Value;
use std::{
    error,
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
    pub state: Value,
    pub stream: tokio::io::Lines<BufReader<ChildStdout>>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(theme: Theme) -> Self {
        Self {
            running: true,
            state: Value::Null,
            stream: App::init_reader(),
            theme,
        }
    }

    pub fn set_state(&mut self, state: String) -> Result<(), ()> {
        let json = serde_json::from_str(&state);
        match json {
            Ok(result) => {
                self.state = result;
                Ok(())
            }
            Err(e) => {
                log::error!("Error: {}", e);
                Err(())
            }
        }
    }

    pub fn update(&mut self) -> Result<(), ()> {
        if let Some(workspaces) = self.state.as_array() {
            let iter = workspaces.iter().map(|ws| Workspace::new);
        }

        Ok(())
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
