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

#[derive(Clone, Default, Debug)]
pub struct Color {
    pub c: u8,
}

#[derive(Clone, Debug)]
struct Tile {
    filled: bool,
    color: Color,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn from(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    fn mov(&mut self, dir: &Direction) {
        match dir {
            Direction::Left => *self = *self + (-1, 0),
            Direction::Right => *self = *self + (1, 0),
            Direction::Up => *self = *self + (0, 1),
            Direction::Down => *self = *self + (0, -1),
        }
    }
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

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy, Default)]
enum TetrominoVariant {
    #[default]
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl TetrominoVariant {
    fn to_index(self) -> usize {
        match self {
            TetrominoVariant::I => 0,
            TetrominoVariant::J => 1,
            TetrominoVariant::L => 2,
            TetrominoVariant::O => 3,
            TetrominoVariant::S => 4,
            TetrominoVariant::T => 5,
            TetrominoVariant::Z => 6,
        }
    }
    fn by_index(i: usize) -> Self {
        match i {
            0 => TetrominoVariant::I,
            1 => TetrominoVariant::J,
            2 => TetrominoVariant::L,
            3 => TetrominoVariant::O,
            4 => TetrominoVariant::S,
            5 => TetrominoVariant::T,
            6 => TetrominoVariant::Z,
            _ => panic!("No Variant i=\"{i}\"!"),
        }
    }
    fn random() -> Self {
        TetrominoVariant::by_index(rand::random_range(0..7))
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
enum Rotation {
    #[default]
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Rotation {
    fn rotate(&mut self, d: &Direction) {
        match self {
            Rotation::Deg0 => {
                if *d == Direction::Right {
                    *self = Rotation::Deg90;
                }
                if *d == Direction::Left {
                    *self = Rotation::Deg270;
                }
            }
            Rotation::Deg90 => {
                if *d == Direction::Right {
                    *self = Rotation::Deg180;
                }
                if *d == Direction::Left {
                    *self = Rotation::Deg0;
                }
            }
            Rotation::Deg180 => {
                if *d == Direction::Right {
                    *self = Rotation::Deg270;
                }
                if *d == Direction::Left {
                    *self = Rotation::Deg90;
                }
            }
            Rotation::Deg270 => {
                if *d == Direction::Right {
                    *self = Rotation::Deg0;
                }
                if *d == Direction::Left {
                    *self = Rotation::Deg180;
                }
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct Tetromino {
    variant: TetrominoVariant,
    center: Point,
    rotation: Rotation,
    pub color: Color,
}

impl Tetromino {
    fn random(x: i32, y: i32) -> Self {
        let variant = TetrominoVariant::random();
        Self {
            variant,
            center: Point { x, y },
            rotation: Rotation::Deg0,
            color: Color {
                c: 9 + variant.to_index() as u8,
            },
        }
    }
    pub fn points(&self) -> [Point; 4] {
        let mut tet = TETS[self.variant.to_index()];
        for p in &mut tet {
            let p_prev = *p;
            match self.rotation {
                Rotation::Deg0 => (),
                Rotation::Deg90 => {
                    p.x = p_prev.y;
                    p.y = -p_prev.x;
                }
                Rotation::Deg180 => {
                    p.x = -p_prev.x;
                    p.y = -p_prev.y;
                }
                Rotation::Deg270 => {
                    p.x = -p_prev.y;
                    p.y = p_prev.x;
                }
            }
            *p = *p + self.center;
        }
        tet
    }
    fn fall(&mut self) {
        self.center = self.center + (0, 1);
    }
    fn collides(&self, board: &Board) -> bool {
        for p in self.points() {
            if p.y < 0 {
                return true;
            }
            if p.y >= board.height {
                return true;
            }
            if p.x < 0 {
                return true;
            }
            if p.x >= board.width {
                return true;
            }
            if board.get_tile(p).filled {
                return true;
            }
        }
        false
    }
    fn rotate(&mut self, dir: &Direction) -> bool {
        self.rotation.rotate(dir);
        true
    }
    fn mov(&mut self, dir: &Direction) -> bool {
        self.center.mov(dir);
        true
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

pub struct Tetris {
    state: GameState,
    board: Board,
    tetromino: Tetromino,
}

#[derive(PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Board {
    fn new(width: i32, height: i32) -> Self {
        assert!(width >= 4);
        assert!(height >= 4);
        Board {
            tiles: vec![
                Tile {
                    filled: false,
                    color: Color { c: 0 },
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
    fn fill_tetromino_check_gameover(&mut self, tet: &Tetromino) -> bool {
        for p in &tet.points() {
            let i = self.p2i(p);
            let t = &mut self.tiles[i];
            if t.filled {
                return true;
            } else {
                t.filled = true;
                t.color = tet.color.clone();
            }
        }
        false
    }
    fn check_and_remove_lines(&mut self) -> bool {
        let mut c = 0;
        let mut lines_cleared = false;
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point::from(x, y);
                if self.tiles[self.p2i(&p)].filled {
                    c += 1;
                }
            }
            if c == self.width {
                self.remove_line_and_drop(y);
                lines_cleared = true;
            }
            c = 0;
        }
        lines_cleared
    }
    fn remove_line_and_drop(&mut self, y: i32) {
        for y in (1..=y).rev() {
            for x in 0..self.width {
                let i = self.p2i(&Point::from(x, y));
                let i_top = self.p2i(&Point::from(x, y - 1));
                let tile_top = self.tiles[i_top].clone();
                let tile = &mut self.tiles[i];
                tile.filled = tile_top.filled;
                tile.color = tile_top.color;
            }
        }
    }
}

impl Tetris {
    pub fn new(width: i32, height: i32) -> Self {
        Tetris {
            state: GameState::Paused,
            board: Board::new(width, height),
            tetromino: Tetromino::random(width / 2, 0),
        }
    }
    pub fn dimensions(&self) -> (i32, i32) {
        (self.board.width, self.board.height)
    }
    pub fn start(&mut self) {
        self.state = GameState::Running
    }
    pub fn pause(&mut self) {
        self.state = GameState::Paused
    }
    pub fn is_running(&self) -> bool {
        self.state == GameState::Running
    }
    pub fn is_gameover(&self) -> bool {
        self.state == GameState::GameOver
    }
    pub fn move_tet(&mut self, mov: Option<Direction>, rot: Option<Direction>) -> bool {
        if self.state != GameState::Running {
            return false;
        }
        let mut tet_next = self.tetromino.clone();

        if let Some(r) = rot {
            tet_next.rotate(&r);
        }
        if tet_next.collides(&self.board) {
            return false;
        }

        if let Some(m) = mov {
            tet_next.mov(&m);
        }
        if tet_next.collides(&self.board) {
            return false;
        }

        self.tetromino = tet_next;
        true
    }

    pub fn get_tetromino(&self) -> &Tetromino {
        &self.tetromino
    }

    pub fn get_tetromino_preview(&self) -> Tetromino {
        let mut tet = self.tetromino.clone();
        while !tet.collides(&self.board) {
            tet.fall();
        }
        tet.mov(&Direction::Down);
        tet
    }

    pub fn rush(&mut self) -> bool {
        if self.state != GameState::Running {
            return false;
        }
        let mut prev = self.tetromino.clone();
        self.tetromino.fall();
        while !self.tetromino.collides(&self.board) {
            self.tetromino.fall();
            prev.fall();
        }
        if self.board.fill_tetromino_check_gameover(&prev) {
            self.state = GameState::GameOver;
            return false;
        }
        let lines_cleared = self.board.check_and_remove_lines();
        self.tetromino = Tetromino::random(self.board.width / 2, 0);
        lines_cleared
    }

    pub fn step(&mut self) -> (bool, bool) {
        if self.state != GameState::Running {
            return (false, false);
        }
        let prev = self.tetromino.clone();
        self.tetromino.fall();
        if self.tetromino.collides(&self.board) {
            if self.board.fill_tetromino_check_gameover(&prev) {
                self.state = GameState::GameOver;
                return (false, false);
            }
            let lines_cleared = self.board.check_and_remove_lines();
            self.tetromino = Tetromino::random(self.board.width / 2, 0);
            return (true, lines_cleared);
        }
        (false, false)
    }

    pub fn tile_color(&self, x: i32, y: i32) -> Color {
        let b = &self.board;
        b.tiles[b.p2i(&Point::from(x, y))].color.clone()
    }

    pub fn print(&self) {
        for j in 0..self.board.height {
            print!("{j:2}");
            for i in 0..self.board.width {
                let p = Point { x: i, y: j };
                let mut c = ' ';
                let t = &self.board.get_tile(p);
                if t.filled {
                    c = 'x';
                } else {
                    let mut idx = '0';
                    for tp in self.tetromino.points() {
                        idx = (idx as u8 + 1) as char;
                        if tp == p {
                            c = idx;
                            break;
                        }
                    }
                }
                print!("{c}");
            }
            println!();
        }
        println!(
            "  {}",
            String::from_iter((0..self.board.width).map(|e| (e + 0x30) as u8 as char))
        );
    }
}
