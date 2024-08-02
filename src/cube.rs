use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Error};
use crate::r#move::Move;

#[repr(u8)]
#[derive(Clone, Debug, Copy)]
pub enum Color {
    WHITE,
    YELLOW,
    RED,
    GREEN,
    BLUE,
    ORANGE,
}

impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::WHITE),
            1 => Ok(Color::YELLOW),
            2 => Ok(Color::RED),
            3 => Ok(Color::GREEN),
            4 => Ok(Color::BLUE),
            5 => Ok(Color::ORANGE),
            _ => Err(()),
        }
    }
}

pub struct Cube {
    pub faces: Vec<Color>,
    size: usize,
}

impl Cube {
    pub fn new(n: usize) -> Cube {
        println!("New Cube !");
        let mut faces = Vec::new();
        for i in 0..6 * n * n {
            let face = i / (n * n);
            // let _y = (i % (n * n)) / n;
            // let _x = (i % (n * n)) % n;
            let color = (face as u8).try_into();
            faces.push(color.unwrap());
        }
        Cube { faces, size: n }
    }

    // TODO
    pub fn do_move(&self, _move: Move) {
        println!("{:?}", _move);
    }

    pub fn scramble(&self, sequence: &str) -> Result<(), Error> {
        let as_moves = sequence.split_whitespace().map(Move::try_from);

        for mov in as_moves {
            match mov {
                Ok(m) => self.do_move(m),
                Err(_) => return Err(Error),
            }
        }

        Ok(())
    }

	pub fn get_face(&self, face: u8) -> Vec<Color> {
        let start = face as usize * self.size * self.size;
        let end = (face as usize + 1) * self.size * self.size;
        self.faces[start..end].to_vec()
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		for i in 0..6 {
			let face = self.get_face(i);
			writeln!(f, "Face {}\n {:?}", i, face)?;
		}

		Ok(())
		
        // self.faces
        //     .iter()
        //     .try_for_each(|faces| write!(f, "{:?}", faces))
    }
}