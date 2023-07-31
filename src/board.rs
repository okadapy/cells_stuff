mod cell;
mod creature;
mod level;
use cell::Cell;

use self::creature::Creature;
#[derive(Debug, Clone)]
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

    pub fn update(&mut self) {
        self.content.iter_mut().map(|row| {
            row.iter_mut().map(|cell| {
                cell.creatures
                    .iter_mut()
                    .filter(|creature| creature.saturation > 0 && creature.thirst > 0).map(move |creature| creature.reproduce(self))
            })
        });
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut buf: String = String::new();
        for row in &self.content {
            let mut row_buf: String = String::new();
            for cell in row {
                row_buf.push_str(&format!(
                    "{},\t{},\t{}\t[{}]\t",
                    cell.food.to_string(),
                    cell.water.to_string(),
                    cell.danger.to_string(),
                    cell.creatures.len()
                ));
            }
            buf.push_str(&format!("{}\n", row_buf))
        }
        buf
    }
}
