mod cell;
mod creature;
mod level;
use std::{borrow::BorrowMut, ops::IndexMut};

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
        let mut board = self.clone();
        let mut reproduction: Board = self.clone();
        let mut reproduce: Vec<((usize, usize, usize), (usize, usize))> = vec![];
        for row in board.content.iter_mut() {
            for cell in row.iter_mut() {
                cell.creatures = cell.creatures.iter().filter(|creature| creature.saturation > 0 || creature.thirst > 0).cloned().collect();
                for creature in cell.creatures.clone().iter_mut() {
                        creature.feed(
                            match cell.food {
                                level::Level::Zero => (0 - (cell.creatures.len() as i32)),
                                level::Level::Low => {
                                    (10 / (cell.creatures.len() as i32)
                                        - (cell.creatures.len() as i32))
                                }
                                level::Level::Normal => {
                                    (50 / (cell.creatures.len() as i32)
                                        - (cell.creatures.len() as i32))
                                }
                                level::Level::High => {
                                    (150 / (cell.creatures.len() as i32)
                                        - (cell.creatures.len() as i32))
                                }
                            },
                            match cell.water {
                                level::Level::Zero => (0 - (cell.creatures.len() as i32)),
                                level::Level::Low => {
                                    (10 / (cell.creatures.len() as i32)
                                        - (cell.creatures.len() as i32))
                                }
                                level::Level::Normal => {
                                    (50 / (cell.creatures.len() as i32)
                                        - (cell.creatures.len() as i32))
                                }
                                level::Level::High => {
                                    (150 / (cell.creatures.len() as i32)
                                        - (cell.creatures.len() as i32))
                                }
                            },
                        );
                        creature.reproduce(&mut reproduction)
                    }
                }
            }
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
