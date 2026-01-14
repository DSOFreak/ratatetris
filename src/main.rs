use crate::lmtetris::Tetris;

pub mod lmtetris;
pub mod lmtetris_ui;

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use std::thread::sleep;
use std::time::{Duration, Instant};

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
        lmtetris_ui::Ui::render(&self.game, frame);
    }
}
