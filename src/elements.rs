use std::process::Stdio;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Position, Rect},
    style::{Color, Style},
    text::ToLine,
    widgets::{Block, Borders, Widget, WidgetRef},
};
use serde_json::Value;
use tokio::process::Command;
use tui_big_text::{BigText, PixelSize};

use crate::config::Colors;

#[derive(Debug)]
pub struct Workspace {
    pub name: String,
    active: bool,
    colors: Colors,
    borders: bool,
    big_text: bool,
    pub rect: Rect,
    focused: bool,
}

impl Workspace {
    pub fn new(value: Value, colors: Colors, big_text: bool, borders: bool) -> Self {
        let n = value["name"].to_string();
        let size = n.len();
        let name = n[1..size - 1].to_string();
        Workspace {
            name,
            active: value["active"].to_string().parse().unwrap_or(false),
            //I don't think we need this, but let it be here for a while: id: value["id"].to_string().parse().unwrap_or(-9999),
            colors,
            big_text,
            rect: Rect::new(0, 0, 0, 0),
            focused: false,
            borders,
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

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }

    fn get_border_color(&self) -> Color {
        let colors = &self.colors;
        let mut color = colors.border;
        if self.focused {
            color = colors.border_focused;
        }
        if self.active {
            color = colors.border_active;
        }
        color
    }

    fn get_colors(&self) -> (Color, Color) {
        let mut background = self.colors.bg;
        let mut foreground = self.colors.fg;

        if self.focused {
            background = self.colors.bg_focused;
            foreground = self.colors.fg_focused;
        }
        if self.active {
            background = self.colors.bg_active;
            foreground = self.colors.fg_active;
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

        'a: {
            if self.borders {
                let color = self.get_border_color();
                if color == Color::Reset {
                    break 'a;
                }
                log::info!("Border color: {}", color);
                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(color));
                block.render(area, buf);
            }
        }
    }
}
