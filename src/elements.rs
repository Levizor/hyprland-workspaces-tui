use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, ToLine},
    widgets::WidgetRef,
};
use serde_json::Value;

use crate::{app::App, themes::Theme};

#[derive(Debug)]
pub struct Workspace {
    pub name: String,
    id: i32,
    active: bool,
    theme: Theme,
}

impl Workspace {
    pub fn new(value: Value, theme: Theme) -> Self {
        let n = value["name"].to_string();
        let size = n.len();
        let name = n[1..size - 1].to_string();
        Workspace {
            name,
            active: value["active"].to_string().parse().unwrap_or(false),
            id: value["id"].to_string().parse().unwrap_or(-9999),
            theme,
        }
    }
}

impl WidgetRef for Workspace {
    #[allow(clippy::cast_possible_truncation)]
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let button_col = self
            .active
            .then_some(self.theme.active)
            .unwrap_or(self.theme.button);
        let line = self.name.to_line();

        buf.set_style(area, Style::new().bg(button_col).fg(self.theme.text));
        if area.height > 2 {
            buf.set_string(
                area.x,
                area.y,
                " ".repeat(area.width as usize),
                Style::new().fg(self.theme.text).bg(button_col),
            );
        }

        // render bottom line if there's enough space
        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                " ".repeat(area.width as usize),
                Style::new().bg(button_col),
            );
        }
        buf.set_line(
            area.x + (area.width.saturating_sub(line.width() as u16)) / 2,
            area.y + (area.height.saturating_sub(1)) / 2,
            &line,
            area.width,
        );
    }
}
