use std::convert::TryFrom;
use std::fmt::Display;
use std::hash::Hash;

use colored::Colorize;

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
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::WHITE),
            1 => Ok(Color::RED),
            2 => Ok(Color::GREEN),
            3 => Ok(Color::YELLOW),
            4 => Ok(Color::ORANGE),
            5 => Ok(Color::BLUE),
            _ => Err("Colors are from 0 to 5 included"),
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

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Color::WHITE => "■".truecolor(0xff, 0xff, 0xff),
                Color::RED => "■".truecolor(0xff, 0x12, 0x34),
                Color::GREEN => "■".truecolor(0x00, 0x9b, 0x48),
                Color::YELLOW => "■".truecolor(0xff, 0xd5, 0x00),
                Color::ORANGE => "■".truecolor(0xff, 0x58, 0x00),
                Color::BLUE => "■".truecolor(0x00, 0x46, 0xad),
            }
        )
    }
}
