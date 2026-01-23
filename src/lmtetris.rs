use rand::seq::SliceRandom;
use std::cmp::Eq;
use std::fs;
use std::ops::Add;

const HIGHSCORE_FILE: &str = "highscore.txt";

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

/// 7-Bag Randomizer: shuffles all 7 tetrominos, deals them out, then reshuffles
struct Bag {
    pieces: Vec<TetrominoVariant>,
}

impl Bag {
    fn new() -> Self {
        let mut bag = Bag { pieces: Vec::new() };
        bag.refill();
        bag
    }

    fn refill(&mut self) {
        self.pieces = vec![
            TetrominoVariant::I,
            TetrominoVariant::J,
            TetrominoVariant::L,
            TetrominoVariant::O,
            TetrominoVariant::S,
            TetrominoVariant::T,
            TetrominoVariant::Z,
        ];
        self.pieces.shuffle(&mut rand::rng());
    }

    fn next(&mut self) -> TetrominoVariant {
        if self.pieces.is_empty() {
            self.refill();
        }
        self.pieces.pop().unwrap()
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
    fn new(variant: TetrominoVariant, x: i32, y: i32) -> Self {
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
    fn offset(&mut self, dx: i32, dy: i32) {
        self.center.x += dx;
        self.center.y += dy;
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
    score: u32,
    high_score: u32,
    lines_cleared: u32,
    level: u32,
    bag: Bag,
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
    fn is_line_full(&self, y: i32) -> bool {
        (0..self.width).all(|x| self.tiles[self.p2i(&Point::from(x, y))].filled)
    }
    fn clear_line(&mut self, y: i32) {
        // Drop all rows above down by one
        for row in (1..=y).rev() {
            let src_start = ((row - 1) * self.width) as usize;
            let dst_start = (row * self.width) as usize;
            for x in 0..self.width as usize {
                self.tiles[dst_start + x] = self.tiles[src_start + x].clone();
            }
        }
        // Clear top row
        for x in 0..self.width as usize {
            self.tiles[x] = Tile {
                filled: false,
                color: Color { c: 0 },
            };
        }
    }
    fn check_and_clear_lines(&mut self) -> u32 {
        let mut lines_count = 0;
        let mut y = self.height - 1;
        while y >= 0 {
            if self.is_line_full(y) {
                self.clear_line(y);
                lines_count += 1;
                // Don't decrement y - check same row again since rows dropped
            } else {
                y -= 1;
            }
        }
        lines_count
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
}

impl Tetris {
    fn load_highscore() -> u32 {
        if let Ok(content) = fs::read_to_string(HIGHSCORE_FILE) {
            content.trim().parse().unwrap_or(0)
        } else {
            0
        }
    }

    fn save_highscore(&self) {
        let _ = fs::write(HIGHSCORE_FILE, self.high_score.to_string());
    }

    pub fn new(width: i32, height: i32) -> Self {
        let mut bag = Bag::new();
        let tetromino = Tetromino::new(bag.next(), width / 2, 0);
        Tetris {
            state: GameState::Paused,
            board: Board::new(width, height),
            tetromino,
            score: 0,
            high_score: Tetris::load_highscore(),
            lines_cleared: 0,
            level: 1,
            bag,
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

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn high_score(&self) -> u32 {
        self.high_score
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    /// Returns tick rate in milliseconds based on current level
    /// Speed increases as level goes up (tick rate decreases)
    pub fn tick_rate_ms(&self) -> u64 {
        // Base tick rate of 500ms at level 1
        // Each level reduces by ~10%, minimum 100ms
        let base = 500u64;
        let reduction = (self.level - 1) as u64 * 40;
        base.saturating_sub(reduction).max(100)
    }

    /// Calculate and add score based on lines cleared (classic Tetris scoring)
    fn add_score(&mut self, lines: u32) {
        let points = match lines {
            1 => 100, // Single
            2 => 300, // Double
            3 => 500, // Triple
            4 => 800, // Tetris
            _ => 0,
        };
        self.score += points * self.level;
        if self.score > self.high_score {
            self.high_score = self.score;
            self.save_highscore();
        }
    }

    /// Check if player should level up (every 10 lines)
    fn check_level_up(&mut self) {
        let new_level = (self.lines_cleared / 10) + 1;
        if new_level > self.level {
            self.level = new_level;
        }
    }

    pub fn move_tet(&mut self, mov: Option<Direction>, rot: Option<Direction>) -> bool {
        if self.state != GameState::Running {
            return false;
        }
        let mut tet_next = self.tetromino.clone();

        if let Some(r) = rot {
            tet_next.rotate(&r);
            if tet_next.collides(&self.board) {
                // Simple Wall Kick Logic
                // Try shifting right, left, up (floor kick), and 2 steps for I-piece
                let kicks = [(1, 0), (-1, 0), (0, -1), (2, 0), (-2, 0)];
                let mut success = false;
                for (dx, dy) in kicks {
                    let mut test_tet = tet_next.clone();
                    test_tet.offset(dx, dy);
                    if !test_tet.collides(&self.board) {
                        tet_next = test_tet;
                        success = true;
                        break;
                    }
                }
                if !success {
                    return false;
                }
            }
        }
        // No need to check collides here again because we either didn't rotate,
        // rotated successfully without kick, or kicked successfully.
        // But we DO need to check if we are applying movement NEXT.

        if let Some(m) = mov {
            tet_next.mov(&m);
            // If movement causes collision, we revert ONLY the movement, but keep rotation?
            // Usually rotate and move are separate events.
            // But this function handles both.
            // If rotation succeeded (maybe with kick), tet_next is rotated.
            // Then we apply move.
            if tet_next.collides(&self.board) {
                // If move collides, we should probably return false?
                // Or should we just ignore the move but keep rotation?
                // The UI calls `rotate_right` (only rot) or `move_left` (only mov).
                // It rarely calls both.
                // But if it did, standard behavior is atomic or sequential.
                // Given the signature `Option<Direction>`, it allows both.
                // If move fails, we likely want to keep the rotation if it succeeded.
                // But `return false` implies the whole operation failed.
                // Let's assume if move fails, we fall back to just rotated state?
                // Or just fail the whole thing.
                // Given existing code:
                /*
                if tet_next.collides(&self.board) {
                     return false;
                }
                */
                // It returns false. So if I pass (Some(Move), Some(Rot)), and Rot succeeds but Move fails,
                // the whole thing is cancelled. That seems acceptable.
                return false;
            }
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

    /// Lock the current piece, clear lines, update score, and spawn new piece.
    /// Returns (is_gameover, lines_cleared_count)
    fn lock_piece(&mut self, piece: &Tetromino) -> (bool, u32) {
        if self.board.fill_tetromino_check_gameover(piece) {
            self.state = GameState::GameOver;
            return (true, 0);
        }
        let lines = self.board.check_and_clear_lines();
        if lines > 0 {
            self.lines_cleared += lines;
            self.add_score(lines);
            self.check_level_up();
        }
        self.tetromino = Tetromino::new(self.bag.next(), self.board.width / 2, 0);
        if self.tetromino.collides(&self.board) {
            self.state = GameState::GameOver;
            return (true, lines);
        }
        (false, lines)
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
        let (gameover, lines) = self.lock_piece(&prev);
        !gameover && lines > 0
    }

    pub fn step(&mut self) -> (bool, bool) {
        if self.state != GameState::Running {
            return (false, false);
        }
        let prev = self.tetromino.clone();
        self.tetromino.fall();
        if self.tetromino.collides(&self.board) {
            let (gameover, lines) = self.lock_piece(&prev);
            if gameover {
                return (false, false);
            }
            return (true, lines > 0);
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
