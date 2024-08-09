use crate::r#move::Move;
use colored::*;
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};
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

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Cube {
    pub faces: Vec<Color>,
    pub size: usize,
}

// Always fronting Green face
impl Cube {
    pub fn new(n: usize) -> Cube {
        println!("New Cube !");
        let mut faces = Vec::new();
        for i in 0..6 * n * n {
            let face = i / (n * n);
            let color = (face as u8).try_into();
            faces.push(color.unwrap());
        }
        Cube { faces, size: n }
    }

    pub fn do_move(&mut self, _move: Move) {
        //println!("{:?}", _move);
        match _move {
            Move::F => {
                // Swap White & Red
                let white_start = Color::WHITE as usize * self.size * self.size;
                let red_start = Color::RED as usize * self.size * self.size;
                for i in 0..self.size {
                    let white_idx = white_start + (self.size - 1) * self.size + i;
                    let red_idx = red_start + self.size * i;
                    self.faces.swap(white_idx, red_idx);
                }

                // Swap White & Yellow
                let yellow_start = Color::YELLOW as usize * self.size * self.size;
                for i in 0..self.size {
                    let white_idx = white_start + (self.size - 1) * self.size + i;
                    let yellow_idx = yellow_start + self.size - i - 1;
                    self.faces.swap(white_idx, yellow_idx);

                }

                // Swap White & Orange
                let orange_start = Color::ORANGE as usize * self.size * self.size;
                for i in 0..self.size {
                    let white_idx = white_start + (self.size - 1) * self.size + i;
                    let orange_idx = orange_start + self.size * (self.size - i) - 1;
                    self.faces.swap(white_idx, orange_idx);
                }

                self.rotate_clockwise(Color::GREEN);
            }
            Move::F2 => {
                for _ in 0..2 {
                    self.do_move(Move::F);
                }
            }
            Move::F3 => {
                for _ in 0..3 {
                    self.do_move(Move::F);
                }
            }
            Move::R => {
                // Swap Green & White
                let green_start = Color::GREEN as usize * self.size * self.size;
                let white_start = Color::WHITE as usize * self.size * self.size;
                for i in 0..self.size {
                    let green_idx = green_start + self.size * (i + 1) - 1;
                    let white_idx = white_start + self.size * (i + 1) - 1;
                    self.faces.swap(green_idx, white_idx);
                }

                // Swap Green & Blue
                let blue_start = Color::BLUE as usize * self.size * self.size;
                for i in 0..self.size {
                    let green_idx = green_start + self.size * (i + 1) - 1;
                    let blue_idx = blue_start + self.size * (self.size - i - 1);
                    self.faces.swap(green_idx, blue_idx);
                }

                // Swap Green & Yellow
                let yellow_start = Color::YELLOW as usize * self.size * self.size;
                for i in 0..self.size {
                    let green_idx = green_start + self.size * (i + 1) - 1;
                    let yellow_idx = yellow_start + self.size * (i + 1) - 1;
                    self.faces.swap(green_idx, yellow_idx);
                }

                self.rotate_clockwise(Color::RED);
            }
            Move::R2 => {
                for _ in 0..2 {
                    self.do_move(Move::R);
                }
            }
            Move::R3 => {
                for _ in 0..3 {
                    self.do_move(Move::R);
                }
            }
            Move::U => {
                // Swap Green & Orange
                let green_start = Color::GREEN as usize * self.size * self.size;
                let orange_start = Color::ORANGE as usize * self.size * self.size;
                for i in 0..self.size {
                    let green_idx = green_start + i;
                    let orange_idx = orange_start + i;
                    self.faces.swap(green_idx, orange_idx);
                }

                // Swap Green & Blue
                let blue_start = Color::BLUE as usize * self.size * self.size;
                for i in 0..self.size {
                    let green_idx = green_start + i;
                    let blue_idx = blue_start + i;
                    self.faces.swap(green_idx, blue_idx);
                }

                // Swap Green & Red
                let red_start = Color::RED as usize * self.size * self.size;
                for i in 0..self.size {
                    let green_idx = green_start + i;
                    let red_idx = red_start + i;
                    self.faces.swap(green_idx, red_idx);
                }

                self.rotate_clockwise(Color::WHITE);
            }
            Move::U2 => {
                for _ in 0..2 {
                    self.do_move(Move::U);
                }
            }
            Move::U3 => {
                for _ in 0..3 {
                    self.do_move(Move::U);
                }
            }
            Move::B => {
                // Swap White & Orange
                let white_start = Color::WHITE as usize * self.size * self.size;
                let orange_start = Color::ORANGE as usize * self.size * self.size;
                for i in 0..self.size {
                    let white_idx = white_start + i;
                    let orange_idx = orange_start + self.size * (self.size - i - 1);
                    self.faces.swap(white_idx, orange_idx);
                }

                // Swap White & Yellow
                let yellow_start = Color::YELLOW as usize * self.size * self.size;
                for i in 0..self.size {
                    let white_idx = white_start + i;
                    let yellow_idx = yellow_start + self.size * self.size - i - 1;
                    self.faces.swap(white_idx, yellow_idx);
                }

                // Swap White & Red
                let red_start = Color::RED as usize * self.size * self.size;
                for i in 0..self.size {
                    let white_idx = white_start + i;
                    let red_idx = red_start + self.size * (i + 1) - 1;
                    self.faces.swap(white_idx, red_idx);
                }

                self.rotate_clockwise(Color::BLUE);
            }
            Move::B2 => {
                for _ in 0..2 {
                    self.do_move(Move::B);
                }
            }
            Move::B3 => {
                for _ in 0..3 {
                    self.do_move(Move::B);
                }
            }
            Move::L => {
                // Swap Green & Yellow
                let green_start = Color::GREEN as usize * self.size * self.size;
                let yellow_start = Color::YELLOW as usize * self.size * self.size;
                for i in 0..self.size {
                    let green_idx = green_start + self.size * i;
                    let yellow_idx = yellow_start + self.size * i;
                    self.faces.swap(green_idx, yellow_idx);
                }

                // Swap Green & Blue
                let blue_start = Color::BLUE as usize * self.size * self.size;
                for i in 0..self.size {
                    let green_idx = green_start + self.size * i;
                    let blue_idx = blue_start + self.size * (self.size - i) - 1;
                    self.faces.swap(green_idx, blue_idx);
                }

                // Swap Green & White
                let white_start = Color::WHITE as usize * self.size * self.size;
                for i in 0..self.size {
                    let green_idx = green_start + self.size * i;
                    let white_idx = white_start + self.size * i;
                    self.faces.swap(green_idx, white_idx);
                }

                self.rotate_clockwise(Color::ORANGE);
            }
            Move::L2 => {
                for _ in 0..2 {
                    self.do_move(Move::L);
                }
            }
            Move::L3 => {
                for _ in 0..3 {
                    self.do_move(Move::L);
                }
            }
            Move::D => {
                // Swap Green & Red
                let green_start = Color::GREEN as usize * self.size * self.size;
                let red_start = Color::RED as usize * self.size * self.size;
                for i in 0..self.size {
                    let green_idx = green_start + self.size * self.size - i - 1;
                    let red_idx = red_start + self.size * self.size - i - 1;
                    self.faces.swap(green_idx, red_idx);
                }

                // Swap Green & Blue
                let blue_start = Color::BLUE as usize * self.size * self.size;
                for i in 0..self.size {
                    let green_idx = green_start + self.size * self.size - i - 1;
                    let blue_idx = blue_start + self.size * self.size - i - 1;
                    self.faces.swap(green_idx, blue_idx);
                }

                // Swap Green & Orange
                let orange_start = Color::ORANGE as usize * self.size * self.size;
                for i in 0..self.size {
                    let green_idx = green_start + self.size * self.size - i - 1;
                    let orange_idx = orange_start + self.size * self.size - i - 1;
                    self.faces.swap(green_idx, orange_idx);
                }

                self.rotate_clockwise(Color::YELLOW);
            }
            Move::D2 => {
                for _ in 0..2 {
                    self.do_move(Move::D);
                }
            }
            Move::D3 => {
                for _ in 0..3 {
                    self.do_move(Move::D);
                }
            }
        }
        // println!("{:?}", self.faces);
        //println!("{}", self);
    }

    pub fn scramble(&mut self, sequence: &str) -> Result<(), Error> {
        let as_moves = sequence.split_whitespace().map(Move::try_from);

        for mov in as_moves {
            match mov {
                Ok(m) => self.do_move(m),
                Err(_) => return Err(Error),
            }
        }

        Ok(())
    }

    pub fn opposite_face(&self, face: Color) -> Color {
        match face {
            Color::WHITE => Color::YELLOW,
            Color::RED => Color::ORANGE,
            Color::GREEN => Color::BLUE,
            Color::YELLOW => Color::WHITE,
            Color::ORANGE => Color::RED,
            Color::BLUE => Color::GREEN,
        }
    }

    fn rotate_clockwise(&mut self, face: Color) {
        // Transpose
        let start = face as usize * self.size * self.size;
        for y in 0..self.size {
            for x in y + 1..self.size {
                self.faces.swap(start + y * self.size + x, start + x * self.size + y);
            }
        }

        // Reverse rows
        for y in 0..self.size {
            let start = start + y * self.size;
            let end = start + self.size;
            self.faces[start..end].reverse();
        }
    }

    pub fn adjency_faces(&self, face: Color) -> Vec<Color> {
        let opposite = self.opposite_face(face);
        (0..6)
            .map(|x| Color::try_from(x).unwrap())
            .filter(|f| *f != face && *f != opposite)
            .collect()
    }

    pub fn get_face(&self, face: Color) -> Vec<Color> {
        let start = face as usize * self.size * self.size;
        let end = (face as usize + 1) * self.size * self.size;
        self.faces[start..end].to_vec()
    }

    // TODO Fix
    #[allow(dead_code)]
    fn get_face_mut(&mut self, face: Color) -> &mut [Color] {
        let start = face as usize * self.size * self.size;
        let end = (face as usize + 1) * self.size * self.size;
        &mut self.faces[start..end]
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        fn colored(color: &Color) -> String {
            match color {
                Color::WHITE => "■".truecolor(0xff, 0xff, 0xff),
                Color::RED => "■".truecolor(0xff, 0x12, 0x34),
                Color::GREEN => "■".truecolor(0x00, 0x9b, 0x48),
                Color::YELLOW => "■".truecolor(0xff, 0xd5, 0x00),
                Color::ORANGE => "■".truecolor(0xff, 0x58, 0x00),
                Color::BLUE => "■".truecolor(0x00, 0x46, 0xad),
            }
            .to_string()
        }

        fn format(face: &Vec<Color>, size: usize, line: usize) -> String {
            face[line * size..(line + 1) * size]
                .iter()
                .map(|c| colored(c))
                .collect::<Vec<_>>()
                .join(" ")
        }

        let face = self.get_face(Color::WHITE);
        for line in 0..self.size {
            writeln!(
                f,
                "{}{}",
                " ".repeat(self.size * 2),
                format(&face, self.size, line)
            )?;
        }

        let faces: Vec<Vec<Color>> = vec![4, 2, 1, 5]
            .into_iter()
            .map(|f| self.get_face(Color::try_from(f).unwrap()))
            .collect();

        for line in 0..self.size {
            writeln!(
                f,
                "{}",
                faces
                    .iter()
                    .map(|face| format(face, self.size, line))
                    .collect::<Vec<String>>()
                    .join(" ")
            )?;
        }

        let face = self.get_face(Color::YELLOW);
        for line in 0..self.size {
            writeln!(
                f,
                "{}{}",
                " ".repeat(self.size * 2),
                format(&face, self.size, line)
            )?;
        }

        Ok(())
    }
}
