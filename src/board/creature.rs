use super::Board;
use rand::Rng;
#[derive(Debug, Clone, Copy)]
pub struct Creature {
    pub x: usize,
    pub y: usize,
    pub thirst: usize,
    pub saturation: usize,
}

impl Creature {
    pub fn new(thirst: usize, saturation: usize, x: usize, y: usize) -> Self {
        Creature {
            x,
            y,
            thirst,
            saturation,
        }
    }

    pub fn reproduce(&mut self, board: &mut Board) {
        let new_x = match rand::thread_rng().gen_range(-1..1) {
            -1 => self.x - 1,
            0 => self.x,
            1 => self.x + 1,
            _ => panic!("value in range -1..1 is somehow not in range -1..1"),
        };
        let new_y = match rand::thread_rng().gen_range(-1..1) {
            -1 => self.y - 1,
            0 => self.y,
            1 => self.y + 1,
            _ => panic!("value in range -1..1 is somehow not in range -1..1"),
        };

        let result_pos = normalize_new_pos(board.x, board.y, new_x, new_y);

        let new_creature = Creature::new(
            self.thirst / 2,
            self.saturation / 2,
            result_pos.0,
            result_pos.1,
        );
        board.content[result_pos.0][result_pos.1]
            .creatures
            .push(new_creature);
    }

    fn feed(&mut self, food_amount: i32, water_amount: i32) {
        self.saturation += food_amount as usize;
        self.thirst += water_amount as usize;
    }
}

fn normalize_new_pos(board_x: usize, board_y: usize, x: usize, y: usize) -> (usize, usize) {
    let mut normalized_x = x;
    let mut normalized_y = y;
    if normalized_x > board_x {
        normalized_x -= 1
    } else if normalized_x < 0 {
        normalized_x += 1
    }
    if normalized_y > board_y {
        normalized_y -= 1
    } else if normalized_y < 0 {
        normalized_y += 1
    }

    (normalized_x, normalized_y)
}
