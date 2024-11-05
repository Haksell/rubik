use kiss3d::event::{Key, Modifiers};
use std::fmt::{Debug, Formatter};

// move.cycles

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Move {
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

impl Move {
    pub fn as_int(&self) -> u8 {
        *self as u8
    }

    pub fn repetitions(&self) -> u8 {
        self.as_int() / 6 + 1
    }

    pub fn opposite(&self) -> Move {
        let mut i = self.as_int();
        if i < 6 {
            i += 12;
        } else if i > 11 {
            i %= 12;
        }
        Move::try_from(i).unwrap()
    }

    pub fn same_face(&self, move_: &Move) -> bool {
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
            Move::F => Move::R,
            Move::R => Move::B,
            Move::B => Move::L,
            Move::L => Move::F,
            Move::F2 => Move::R2,
            Move::R2 => Move::B2,
            Move::B2 => Move::L2,
            Move::L2 => Move::F2,
            Move::F3 => Move::R3,
            Move::R3 => Move::B3,
            Move::B3 => Move::L3,
            Move::L3 => Move::F3,
            _ => *self,
        }
    }

    pub fn format_sequence(sequence: &Vec<Move>) -> String {
        sequence
            .iter()
            .map(|move_| format!("{:?}", move_))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl TryFrom<u8> for Move {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Move::F),
            1 => Ok(Move::R),
            2 => Ok(Move::U),
            3 => Ok(Move::B),
            4 => Ok(Move::L),
            5 => Ok(Move::D),
            6 => Ok(Move::F2),
            7 => Ok(Move::R2),
            8 => Ok(Move::U2),
            9 => Ok(Move::B2),
            10 => Ok(Move::L2),
            11 => Ok(Move::D2),
            12 => Ok(Move::F3),
            13 => Ok(Move::R3),
            14 => Ok(Move::U3),
            15 => Ok(Move::B3),
            16 => Ok(Move::L3),
            17 => Ok(Move::D3),
            _ => Err("Moves are from 0 to 17 included"),
        }
    }
}

impl TryFrom<(Key, Modifiers)> for Move {
    type Error = &'static str;

    fn try_from(value: (Key, Modifiers)) -> Result<Self, Self::Error> {
        let (key, mods) = value;
        let move_ = match key {
            Key::F => Ok(Move::F),
            Key::R => Ok(Move::R),
            Key::U => Ok(Move::U),
            Key::B => Ok(Move::B),
            Key::L => Ok(Move::L),
            Key::D => Ok(Move::D),
            _ => Err("Invalid key"),
        };

        if move_.is_ok() && mods.contains(Modifiers::Shift) {
            Ok(move_.unwrap().opposite())
        } else {
            move_
        }
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = match self {
            Move::F => "F",
            Move::R => "R",
            Move::U => "U",
            Move::B => "B",
            Move::L => "L",
            Move::D => "D",
            Move::F2 => "F2",
            Move::R2 => "R2",
            Move::U2 => "U2",
            Move::B2 => "B2",
            Move::L2 => "L2",
            Move::D2 => "D2",
            Move::F3 => "F'",
            Move::R3 => "R'",
            Move::U3 => "U'",
            Move::B3 => "B'",
            Move::L3 => "L'",
            Move::D3 => "D'",
            Move::TR => "r",
            Move::TU => "u",
            Move::TB => "b",
            Move::TL => "l",
            Move::TR2 => "r'",
            Move::TU2 => "u'",
            Move::TB2 => "b'",
            Move::TL2 => "l'",
        };
        write!(f, "{s}")
    }
}

pub const MOVES: [Move; 18] = [
    Move::F,
    Move::R,
    Move::U,
    Move::B,
    Move::L,
    Move::D,
    Move::F2,
    Move::R2,
    Move::U2,
    Move::B2,
    Move::L2,
    Move::D2,
    Move::F3,
    Move::R3,
    Move::U3,
    Move::B3,
    Move::L3,
    Move::D3,
];

pub const MOVES_RUL: [Move; 9] = [
    Move::R,
    Move::U,
    Move::L,
    Move::R2,
    Move::U2,
    Move::L2,
    Move::R3,
    Move::U3,
    Move::L3,
];

pub const MOVES_RU: [Move; 6] = [Move::R, Move::U, Move::R2, Move::U2, Move::R3, Move::U3];
