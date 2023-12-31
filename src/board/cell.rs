use super::creature::Creature;
use super::level::Level;
#[derive(Debug, Clone)]
pub struct Cell {
    pub food: Level,
    pub water: Level,
    pub danger: Level,
    pub creatures: Vec<Creature>,
}

impl Cell {
    pub fn random_new() -> Self {
        Cell {
            food: Level::random(),
            water: Level::random(),
            danger: Level::random(),
            creatures: vec![Creature::new(100, 100, 5, 5)],
        }
    }
}
