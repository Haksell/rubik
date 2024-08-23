use crate::color::Color;
use crate::r#move::Move;
use crate::trigger::Trigger;
use crate::Puzzle;
use colored::*;
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};
use std::hash::Hash;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Pyraminx {
    pub faces: Vec<Color>,
}

impl Pyraminx {
    pub fn new() -> Pyraminx {
        const N: usize = 3;
        Pyraminx {
            faces: (0..4 * N * N)
                .map(|i| Color::try_from((i / (N * N)) as u8).unwrap())
                .collect(),
        }
    }

    pub fn get_face(&self, face: Color) -> Vec<Color> {
        const N: usize = 3;
        let start = face as usize * N * N;
        let end = (face as usize + 1) * N * N;
        self.faces[start..end].to_vec()
    }
}

impl Puzzle for Pyraminx {
    fn do_move(&mut self, move_: Move) {
        todo!()
    }
}

impl Display for Pyraminx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const N: usize = 3;

        todo!();

        fn format(face: &[Color], size: usize, line: usize) -> String {
            face[line * size..(line + 1) * size]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        }

        let face = self.get_face(Color::WHITE);
        for line in 0..N {
            writeln!(f, "{}{}", " ".repeat(N * 2), format(&face, N, line))?;
        }

        let faces: Vec<Vec<Color>> = vec![4, 2, 1, 5]
            .into_iter()
            .map(|f| self.get_face(Color::try_from(f).unwrap()))
            .collect();

        for line in 0..N {
            writeln!(
                f,
                "{}",
                faces
                    .iter()
                    .map(|face| format(face, N, line))
                    .collect::<Vec<String>>()
                    .join(" ")
            )?;
        }

        let face = self.get_face(Color::YELLOW);
        for line in 0..N {
            writeln!(f, "{}{}", " ".repeat(N * 2), format(&face, N, line))?;
        }

        Ok(())
    }
}
