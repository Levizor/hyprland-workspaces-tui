use crate::{
    config::Config,
    elements::Workspace,
    themes::{Theme, Themes},
};
use ratatui::layout::Position;
use serde_json::Value;
use std::{
    error::{self},
    process::{exit, Stdio},
};
use tokio::process::{ChildStdout, Command};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Child,
};

pub fn get_state(state: String) -> Result<Value, ()> {
    let json = serde_json::from_str(&state);
    match json {
        Ok(result) => Ok(result),
        Err(e) => {
            log::error!("Error: {}", e);
            Err(())
        }
    }
}
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub stream: tokio::io::Lines<BufReader<ChildStdout>>,
    pub workspaces: Vec<Workspace>,
    pub child: Child,
    pub config: Config,
    pub theme: Theme,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(config: Config) -> Self {
        let (stream, child) = App::init_reader(&config.monitor);
        let theme = Themes::get_theme(&config.theme, config.transparent);
        Self {
            running: true,
            stream,
            workspaces: Vec::new(),
            child,
            theme,
            config,
        }
    }

    fn update(&mut self, state: Value) -> Result<(), ()> {
        if let Some(workspaces) = state.as_array() {
            self.workspaces.clear();
            workspaces
                .into_iter()
                .filter(|ws| {
                    ws["name"]
                        .to_string()
                        .starts_with("\"special")
                        .then(|| self.config.show_special)
                        .unwrap_or(true)
                })
                .map(|ws| Workspace::new(ws.clone(), self.theme.clone(), self.config.big_text))
                .for_each(|ws| self.workspaces.push(ws));
            return Ok(());
        }
        Err(())
    }

    pub fn feed(&mut self, line: String) {
        if let Ok(state) = get_state(line) {
            let _ = self.update(state);
        }
    }

    fn init_reader(
        monitor: &String,
    ) -> (
        tokio::io::Lines<BufReader<ChildStdout>>,
        tokio::process::Child,
    ) {
        let child = Command::new("hyprland-workspaces")
            .arg(monitor)
            .stdout(Stdio::piped())
            .spawn();
        let Ok(mut child) = child else {
            eprintln!("Couldn't run hyprland-workspaces");
            exit(1);
        };

        let stdout = child
            .stdout
            .take()
            .expect("Couldn't take stdout of the hyprland-workspaces");

        let reader = BufReader::new(stdout);
        (reader.lines(), child)
    }

    pub fn find_ws_by_mouse_pos(&self, pos: Position) -> Option<&Workspace> {
        self.workspaces.iter().find(|ws| ws.contains(pos))
    }

    pub fn find_ws_mut_by_mouse_pos(&mut self, pos: Position) -> Option<&mut Workspace> {
        self.workspaces.iter_mut().find(|ws| ws.contains(pos))
    }

    pub async fn close_reader(&mut self) {
        let _ = self.child.kill().await;
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
