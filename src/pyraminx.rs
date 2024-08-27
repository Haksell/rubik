use move_macro::moves;

use crate::color::Color;
use crate::r#move::Move;
use crate::solvers::{iddfs, DFSAble};
use crate::Puzzle;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Pyraminx<const N: usize> {
    pub faces: Vec<Color>,
}

impl<const N: usize> Pyraminx<N> {
    pub fn new() -> Pyraminx<N> {
        const ORDER: [Color; 4] = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW];

        Pyraminx {
            faces: (0..ORDER.len() * N * N)
                .map(|i| ORDER[i / (N * N)])
                .collect(),
        }
    }

    pub fn to_pyraminx2(&self) -> Result<Pyraminx<2>, &'static str> {
        if N == 2 {
            Ok(Pyraminx {
                faces: self.faces.clone(),
            })
        } else {
            Err("Cannot convert Pyraminx<N> to Pyraminx<3>: N is not 3")
        }
    }

    pub fn to_pyraminx3(&self) -> Result<Pyraminx<3>, &'static str> {
        if N == 3 {
            Ok(Pyraminx {
                faces: self.faces.clone(),
            })
        } else {
            Err("Cannot convert Pyraminx<N> to Pyraminx<3>: N is not 3")
        }
    }

    fn get_face(&self, face: usize) -> &[Color] {
        let start = face * N * N;
        let end = (face + 1) * N * N;
        &self.faces[start..end]
    }
}

impl<const N: usize> Puzzle for Pyraminx<N> {
    fn do_move(&mut self, move_: Move) {
        match move_ {
            Move::F => todo!(),
            Move::R => todo!(),
            Move::U => todo!(),
            Move::B => todo!(),
            Move::L => todo!(),
            Move::D => todo!(),
            Move::F2 => todo!(),
            Move::R2 => todo!(),
            Move::U2 => todo!(),
            Move::B2 => todo!(),
            Move::L2 => todo!(),
            Move::D2 => todo!(),
            _ => panic!("Invalid move for pyraminx: '{:?}'", move_), // TODO Or maybe ignore ?
        }
    }

    fn allowed_moves(&self) -> Vec<Move> {
        moves!("R L U B R2 L2 U2 B2")
    }

    fn solve(&self) -> Option<Vec<Move>> {
        match N {
            2 => Some(iddfs(self.to_pyraminx2().unwrap())),
            3 => Some(iddfs(self.to_pyraminx3().unwrap())),
            _ => None,
        }
    }

    // TODO Better check ?
    fn is_solved(&self) -> bool {
        const ORDER: [Color; 4] = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW];
        self.faces
            .iter()
            .enumerate()
            .all(|(i, &col)| col == ORDER[i / (N * N)])
    }
}

impl DFSAble for Pyraminx<2> {}
impl DFSAble for Pyraminx<3> {}

impl<const N: usize> Display for Pyraminx<N> {
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

        for line in 0..N {
            writeln!(
                f,
                "{}{}",
                " ".repeat((N - line - 1) * 2).as_str(),
                faces
                    .iter()
                    .map(|face| format(face, line))
                    .collect::<Vec<String>>()
                    .join(" ".repeat((N - line - 1) * 4 + 1).as_str())
            )?;
        }

        let face = self.get_face(3);
        for line in 0..N {
            writeln!(
                f,
                "{}{}{}",
                " ".repeat((N + N - 1) * 2).as_str(),
                " ".repeat((line) * 2).as_str(),
                format(&face, N - line - 1)
            )?;
        }

        Ok(())
    }
}
