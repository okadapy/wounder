use std::fmt::Display;

use crate::wounds;
use log::debug;
use rand::Rng;
use wounds::Wound;

pub enum Side {
    Left,
    Right,
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Left => "Слева",
                Self::Right => "Справа",
            }
        )
    }
}

impl Side {
    fn generate() -> Self {
        let random = rand::thread_rng().gen_range(0..2);
        let value = match random {
            0 => Self::Left,
            1 => Self::Right,
            _ => panic!("{} out of range of 0..2!", random),
        };
        debug!("Сторона:\t{}\t=>\t{}", random, value);
        value
    }
}

pub enum Bodypart {
    Shoulder(Side, Wound),
    Forearm(Side, Wound),
    Wrist(Side, Wound),
    Ribs(Side, Wound),
    Stomach(Wound),
    HipJoint(Side, Wound),
    Hip(Side, Wound),
    Knee(Side, Wound),
    Shin(Side, Wound),
    Foot(Side, Wound),
}

impl Display for Bodypart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Foot(side, wound) => format!("{} Ступни {}", wound, side),
                Self::Shoulder(side, wound) => format!("{} Плеча {}", wound, side),
                Self::Forearm(side, wound) => format!("{} Предплечья {}", wound, side),
                Self::Wrist(side, wound) => format!("{} Запястья {}", wound, side),
                Self::Ribs(side, wound) => format!("{} Ребер {}", wound, side),
                Self::HipJoint(side, wound) => format!("{} Таза {}", wound, side),
                Self::Hip(side, wound) => format!("{} Бедра {}", wound, side),
                Self::Knee(side, wound) => format!("{} Колена {}", wound, side),
                Self::Shin(side, wound) => format!("{} Голени {}", wound, side),
                Self::Stomach(wound) => format!("{} Живота", wound),
            }
        )
    }
}

impl Bodypart {
    pub fn generate() -> Self {
        let random = rand::thread_rng().gen_range(0..10);
        let value = match random {
            0 => Self::Foot(Side::generate(), Wound::generate()),
            1 => Self::Shoulder(Side::generate(), Wound::generate()),
            2 => Self::Forearm(Side::generate(), Wound::generate()),
            3 => Self::Wrist(Side::generate(), Wound::generate()),
            4 => Self::Ribs(Side::generate(), Wound::generate()),
            5 => Self::HipJoint(Side::generate(), Wound::generate()),
            6 => Self::Hip(Side::generate(), Wound::generate()),
            7 => Self::Knee(Side::generate(), Wound::generate()),
            8 => Self::Shin(Side::generate(), Wound::generate()),
            9 => Self::Stomach(Wound::generate()),
            _ => panic!("{} out of 0..10", random),
        };
        debug!("Часть:\t{}\t=>\t{}", random, value);
        value
    }
}
