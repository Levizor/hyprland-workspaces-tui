use crate::app::{App, AppResult};
use crossterm::{
    cursor::MoveDown,
    event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent},
};
use ratatui::layout::Position;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

pub fn handle_mouse_event(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    match mouse_event.kind {
        crossterm::event::MouseEventKind::Down(MouseButton::Left) => {
            let ws = app
                .workspaces
                .iter()
                .find(|ws| ws.contains(Position::new(mouse_event.column, mouse_event.row)));
            match ws {
                Some(ws) => {
                    log::info!("Found ws with name {}", ws.name);
                    if let Err(e) = ws.activate() {
                        log::error!("{}", e);
                    }
                }
                None => {
                    log::info!("Couldn't found the ws");
                }
            }
        }

        crossterm::event::MouseEventKind::Down(MouseButton::Right) => {}
        crossterm::event::MouseEventKind::Down(MouseButton::Middle) => {}
        _ => {}
    }
    Ok(())
}
