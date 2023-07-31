mod board;
use board::Board;
use rand::Rng;
use std::time;

fn main() {
    let board: Board = Board::new( 11, 6);
    println!("{}", board.to_string())
}
