use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::ToLine,
    Frame,
};

use crate::{app::App, elements::Workspace};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    render_workspaces(frame, frame.area(), &mut app.workspaces);
}

pub fn render_workspaces(frame: &mut Frame, area: Rect, workspaces: &mut Vec<Workspace>) {
    let horizontal = Layout::horizontal(workspaces.iter().map(|ws| {
        Constraint::Length(
            (ws.name.to_line().width() + area.width as usize / workspaces.len()) as u16,
        )
    }));

    workspaces
        .iter_mut()
        .zip(horizontal.split(area).into_iter())
        .for_each(|(ws, ar)| {
            ws.set_rect(*ar);
            frame.render_widget(&*ws, *ar);
        });
}
