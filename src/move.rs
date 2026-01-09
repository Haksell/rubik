use {
    kiss3d::event::{Key, Modifiers},
    std::fmt::{Debug, Formatter},
};

// move.cycles

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
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
    pub const fn as_int(&self) -> u8 {
        *self as u8
    }

    pub const fn repetitions(&self) -> u8 {
        self.as_int() / 6 + 1
    }

    // TODO: #[deprecated]
    pub fn opposite(&self) -> Self {
        let mut i = self.as_int();
        if i < 6 {
            i += 12;
        } else if i > 11 {
            i %= 12;
        }
        Self::try_from(i).unwrap()
    }

    // TODO convertir en i16 pour pas overflow ou garder + 30 ?
    pub const fn same_face(&self, move_: &Self) -> bool {
        (self.as_int() + 30 - move_.as_int()).is_multiple_of(6)
    }

    pub fn random() -> Self {
        Self::choice(&MOVES)
    }

    pub fn choice(moves: &[Self]) -> Self {
        use rand::{prelude::*, rng};
        *moves.choose(&mut rng()).unwrap()
    }

    #[must_use]
    pub const fn rotate_y(&self) -> Self {
        match self {
            Self::F => Self::R,
            Self::R => Self::B,
            Self::B => Self::L,
            Self::L => Self::F,
            Self::F2 => Self::R2,
            Self::R2 => Self::B2,
            Self::B2 => Self::L2,
            Self::L2 => Self::F2,
            Self::F3 => Self::R3,
            Self::R3 => Self::B3,
            Self::B3 => Self::L3,
            Self::L3 => Self::F3,
            _ => *self,
        }
    }

    pub fn format_sequence(sequence: &[Self]) -> String {
        sequence
            .iter()
            .map(|move_| format!("{move_:?}"))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl TryFrom<u8> for Move {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::F),
            1 => Ok(Self::R),
            2 => Ok(Self::U),
            3 => Ok(Self::B),
            4 => Ok(Self::L),
            5 => Ok(Self::D),
            6 => Ok(Self::F2),
            7 => Ok(Self::R2),
            8 => Ok(Self::U2),
            9 => Ok(Self::B2),
            10 => Ok(Self::L2),
            11 => Ok(Self::D2),
            12 => Ok(Self::F3),
            13 => Ok(Self::R3),
            14 => Ok(Self::U3),
            15 => Ok(Self::B3),
            16 => Ok(Self::L3),
            17 => Ok(Self::D3),
            _ => Err("Moves are from 0 to 17 included"),
        }
    }
}

impl TryFrom<(Key, Modifiers)> for Move {
    type Error = &'static str;

    fn try_from(value: (Key, Modifiers)) -> Result<Self, Self::Error> {
        let (key, mods) = value;
        let move_ = match key {
            Key::F => Ok(Self::F),
            Key::R => Ok(Self::R),
            Key::U => Ok(Self::U),
            Key::B => Ok(Self::B),
            Key::L => Ok(Self::L),
            Key::D => Ok(Self::D),
            _ => Err("Invalid key"),
        };

        if let Ok(move_) = move_
            && mods.contains(Modifiers::Shift)
        {
            Ok(move_.opposite())
        } else {
            move_
        }
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = match self {
            Self::F => "F",
            Self::R => "R",
            Self::U => "U",
            Self::B => "B",
            Self::L => "L",
            Self::D => "D",
            Self::F2 => "F2",
            Self::R2 => "R2",
            Self::U2 => "U2",
            Self::B2 => "B2",
            Self::L2 => "L2",
            Self::D2 => "D2",
            Self::F3 => "F'",
            Self::R3 => "R'",
            Self::U3 => "U'",
            Self::B3 => "B'",
            Self::L3 => "L'",
            Self::D3 => "D'",
            Self::TR => "r",
            Self::TU => "u",
            Self::TB => "b",
            Self::TL => "l",
            Self::TR2 => "r'",
            Self::TU2 => "u'",
            Self::TB2 => "b'",
            Self::TL2 => "l'",
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
