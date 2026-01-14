use color_eyre::owo_colors::{OwoColorize, style};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Margin, Position, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::canvas::{Canvas, Rectangle, Shape};
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::{Frame, symbols};

use crate::lmtetris::{self, Tetris};

pub struct Ui {}

impl Ui {
    pub fn render(game: &lmtetris::Tetris, frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Min(1)]);
        let [header, game_area] = frame.area().layout(&layout);

        let (msg, style) = (
            vec![
                "Press ".into(),
                "q".bold(),
                " to exit, ".into(),
                "e".bold(),
                " to start editing.".bold(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        );
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, header);
        frame.render_widget(game, game_area.inner(Margin::default()));
    }
}

impl Widget for &Tetris {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        buf.set_style(area, Style::default().bg(Color::Indexed(16)));

        let elem_size = std::cmp::min(area.width, area.height) / self.dimensions().0 as u16 / 2;
        for x in 0..self.dimensions().0 as u16 {
            for y in 1..=self.dimensions().1 as u16 {
                let y = self.dimensions().1 as u16 - y;
                let xd = x * elem_size * 2;
                let yd = y * elem_size;
                buf.set_style(
                    Rect::new(xd, yd, elem_size * 2, elem_size),
                    Style::default().bg(Color::Indexed(self.tile_color(x as i32, y as i32).c)),
                );
            }
        }

        let tet = self.get_tetromino();
        let tet_ghost = self.get_tetromino_preview();
        for (i, p) in tet.points().iter().chain(&tet_ghost.points()).enumerate() {
            let x = p.x as u16;
            let y = p.y as u16;
            let xd = x * elem_size * 2;
            let yd = y * elem_size;
            let mut style = Style::default().bg(Color::Indexed(tet.color.c));
            if i > 3 {
                style = Style::default().bg(Color::Indexed(234));
            }
            buf.set_style(Rect::new(xd, yd, elem_size * 2, elem_size), style);
        }
    }
}
