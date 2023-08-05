use log::debug;
use rand::Rng;
use std::fmt::Display;

pub enum Wound {
    Missing,
    DeepCut,
    OpenFracture,
    Fracture,
    Dislocated,
}

impl Display for Wound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Missing => "Отсутствие",
                Self::DeepCut => "Глубокий порез",
                Self::OpenFracture => "Открытый перелом",
                Self::Fracture => "Перелом",
                Self::Dislocated => "Вывих",
            }
        )
    }
}

impl Wound {
    pub fn generate() -> Self {
        let random = rand::thread_rng().gen_range(0..5);
        let value = match random {
            0 => Self::Missing,
            1 => Self::DeepCut,
            2 => Self::Dislocated,
            3 => Self::Fracture,
            4 => Self::OpenFracture,
            _ => panic!("{} is out of range 0..5", random),
        };
        debug!("Травма:\t{}\t=>\t{}", random, value);
        value
    }
}
