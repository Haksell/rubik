use colored::Colorize;
use std::convert::TryFrom;
use std::fmt::Display;
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
    // TODO: specific for Cube<3>
    pub fn side(&self) -> i8 {
        match self {
            Color::BLUE => 0,
            Color::RED => 1,
            Color::GREEN => 2,
            Color::ORANGE => 3,
            _ => unimplemented!(),
        }
    }

    pub fn as_rgb(&self) -> [f32; 3] {
        match self {
            Color::WHITE => [1.0, 1.0, 1.0],
            Color::RED => [1.0, 0.071, 0.204],
            Color::GREEN => [0.0, 0.608, 0.282],
            Color::YELLOW => [1.0, 0.835, 0.0],
            Color::ORANGE => [1.0, 0.345, 0.0],
            Color::BLUE => [0.0, 0.275, 0.678],
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let [r, g, b] = self.as_rgb();
        write!(
            f,
            "{}",
            "■".truecolor(
                (r * 255.0).round() as u8,
                (g * 255.0).round() as u8,
                (b * 255.0).round() as u8
            ),
        )
    }
}
