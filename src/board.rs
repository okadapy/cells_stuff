mod cell;
mod level;
use cell::Cell;

pub struct Board {
    x: usize,
    y: usize,
    content: Vec<Vec<Cell>>
}