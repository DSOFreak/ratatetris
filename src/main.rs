use crate::lmtetris::Tetris;

pub mod lmtetris;

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Position};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::canvas::{Canvas, Rectangle};
use ratatui::widgets::{Block, Paragraph};

use ratatui::{DefaultTerminal, Frame};

use std::thread::sleep;
use std::time::{Duration, Instant};

fn main2() {
    let mut game = Tetris::new(10, 20);
    game.start();
    let mut i = 0;
    println!("Hello, ratatetris!");
    while game.is_running() {
        game.step();
        println!();
        println!("i = {i}");
        println!();
        game.print();

        if rand::random_bool(0.5) {
            game.move_tet(Some(lmtetris::Direction::Left), None);
        } else {
            game.move_tet(Some(lmtetris::Direction::Right), None);
        }
        i += 1;
        if i > 500 {
            game.stop();
        }
    }
    println!("Game Over!");
}

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::new().run(terminal))
}

/// App holds the state of the application
struct App {
    game: Tetris,
}

impl App {
    fn new() -> Self {
        Self {
            game: Tetris::new(10, 20),
        }
    }

    fn move_left(&mut self) {
        self.game.move_tet(Some(lmtetris::Direction::Left), None);
    }

    fn move_right(&mut self) {
        self.game.move_tet(Some(lmtetris::Direction::Right), None);
    }

    fn rotate_left(&mut self) {
        self.game.move_tet(None, Some(lmtetris::Direction::Left));
    }

    fn rotate_right(&mut self) {
        self.game.move_tet(None, Some(lmtetris::Direction::Left));
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.game.start();
        let tick_rate = Duration::from_millis(1000 / 2);
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if !event::poll(timeout)? {
                terminal.draw(|frame| self.render(frame))?;
                self.game.step();
                last_tick = Instant::now();
                continue;
            }
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Left => {
                        self.move_left();
                    }
                    KeyCode::Right => {
                        self.move_right();
                    }
                    KeyCode::Up => {
                        self.rotate_right();
                    }
                    KeyCode::Char(' ') => {
                        self.game.rush();
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        self.game = Tetris::new(10, 20);
                        self.game.start();
                    }
                    _ => {}
                }
            }
            terminal.draw(|frame| self.render(frame))?;
        }
    }

    fn render(&self, frame: &mut Frame) {
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
        let game_view = Canvas::default()
            .block(Block::bordered().title("LM Tetris"))
            .x_bounds([0.0, (self.game.dimensions().0 as f64)])
            .y_bounds([0.0, (self.game.dimensions().1 as f64)])
            .paint(|c| {
                for x in 0..self.game.dimensions().0 {
                    for y in 0..self.game.dimensions().1 {
                        c.draw(&Rectangle {
                            x: x as f64 + 0.1,
                            y: (self.game.dimensions().1 - y) as f64 + 0.1,
                            width: 0.8,
                            height: 0.8,
                            color: Color::Indexed(self.game.tile_color(x, y).c),
                        });
                    }
                }
                let tet_preview = self.game.get_tetromino_preview();
                for p in tet_preview.points() {
                    let x = p.x;
                    let y = p.y;
                    c.draw(&Rectangle {
                        x: x as f64 + 0.1,
                        y: (self.game.dimensions().1 - y) as f64 + 0.1,
                        width: 0.8,
                        height: 0.8,
                        color: Color::Indexed(tet_preview.color.c),
                    })
                }
                let tet = self.game.get_tetromino();
                for p in tet.points() {
                    let x = p.x;
                    let y = p.y;
                    c.draw(&Rectangle {
                        x: x as f64 + 0.1,
                        y: (self.game.dimensions().1 - y) as f64 + 0.1,
                        width: 0.8,
                        height: 0.8,
                        color: Color::Indexed(tet.color.c),
                    })
                }
            });
        frame.render_widget(game_view, game_area);
    }
}
