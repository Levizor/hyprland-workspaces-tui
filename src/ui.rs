use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::ToLine,
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use tui_big_text::{BigText, PixelSize};

use crate::{app::App, elements::Workspace};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    render_workspaces(frame, frame.area(), &app.workspaces);
}

pub fn render_workspaces(frame: &mut Frame, area: Rect, workspaces: &Vec<Workspace>) {
    let horizontal = Layout::horizontal(
        workspaces
            .iter()
            .map(|ws| Constraint::Length(area.width / workspaces.len() as u16)),
    );

    workspaces
        .iter()
        .zip(horizontal.split(area).into_iter())
        .for_each(|(ws, ar)| frame.render_widget(ws, *ar));
}
