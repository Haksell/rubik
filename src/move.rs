use std::fmt::{Debug, Formatter};

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
}

impl Move {
    // TODO: there is probably a better way
    pub fn iterator() -> Vec<Move> {
        (0..18)
            .map(|m| Move::try_from(m).unwrap())
            .collect::<Vec<Move>>()
    }

    pub fn as_int(&self) -> u8 {
        *self as u8
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
        // TODO: one modulo
        return self.as_int() % 6 == move_.as_int() % 6;
    }
}

impl TryFrom<u8> for Move {
    type Error = ();

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
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "F" => Ok(Move::F),
            "F2" => Ok(Move::F2),
            "F'" | "F’" => Ok(Move::F3),
            "R" => Ok(Move::R),
            "R2" => Ok(Move::R2),
            "R'" | "R’" => Ok(Move::R3),
            "U" => Ok(Move::U),
            "U2" => Ok(Move::U2),
            "U'" | "U’" => Ok(Move::U3),
            "B" => Ok(Move::B),
            "B2" => Ok(Move::B2),
            "B'" | "B’" => Ok(Move::B3),
            "L" => Ok(Move::L),
            "L2" => Ok(Move::L2),
            "L'" | "L’" => Ok(Move::L3),
            "D" => Ok(Move::D),
            "D2" => Ok(Move::D2),
            "D'" | "D’" => Ok(Move::D3),
            _ => Err(()),
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
        };
        write!(f, "{s}")
    }
}
