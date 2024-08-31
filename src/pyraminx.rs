use crate::color::Color;
use crate::r#move::Move;
use crate::solvers::{iddfs, DFSAble};
use crate::Puzzle;
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
            Move::R => todo!(),
            Move::U => {
                let idxs: Vec<usize> = (0..N * N - (N + N - 1)).collect();

                // Face 1 to face 0

                idxs.iter().for_each(|&i| self.faces.swap(N * N + i, i));

                // Face 1 to face 2
                idxs.iter()
                    .for_each(|&i| self.faces.swap(N * N + i, N * N * 2 + i));
            }
            Move::B => todo!(),
            Move::L => todo!(),
            Move::R2 => {
                for _ in 0..2 {
                    self.do_move(Move::R);
                }
            }
            Move::U2 => {
                for _ in 0..2 {
                    self.do_move(Move::U);
                }
            }
            Move::B2 => {
                for _ in 0..2 {
                    self.do_move(Move::B);
                }
            }
            Move::L2 => {
                for _ in 0..2 {
                    self.do_move(Move::L);
                }
            }
            _ => panic!("Invalid move for pyraminx: '{:?}'", move_), // TODO Or maybe ignore ?
        };
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

    fn get_faces(&self) -> &Vec<Color> {
        &self.faces
    }
}

impl DFSAble for Pyraminx<2> {
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
impl DFSAble for Pyraminx<3> {
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
