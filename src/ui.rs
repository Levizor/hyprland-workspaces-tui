use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::ToLine,
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::{app::App, elements::Workspace};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    render_workspaces(frame, frame.area(), &app.workspaces);
}

pub fn render_workspaces(frame: &mut Frame, area: Rect, workspaces: &Vec<Workspace>) {
    let horizontal = Layout::horizontal(
        workspaces
            .iter()
            .map(|ws| Constraint::Length(ws.name.to_line().width() as u16 + 10)),
    );

    workspaces
        .iter()
        .zip(horizontal.split(area).into_iter())
        .for_each(|(ws, ar)| frame.render_widget(ws, *ar));
}
