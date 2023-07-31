mod board;
use board::Board;
use rand::Rng;
use std::time;

fn main() {
    let mut board: Board = Board::new(11, 6);
    let mut last_time = time::Instant::now();
    loop {
        if last_time.elapsed().as_secs() > 1 {
            board.update();
            println!("{}", board.to_string());
            last_time = time::Instant::now();
        }
    }
}
