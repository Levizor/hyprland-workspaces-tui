use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

use crate::{app::App, config::Allignment, elements::Workspace};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let workspaces = &mut app.workspaces;
    let layout = match app.config.allignment {
        Allignment::Vertical => {
            Layout::vertical(vec![Constraint::Fill(1); workspaces.len()].iter())
        }
        Allignment::Horizontal => {
            Layout::horizontal(vec![Constraint::Fill(1); workspaces.len()].iter())
        }
    };

    render_workspaces(frame, frame.area(), &mut app.workspaces, layout);
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
