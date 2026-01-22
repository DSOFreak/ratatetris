use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use ratatui::Frame;

use crate::lmtetris::{self, Tetris};

pub struct Ui {}

impl Ui {
    pub fn render(game: &lmtetris::Tetris, frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Length(5), Constraint::Min(20)]);
        let [header, game_area] = frame.area().layout(&layout);

        let (msg, style) = (
            vec![
                "  ⬆   : rotate | '␣' : slam     | 'p' : pause \n".into(),
                "⬅   ➡ : move   | '↵' : new game |\n".into(),
                "  ⬇   : skip   | 'q' : exit     |\n".into(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        );
        let text = Text::from(msg).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, header);

        // Split game area into main board and stats panel
        let game_layout = Layout::horizontal([Constraint::Min(20), Constraint::Length(20)]);
        let [board_area, stats_area] = game_area.layout(&game_layout);

        frame.render_widget(game, board_area.inner(Margin::default()));

        // Render stats panel
        let stats_text = format!(
            "\n Level: {}\n\n Lines: {}\n\n Score: {}",
            game.level(),
            game.lines_cleared(),
            game.score()
        );
        let stats = Paragraph::new(stats_text)
            .block(Block::default().borders(Borders::ALL).title(" Stats "))
            .style(Style::default().fg(Color::White));
        frame.render_widget(stats, stats_area);
    }
}

impl Widget for &Tetris {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        buf.set_style(area, Style::default().bg(Color::Indexed(16)));

        let elem_size = std::cmp::min(area.width, area.height) / self.dimensions().0 as u16 / 2;
        let w = elem_size * 2 * self.dimensions().0 as u16;
        let h = elem_size * self.dimensions().1 as u16;

        let xoff: u16 = (area.width - w) / 2;
        let yoff: u16 = (area.height - h) / 2;

        for x in 0..self.dimensions().0 as u16 {
            for y in 1..=self.dimensions().1 as u16 {
                let y = self.dimensions().1 as u16 - y;
                let xd = x * elem_size * 2 + xoff;
                let yd = y * elem_size + yoff + area.y;
                buf.set_style(
                    Rect::new(xd, yd, elem_size * 2, elem_size),
                    Style::default().bg(Color::Indexed(self.tile_color(x as i32, y as i32).c)),
                );
            }
        }

        let tet = self.get_tetromino();
        let tet_ghost = &self.get_tetromino_preview();
        for (i, p) in tet_ghost.points().iter().chain(&tet.points()).enumerate() {
            let x = p.x as u16;
            let y = p.y as u16;
            let xd = x * elem_size * 2 + xoff;
            let yd = y * elem_size + yoff + area.y;
            let mut style = Style::default().bg(Color::Indexed(tet.color.c));
            if i <= 3 {
                style = Style::default().bg(Color::Indexed(234));
            }
            buf.set_style(Rect::new(xd, yd, elem_size * 2, elem_size), style);
        }
    }
}
