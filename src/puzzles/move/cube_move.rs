use kiss3d::event::{Key, Modifiers};
use std::fmt::{Debug, Formatter};

use super::Move;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum CubeMove {
    F,
    R,
    U,
    B,
    L,
    D,
    F2,
    R2,
    U2,
    B2,
    L2,
    D2,
    F3,
    R3,
    U3,
    B3,
    L3,
    D3,
    TR,
    TU,
    TB,
    TL,
    TR2,
    TU2,
    TB2,
    TL2,
}

impl Move for CubeMove {}

impl CubeMove {
    pub fn as_int(&self) -> u8 {
        *self as u8
    }

    pub fn repetitions(&self) -> u8 {
        self.as_int() / 6 + 1
    }

    pub fn opposite(&self) -> CubeMove {
        let mut i = self.as_int();
        if i < 6 {
            i += 12;
        } else if i > 11 {
            i %= 12;
        }
        CubeMove::try_from(i).unwrap()
    }

    pub fn same_face(&self, move_: &CubeMove) -> bool {
        return (self.as_int() + 18 - move_.as_int()) % 6 == 0;
    }

    pub fn random() -> Self {
        Self::choice(&MOVES)
    }

    pub fn choice(moves: &[Self]) -> Self {
        use rand::prelude::*;
        *moves.choose(&mut thread_rng()).unwrap()
    }

    pub fn rotate_y(&self) -> Self {
        match self {
            CubeMove::F => CubeMove::R,
            CubeMove::R => CubeMove::B,
            CubeMove::B => CubeMove::L,
            CubeMove::L => CubeMove::F,
            CubeMove::F2 => CubeMove::R2,
            CubeMove::R2 => CubeMove::B2,
            CubeMove::B2 => CubeMove::L2,
            CubeMove::L2 => CubeMove::F2,
            CubeMove::F3 => CubeMove::R3,
            CubeMove::R3 => CubeMove::B3,
            CubeMove::B3 => CubeMove::L3,
            CubeMove::L3 => CubeMove::F3,
            _ => *self,
        }
    }

    pub fn format_sequence(sequence: &Vec<CubeMove>) -> String {
        sequence
            .iter()
            .map(|move_| format!("{:?}", move_))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl TryFrom<u8> for CubeMove {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CubeMove::F),
            1 => Ok(CubeMove::R),
            2 => Ok(CubeMove::U),
            3 => Ok(CubeMove::B),
            4 => Ok(CubeMove::L),
            5 => Ok(CubeMove::D),
            6 => Ok(CubeMove::F2),
            7 => Ok(CubeMove::R2),
            8 => Ok(CubeMove::U2),
            9 => Ok(CubeMove::B2),
            10 => Ok(CubeMove::L2),
            11 => Ok(CubeMove::D2),
            12 => Ok(CubeMove::F3),
            13 => Ok(CubeMove::R3),
            14 => Ok(CubeMove::U3),
            15 => Ok(CubeMove::B3),
            16 => Ok(CubeMove::L3),
            17 => Ok(CubeMove::D3),
            _ => Err("Moves are from 0 to 17 included"),
        }
    }
}

impl TryFrom<(Key, Modifiers)> for CubeMove {
    type Error = &'static str;

    fn try_from(value: (Key, Modifiers)) -> Result<Self, Self::Error> {
        let (key, mods) = value;
        let move_ = match key {
            Key::F => Ok(CubeMove::F),
            Key::R => Ok(CubeMove::R),
            Key::U => Ok(CubeMove::U),
            Key::B => Ok(CubeMove::B),
            Key::L => Ok(CubeMove::L),
            Key::D => Ok(CubeMove::D),
            _ => Err("Invalid key"),
        };

        if move_.is_ok() && mods.contains(Modifiers::Shift) {
            Ok(move_.unwrap().opposite())
        } else {
            move_
        }
    }
}

impl TryFrom<&str> for CubeMove {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "F" => Ok(CubeMove::F),
            "F2" => Ok(CubeMove::F2),
            "F'" | "F’" => Ok(CubeMove::F3),
            "R" => Ok(CubeMove::R),
            "R2" => Ok(CubeMove::R2),
            "R'" | "R’" => Ok(CubeMove::R3),
            "U" => Ok(CubeMove::U),
            "U2" => Ok(CubeMove::U2),
            "U'" | "U’" => Ok(CubeMove::U3),
            "B" => Ok(CubeMove::B),
            "B2" => Ok(CubeMove::B2),
            "B'" | "B’" => Ok(CubeMove::B3),
            "L" => Ok(CubeMove::L),
            "L2" => Ok(CubeMove::L2),
            "L'" | "L’" => Ok(CubeMove::L3),
            "D" => Ok(CubeMove::D),
            "D2" => Ok(CubeMove::D2),
            "D'" | "D’" => Ok(CubeMove::D3),
            "r" => Ok(CubeMove::TR),
            "u" => Ok(CubeMove::TU),
            "b" => Ok(CubeMove::TB),
            "l" => Ok(CubeMove::TL),
            "r'" | "r’" => Ok(CubeMove::TR2),
            "u'" | "u’" => Ok(CubeMove::TU2),
            "b'" | "b’" => Ok(CubeMove::TB2),
            "l'" | "l’" => Ok(CubeMove::TL2),
            _ => Err(format!("Invalid move '{value}'")),
        }
    }
}

impl Debug for CubeMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = match self {
            CubeMove::F => "F",
            CubeMove::R => "R",
            CubeMove::U => "U",
            CubeMove::B => "B",
            CubeMove::L => "L",
            CubeMove::D => "D",
            CubeMove::F2 => "F2",
            CubeMove::R2 => "R2",
            CubeMove::U2 => "U2",
            CubeMove::B2 => "B2",
            CubeMove::L2 => "L2",
            CubeMove::D2 => "D2",
            CubeMove::F3 => "F'",
            CubeMove::R3 => "R'",
            CubeMove::U3 => "U'",
            CubeMove::B3 => "B'",
            CubeMove::L3 => "L'",
            CubeMove::D3 => "D'",
            CubeMove::TR => "r",
            CubeMove::TU => "u",
            CubeMove::TB => "b",
            CubeMove::TL => "l",
            CubeMove::TR2 => "r'",
            CubeMove::TU2 => "u'",
            CubeMove::TB2 => "b'",
            CubeMove::TL2 => "l'",
        };
        write!(f, "{s}")
    }
}

pub const MOVES: [CubeMove; 18] = [
    CubeMove::F,
    CubeMove::R,
    CubeMove::U,
    CubeMove::B,
    CubeMove::L,
    CubeMove::D,
    CubeMove::F2,
    CubeMove::R2,
    CubeMove::U2,
    CubeMove::B2,
    CubeMove::L2,
    CubeMove::D2,
    CubeMove::F3,
    CubeMove::R3,
    CubeMove::U3,
    CubeMove::B3,
    CubeMove::L3,
    CubeMove::D3,
];

pub const MOVES_RUL: [CubeMove; 9] = [
    CubeMove::R,
    CubeMove::U,
    CubeMove::L,
    CubeMove::R2,
    CubeMove::U2,
    CubeMove::L2,
    CubeMove::R3,
    CubeMove::U3,
    CubeMove::L3,
];

pub const MOVES_RU: [CubeMove; 6] = [
    CubeMove::R,
    CubeMove::U,
    CubeMove::R2,
    CubeMove::U2,
    CubeMove::R3,
    CubeMove::U3,
];
