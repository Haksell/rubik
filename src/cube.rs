use crate::r#move::Move;
use colored::*;
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};

#[repr(u8)]
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
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

pub struct Cube {
    pub faces: Vec<Color>,
    size: usize,
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

    // TODO
    pub fn do_move(&mut self, _move: Move) {
        println!("{:?}", _move);
        match _move {
            Move::F => {
                // let adj = self.adjency_faces(Color::GREEN);

                // let start = Color::WHITE as usize * self.size * self.size;
                // let end = (Color::WHITE as usize + 1) * self.size * self.size;
                // let white: &mut [Color] = &mut self.faces[start..end];

                // println!("White: {:?}", white);
                // for i in self.size * (self.size - 1)..self.size * self.size {
                //     white[i] = Color::RED;
                // }
                // println!("White: {:?}", white);
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
            _ => (),
        }
        println!("{:?}", self.faces);
        println!("{}", self);
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
    fn get_face_mut(&mut self, face: Color) -> &mut [Color] {
        let start = face as usize * self.size * self.size;
        let end = (face as usize + 1) * self.size * self.size;
        &mut self.faces[start..end]
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        fn colored(color: Color) -> String {
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
                .map(|c| colored(*c))
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
