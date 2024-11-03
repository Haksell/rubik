use crate::color::Color;
use crate::r#move::Move;
use crate::solvers::{iddfs, DFSAble};
use crate::Puzzle;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Pyraminx {
    pub faces: Vec<Color>,
}

impl Pyraminx {
    pub fn new() -> Pyraminx {
        const ORDER: [Color; 4] = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW];

        Pyraminx {
            faces: (0..ORDER.len() * 9).map(|i| ORDER[i / 9]).collect(),
        }
    }

    pub fn to_pyraminx3(&self) -> Result<Pyraminx, &'static str> {
        Ok(Pyraminx {
            faces: self.faces.clone(),
        })
    }

    fn get_face(&self, face: usize) -> &[Color] {
        let start = face * 9;
        let end = (face + 1) * 9;
        &self.faces[start..end]
    }
}

impl Puzzle for Pyraminx {
    fn do_move(&mut self, move_: Move) {
        // TODO
    }

    fn solve(&self) -> Option<Vec<Move>> {
        Some(iddfs(self.clone()))
    }

    // TODO Better check ?
    fn is_solved(&self) -> bool {
        const ORDER: [Color; 4] = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW];
        self.faces
            .iter()
            .enumerate()
            .all(|(i, &col)| col == ORDER[i / 9])
    }

    fn get_faces(&self) -> &Vec<Color> {
        &self.faces
    }
}

impl DFSAble for Pyraminx {
    const ALLOWED_MOVES: &'static [Move] = &[
        Move::R,
        Move::U,
        Move::B,
        Move::L,
        Move::R2,
        Move::U2,
        Move::B2,
        Move::L2,
    ];
}

impl Display for Pyraminx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn format(face: &[Color], line: usize) -> String {
            let start: usize = line * line;
            face[start..start + (line * 2 + 1)]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        }

        let faces: Vec<&[Color]> = vec![0, 1, 2]
            .into_iter()
            .map(|f| self.get_face(f))
            .collect();

        for line in 0..3 {
            writeln!(
                f,
                "{}{}",
                " ".repeat((2 - line) * 2).as_str(),
                faces
                    .iter()
                    .map(|face| format(face, line))
                    .collect::<Vec<String>>()
                    .join(" ".repeat((2 - line) * 4 + 1).as_str())
            )?;
        }

        let face = self.get_face(3);
        for line in 0..3 {
            writeln!(
                f,
                "{}{}{}",
                " ".repeat((3 + 3 - 1) * 2).as_str(),
                " ".repeat((line) * 2).as_str(),
                format(&face, 3 - line - 1)
            )?;
        }

        Ok(())
    }
}
