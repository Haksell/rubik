#[repr(u8)]
#[derive(Clone, Debug, Copy)]
pub enum Move {
    F, R, U, B, L, D,
	F2, R2, U2, B2, L2, D2,
    F3, R3, U3, B3, L3, D3,
}

impl Move {
    pub fn from_int(index: u8) -> Option<Move> {
        match index {
            0 => Some(Move::F),
            1 => Some(Move::R),
            2 => Some(Move::U),
            3 => Some(Move::B),
            4 => Some(Move::L),
            5 => Some(Move::D),
            6 => Some(Move::F2),
            7 => Some(Move::R2),
            8 => Some(Move::U2),
            9 => Some(Move::B2),
            10 => Some(Move::L2),
            11 => Some(Move::D2),
            12 => Some(Move::F3),
            13 => Some(Move::R3),
            14 => Some(Move::U3),
            15 => Some(Move::B3),
            16 => Some(Move::L3),
            17 => Some(Move::D3),
            _ => None,
        }
    }

    pub fn as_int(&self) -> u8 {
        *self as u8
    }

    #[allow(dead_code)]
    fn opposite(&self) -> Move {
        let mut i = self.as_int();
        if i < 6 {
            i += 12;
        } else if i > 11 {
            i %= 12;
        }
        Move::from_int(i).unwrap()
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