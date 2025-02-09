use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::ToLine,
    Frame,
};

use crate::{app::App, elements::Workspace};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let workspaces = &mut app.workspaces;
    let layout = app
        .config
        .vertical
        .then(|| {
            Layout::vertical(workspaces.iter().map(|ws| {
                Constraint::Length(
                    (ws.name.to_line().width() + frame.area().height as usize / workspaces.len())
                        as u16,
                )
            }))
        })
        .or_else(|| {
            Some(Layout::horizontal(workspaces.iter().map(|ws| {
                Constraint::Length(
                    (ws.name.to_line().width() + frame.area().width as usize / workspaces.len())
                        as u16,
                )
            })))
        });
    render_workspaces(
        frame,
        frame.area(),
        &mut app.workspaces,
        layout.expect("this should be fine"),
    );
}

pub fn render_workspaces(
    frame: &mut Frame,
    area: Rect,
    workspaces: &mut Vec<Workspace>,
    layout: Layout,
) {
    workspaces
        .iter_mut()
        .zip(layout.split(area).into_iter())
        .for_each(|(ws, ar)| {
            ws.set_rect(*ar);
            frame.render_widget(&*ws, *ar);
        });
}
