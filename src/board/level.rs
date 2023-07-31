use rand::Rng;
#[derive(Debug, Clone)]
pub enum Level {
    Zero,
    Low,
    Normal,
    High,
}

impl ToString for Level {
    fn to_string(&self) -> String {
        match self {
            Level::Zero => "None".to_string(),
            Level::Low => "Low".to_string(),
            Level::Normal => "Normal".to_string(),
            Level::High => "High".to_string(),
        }
    }
}

impl Level {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..4) {
            0 => Level::Zero,
            1 => Level::Low,
            2 => Level::Normal,
            3 => Level::High,
            _ => {
                panic!("random value in range 0..4 is not in range 0..4. IDK how is that possible.")
            }
        }
    }
}
