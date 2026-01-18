use crate::lmtetris::Tetris;
use crate::lmtetris_sound::Sound;

pub mod lmtetris;
pub mod lmtetris_sound;
pub mod lmtetris_ui;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{DefaultTerminal, Frame};

use std::time::{Duration, Instant};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::new().run(terminal))
}

/// App holds the state of the application
struct App {
    game: Tetris,
    sound: Sound,
}

impl App {
    fn new() -> Self {
        Self {
            game: Tetris::new(10, 20),
            sound: Sound::new(44100),
        }
    }

    fn move_left(&mut self) {
        self.game.move_tet(Some(lmtetris::Direction::Left), None);
    }

    fn move_right(&mut self) {
        self.game.move_tet(Some(lmtetris::Direction::Right), None);
    }

    fn rotate_right(&mut self) {
        self.game.move_tet(None, Some(lmtetris::Direction::Left));
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.game.start();
        self.sound.start();
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
                        self.sound.tmove();
                    }
                    KeyCode::Right => {
                        self.move_right();
                        self.sound.tmove();
                    }
                    KeyCode::Up => {
                        self.rotate_right();
                        self.sound.swirl();
                    }
                    KeyCode::Down => {
                        self.game.step();
                        self.sound.combo();
                    }
                    KeyCode::Char(' ') => {
                        self.game.rush();
                        self.sound.smash();
                    }
                    KeyCode::Char('p') => {
                        if self.game.is_running() {
                            self.game.pause();
                            self.sound.pause();
                        } else {
                            self.game.start();
                            self.sound.start();
                        }
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        self.game = Tetris::new(10, 20);
                        self.game.start();
                        self.sound.start();
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
