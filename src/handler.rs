use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent};
use ratatui::layout::Position;

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit().await;
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit().await;
            }
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

fn get_pos(mouse_event: MouseEvent) -> Position {
    Position::new(mouse_event.column, mouse_event.row)
}

fn handle_click(mouse_event: MouseEvent, app: &mut App) {
    if let Some(ws) = app.find_ws_mut_by_mouse_pos(get_pos(mouse_event)) {
        if let Err(e) = ws.activate() {
            log::error!("{}", e);
        }
    }
}

fn handle_move(mouse_event: MouseEvent, app: &mut App) {
    app.workspaces.iter_mut().for_each(|ws| ws.set_focus(false));

    if let Some(ws) = app.find_ws_mut_by_mouse_pos(get_pos(mouse_event)) {
        ws.set_focus(true);
    }
}

pub fn handle_mouse_event(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    match mouse_event.kind {
        crossterm::event::MouseEventKind::Down(MouseButton::Left) => handle_click(mouse_event, app),
        crossterm::event::MouseEventKind::Down(MouseButton::Right) => {}
        crossterm::event::MouseEventKind::Down(MouseButton::Middle) => {}
        crossterm::event::MouseEventKind::Moved => handle_move(mouse_event, app),
        _ => {}
    }
    Ok(())
}
