use std::convert::TryFrom;
use std::hash::Hash;

#[repr(u8)]
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    WHITE,
    RED,
    GREEN,
    YELLOW,
    ORANGE,
    BLUE,
}

impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::WHITE),
            1 => Ok(Color::RED),
            2 => Ok(Color::GREEN),
            3 => Ok(Color::YELLOW),
            4 => Ok(Color::ORANGE),
            5 => Ok(Color::BLUE),
            _ => Err(()),
        }
    }
}

impl Color {
    // TODO: better name
    pub fn side(&self) -> i8 {
        match self {
            Color::BLUE => 0,
            Color::RED => 1,
            Color::GREEN => 2,
            Color::ORANGE => 3,
            _ => unimplemented!(),
        }
    }
}
