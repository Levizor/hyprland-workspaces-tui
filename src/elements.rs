use std::process::Stdio;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Position, Rect},
    style::{Color, Style},
    text::ToLine,
    widgets::{Widget, WidgetRef},
};
use serde_json::Value;
use tokio::process::Command;
use tui_big_text::{BigText, PixelSize};

use crate::themes::Theme;

#[derive(Debug)]
pub struct Workspace {
    pub name: String,
    active: bool,
    theme: Theme,
    big_text: bool,
    pub rect: Rect,
    focused: bool,
}

impl Workspace {
    pub fn new(value: Value, theme: Theme, big_text: bool) -> Self {
        let n = value["name"].to_string();
        let size = n.len();
        let name = n[1..size - 1].to_string();
        Workspace {
            name,
            active: value["active"].to_string().parse().unwrap_or(false),
            //id: value["id"].to_string().parse().unwrap_or(-9999),
            theme,
            big_text,
            rect: Rect::new(0, 0, 0, 0),
            focused: false,
        }
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn contains(&self, position: Position) -> bool {
        self.rect.contains(position)
    }

    pub fn activate(&self) -> Result<tokio::process::Child, std::io::Error> {
        Command::new("hyprctl")
            .args(["dispatch", "workspace", &self.name])
            .stdout(Stdio::null())
            .spawn()
    }

    pub fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn get_colors(&self) -> (Color, Color) {
        let mut background = self.theme.button_bg;
        let mut foreground = self.theme.button_fg;

        if self.focused {
            background = self.theme.focused_bg;
            foreground = self.theme.focused_fg;
        }
        if self.active {
            background = self.theme.active_bg;
            foreground = self.theme.active_fg;
        }

        (background, foreground)
    }
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

impl WidgetRef for Workspace {
    #[allow(clippy::cast_possible_truncation)]
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let line = self.name.to_line();

        let (background, foreground) = self.get_colors();

        buf.set_style(area, Style::new().bg(background).fg(foreground));
        if area.height > 2 {
            buf.set_string(
                area.x,
                area.y,
                " ".repeat(area.width as usize),
                Style::new().fg(foreground).bg(background),
            );
        }

        // render bottom line if there's enough space
        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                " ".repeat(area.width as usize),
                Style::new().bg(background),
            );
        }
        match self.big_text {
            true => {
                let text = BigText::builder()
                    .pixel_size(PixelSize::Full)
                    .style(Style::new().fg(foreground))
                    .centered()
                    .lines(vec![line])
                    .build();
                let ar = center(area, Constraint::Length(area.width), Constraint::Length(7));
                text.render(ar, buf);
            }
            false => {
                buf.set_line(
                    area.x + (area.width.saturating_sub(line.width() as u16)) / 2,
                    area.y + (area.height.saturating_sub(1)) / 2,
                    &line,
                    area.width,
                );
            }
        }
    }
}
