use std::time;

use rand::Rng;
fn main() {
    let mut board = Board::new(6, 5);
    println!("{:?}", board.contents);
    Creature::new(5 / 2, 5 / 2, &mut board);
    let mut now = time::Instant::now();
    loop {
        if now.elapsed().as_secs() > 1 {
            board.update();
            for x in board.contents.iter().enumerate() {
                for y in x.1 {
                    print!(
                        "{}\t",
                        y.creatures.len(),
                    )
                }
                println!("\n")
            }
            now = time::Instant::now();
        }
    }
}

#[derive(Clone, Debug)]
struct Board {
    x: i32,
    y: i32,
    contents: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(x: i32, y: i32) -> Self {
        let mut contents: Vec<Vec<Cell>> = vec![];
        for _ in 0..x {
            let mut line: Vec<Cell> = vec![];
            for _ in 0..y {
                line.push(Cell {
                    food_level: Level::random(),
                    water_level: Level::random(),
                    danger_level: Level::random(),
                    creatures: vec![],
                })
            }
            contents.push(line)
        }
        Board { x, y, contents }
    }

    pub fn update(&mut self) {

        let mut repro = 0;
        let mut contents: Vec<Vec<Cell>> = vec![];
        for i in self.contents.clone() {
            let mut cells: Vec<Cell> = vec![];
            for j in i {
                let mut creatures: Vec<Creature> = vec![];
                for mut z in j.creatures.clone() {
                    match j.food_level {
                        Level::Low => {
                            z.saturation = z.saturation - (j.creatures.len() as i32)
                                + 10 / (j.creatures.len() as i32)
                        }
                        Level::Normal => {
                            z.saturation = z.saturation - (j.creatures.len() as i32)
                                + 50 / (j.creatures.len() as i32)
                        }
                        Level::High => {
                            z.saturation = z.saturation - (j.creatures.len() as i32)
                                + 150 / (j.creatures.len() as i32)
                        }
                    }
                    if z.saturation > 100 {
                        repro += 1;
                        creatures.push(Creature {
                            x: z.x,
                            y: z.y,
                            saturation: z.saturation / 2,
                        })
                    } else if z.saturation > 0 {
                        creatures.push(Creature {
                            x: z.x,
                            y: z.y,
                            saturation: z.saturation,
                        });
                    }
                }
                cells.push(Cell {
                    food_level: j.food_level,
                    water_level: j.water_level,
                    danger_level: j.danger_level,
                    creatures,
                })
            }
            contents.push(cells);
        }
        self.contents = contents;
        for _ in 0..repro {
            Creature::new(rand::thread_rng().gen_range(0..=self.x-1), 
            rand::thread_rng().gen_range(0..=self.y-1), self)
        }
    }
}

#[derive(Debug, Clone)]
enum Level {
    Low,
    Normal,
    High,
}

impl Level {
    pub fn random() -> Level {
        match rand::thread_rng().gen_range(0..=2) {
            0 => Level::Low,
            1 => Level::Normal,
            2 => Level::High,
            _ => panic!("How????"),
        }
    }
}

#[derive(Debug, Clone)]
struct Cell {
    food_level: Level,
    water_level: Level,
    danger_level: Level,
    creatures: Vec<Creature>,
}

#[derive(Clone, Debug)]
struct Creature {
    x: i32,
    y: i32,
    saturation: i32,
}

impl Creature {
    fn new(x: i32, y: i32, board: &mut Board) {
        let mut my_x = x;
        let mut my_y = y;
        if my_x > board.x {
            my_x -= 1
        }
        if my_y > board.y {
            my_y -= 1;
        }
        if my_x < 0 {
            my_x += 1;
        }
        if my_y < 0 {
            my_y += 1;
        }

        board.contents[x as usize][y as usize].creatures.push(Self {
            x: my_x,
            y: my_y,
            saturation: 100,
        })
    }
}
