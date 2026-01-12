use ratatui::{Terminal, style::Color};
use std::cmp::Eq;
use std::ops::Add;

const TETS: [[Point; 4]; 7] = [I, J, L, O, S, T, Z];

const I: [Point; 4] = [
    Point { x: -1, y: 0 },
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 2, y: 0 },
];
const J: [Point; 4] = [
    Point { x: -1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
];
const L: [Point; 4] = [
    Point { x: -1, y: 0 },
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: 1 },
];
const O: [Point; 4] = [
    Point { x: 0, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: 1 },
];
const S: [Point; 4] = [
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
];
const T: [Point; 4] = [
    Point { x: -1, y: 0 },
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
];
const Z: [Point; 4] = [
    Point { x: -1, y: 0 },
    Point { x: 0, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
];

#[derive(Clone, Debug)]
struct Tile {
    filled: bool,
    color: Color,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Point;
    fn add(self, other: (i32, i32)) -> Point {
        Point {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

#[derive(Default, Clone)]
struct Tetromino {
    offset: [Point; 4],
    center: Point,
    color: Color,
}

impl Tetromino {
    fn random(x: i32, y: i32) -> Self {
        Self {
            offset: TETS[rand::random_range(0..TETS.len())],
            center: Point { x, y },
            color: Color::Indexed(rand::random::<u8>()),
        }
    }
    fn fall(&mut self) -> &Self {
        for p in &mut self.offset {
            *p = *p + (0, 1);
        }
        self
    }
    fn collides(&self, board: &Board) -> bool {
        for p in &self.offset {
            if board.get_tile(*p + self.center).filled {
                return true;
            }
            if p.y >= board.height - 1 {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct Board {
    tiles: Vec<Tile>,
    width: i32,
    height: i32,
}

#[derive(PartialEq)]
enum GameState {
    Running,
    Paused,
    GameOver,
}

struct Game {
    state: GameState,
    board: Board,
    tetromino: Tetromino,
}

impl Board {
    fn new(width: i32, height: i32) -> Self {
        assert!(width >= 4);
        assert!(height >= 4);
        Board {
            tiles: vec![
                Tile {
                    filled: false,
                    color: Color::Black
                };
                (width * height) as usize
            ],
            width,
            height,
        }
    }
    fn p2i(&self, p: &Point) -> usize {
        assert!(p.x >= 0);
        assert!(p.y >= 0);
        assert!(p.x < self.width);
        assert!(p.y < self.height);
        (p.x + p.y * self.width) as usize
    }
    fn get_tile(&self, p: Point) -> &Tile {
        &self.tiles[self.p2i(&p)]
    }
    fn fill_tetromino_check_gameover(&mut self, t: &Tetromino) -> bool {
        for p in t.offset {
            let i = self.p2i(&(p + t.center));
            let t = &mut self.tiles[i];
            if t.filled {
                return true;
            } else {
                t.filled = true;
            }
        }
        false
    }
}

impl Game {
    fn new(width: i32, height: i32) -> Self {
        Game {
            state: GameState::Paused,
            board: Board::new(width, height),
            tetromino: Tetromino::random(width / 2, 0),
        }
    }
    fn start(&mut self) {
        self.state = GameState::Running
    }
    fn stop(&mut self) {
        self.state = GameState::Paused
    }
    fn is_running(&self) -> bool {
        self.state == GameState::Running
    }
    fn is_gameover(&self) -> bool {
        self.state == GameState::GameOver
    }
    fn step(&mut self) {
        if self.state != GameState::Running {
            return;
        }
        let prev = self.tetromino.clone();
        self.tetromino.fall();
        if self.tetromino.collides(&self.board) {
            if self.board.fill_tetromino_check_gameover(&prev) {
                self.state = GameState::GameOver;
            }
            self.tetromino = Tetromino::random(self.board.width / 2, 0);
        }
    }
    fn draw(&self) {
        for j in 0..self.board.height {
            for i in 0..self.board.width {
                let p = Point { x: i, y: j };
                let mut c = ' ';
                let t = &self.board.get_tile(p);
                if t.filled {
                    c = 'x';
                } else {
                    for mut tp in self.tetromino.offset {
                        tp = tp + self.tetromino.center;
                        if tp == p {
                            c = 'o';
                            break;
                        }
                    }
                }
                print!("{c}");
            }
            println!();
        }
    }
}

fn main() {
    let mut game = Game::new(10, 20);
    game.start();
    let mut i = 0;
    println!("Hello, ratatetris!");
    while !game.is_gameover() {
        game.step();
        println!("{i}");
        game.draw();
        i += 1;
        if i > 200 {
            game.stop();
        }
    }
    println!("Game Over!");
}
