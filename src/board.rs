mod cell;
mod creature;
mod level;
use cell::Cell;

pub struct Board {
    x: usize,
    y: usize,
    content: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(x: usize, y: usize) -> Self {
        let mut content: Vec<Vec<Cell>> = vec![];
        for _ in 0..x {
            let mut row: Vec<Cell> = vec![];
            for _ in 0..y {
                row.push(Cell::random_new())
            }
            content.push(row)
        }
        Board { x, y, content }
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut buf: String = String::new();
        for row in &self.content {
            let mut row_buf: String = String::new();
            for cell in row {
                row_buf.push_str(&format!(
                    "{}, {}, {} [{}]\t",
                    cell.food.to_string(),
                    cell.water.to_string(),
                    cell.danger.to_string(),
                    cell.creatures.len()
                ))
            }
        }
        buf
    }
}
