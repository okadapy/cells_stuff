use super::Board;
use rand::Rng;
#[derive(Debug, Clone, Copy)]
pub struct Creature {
    pub x: usize,
    pub y: usize,
    pub thirst: i32,
    pub saturation: i32,
}

impl Creature {
    pub fn new(thirst: i32, saturation: i32, x: usize, y: usize) -> Self {
        Creature {
            x,
            y,
            thirst,
            saturation,
        }
    }

    pub fn reproduce(&mut self, board: &mut Board) {
        let new_x = match rand::thread_rng().gen_range(-1..1) {
            -1 => {
                if self.x > 0 {
                    self.x - 1
                } else {
                    self.x
                }
            }
            0 => self.x,
            1 => self.x + 1,
            _ => panic!("value in range -1..1 is somehow not in range -1..1"),
        };
        let new_y = match rand::thread_rng().gen_range(-1..1) {
            -1 => {
                if self.y > 0 {
                    self.y - 1
                } else {
                    self.y
                }
            }
            0 => self.y,
            1 => self.y + 1,
            _ => panic!("value in range -1..1 is somehow not in range -1..1"),
        };

        let result_pos = normalize_new_pos(board.x, board.y, new_x, new_y);

        let new_creature = Creature::new(
            self.thirst / 2,
            self.saturation / 2,
            result_pos.0.try_into().unwrap_or(0),
            result_pos.1.try_into().unwrap_or(0),
        );
        board.content[result_pos.0 as usize][result_pos.1 as usize]
            .creatures
            .push(new_creature);
        self.saturation /= 2;
        self.thirst /= 2;
    }

    pub fn feed(&mut self, food_amount: i32, water_amount: i32) {
        self.saturation += food_amount;
        self.thirst += water_amount;
    }
}

fn normalize_new_pos(board_x: usize, board_y: usize, x: usize, y: usize) -> (usize, usize) {
    let mut normalized_x = x;
    let mut normalized_y = y;
    if normalized_x > board_x {
        normalized_x -= 1
    } else if normalized_x <= 0 {
        normalized_x += 1
    }
    if normalized_y > board_y {
        normalized_y -= 1
    } else if normalized_y <= 0 {
        normalized_y += 1
    }

    (normalized_x, normalized_y)
}
