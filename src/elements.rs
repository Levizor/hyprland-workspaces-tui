use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, ToLine},
    widgets::Widget,
};
use serde_json::Value;

use crate::{app::App, themes::Theme};

#[derive(Debug)]
pub struct Workspace<'a> {
    name: String,
    id: i32,
    active: bool,
    theme: &'a Theme,
}

impl<'a> Workspace<'a> {
    pub fn new(value: Value, theme: &'a Theme) -> Self {
        Workspace {
            name: value["name"].to_string(),
            active: value["active"].to_string().parse().unwrap_or(false),
            id: value["id"].to_string().parse().unwrap_or(-9999),
            theme,
        }
    }
}

impl<'a> Widget for Workspace<'a> {
    #[allow(clippy::cast_possible_truncation)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, Style::new().bg(self.theme.button).fg(self.theme.text));
        let line = self.name.to_line();
        buf.set_line(
            area.x + (area.width.saturating_sub(line.width() as u16)) / 2,
            area.y + (area.height.saturating_sub(1)) / 2,
            &line,
            area.width,
        );
    }
}
