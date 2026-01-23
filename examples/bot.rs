use ratatetris::lmtetris::{Direction, Tetris};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Bot starting to play Ratatetris...");
    let mut game = Tetris::new(10, 20);
    game.start();

    let mut steps = 0;
    while !game.is_gameover() && steps < 500 {
        // Simple Bot Logic
        if rand::random() {
            let _ = game.move_tet(Some(Direction::Left), None);
        } else {
            let _ = game.move_tet(Some(Direction::Right), None);
        }

        if rand::random() {
            let _ = game.move_tet(None, Some(Direction::Left));
        }

        // Step the game (gravity)
        let (_, lines_cleared) = game.step();
        if lines_cleared {
            println!("Bot cleared lines!");
        }

        if steps % 20 == 0 {
            println!(
                "\nStep: {}, Score: {}, Level: {}",
                steps,
                game.score(),
                game.level()
            );
            game.print();
            // clear screen code if we wanted animation, but for logs scrolling is better
        }
        steps += 1;
        thread::sleep(Duration::from_millis(50));
    }

    println!("\nGame Over! Final Score: {}", game.score());
    println!("High Score: {}", game.high_score());
}
