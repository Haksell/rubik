use crate::color::Color;
use crate::r#move::Move;
use crate::solvers::{cfop, iddfs, premover, zz, DFSAble};
use crate::trigger::Trigger;
use crate::Puzzle;
use std::fmt::{Display, Error, Formatter};
use std::hash::Hash;

// TODO: handle N > 3
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Cube<const N: usize> {
    pub faces: Vec<Color>,
}

// TODO: remove dummy PartialOrd and Ord

impl PartialOrd for Cube<3> {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

impl Ord for Cube<3> {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

// TODO: cub4!, cub5!, cub6!, cub7! with macro of macros

#[macro_export]
macro_rules! cub2 {
    () => {
        Cube::<2>::new()
    };
}

#[macro_export]
macro_rules! cub3 {
    () => {
        Cube::<3>::new()
    };
}

// Always fronting Green face
impl<const N: usize> Cube<N> {
    pub fn new() -> Cube<N> {
        const ORDER: [Color; 6] = [
            Color::WHITE,
            Color::RED,
            Color::GREEN,
            Color::YELLOW,
            Color::ORANGE,
            Color::BLUE,
        ];

        Cube {
            faces: (0..ORDER.len() * N * N)
                .map(|i| ORDER[i / (N * N)])
                .collect(),
        }
    }

    pub fn to_cube2(&self) -> Result<Cube<2>, &'static str> {
        if N == 2 {
            Ok(Cube {
                faces: self.faces.clone(),
            })
        } else {
            Err("Cannot convert Cube<N> to Cube<3>: N is not 3")
        }
    }

    pub fn to_cube3(&self) -> Result<Cube<3>, &'static str> {
        if N == 3 {
            Ok(Cube {
                faces: self.faces.clone(),
            })
        } else {
            Err("Cannot convert Cube<N> to Cube<3>: N is not 3")
        }
    }

    // TODO: from_scramble

    fn rotate_clockwise(&mut self, face: Color) {
        // Transpose
        let start = face as usize * N * N;
        for y in 0..N {
            for x in y + 1..N {
                self.faces.swap(start + y * N + x, start + x * N + y);
            }
        }

        // Reverse rows
        for y in 0..N {
            let start = start + y * N;
            let end = start + N;
            self.faces[start..end].reverse();
        }
    }

    fn get_face(&self, face: usize) -> &[Color] {
        let start = face as usize * N * N;
        let end = (face as usize + 1) * N * N;
        &self.faces[start..end]
    }
}

impl Cube<3> {
    // TODO: optimize
    pub fn do_trigger(&mut self, trigger: Trigger) {
        for move_ in trigger.moves() {
            self.do_move(move_);
        }
    }
}

impl<const N: usize> Puzzle for Cube<N> {
    fn do_move(&mut self, move_: Move) {
        // TODO: N+1 assignments instead of 2N with Vec::swap
        // TODO: Implement double and prime moves without loops
        match move_ {
            Move::F => {
                // Swap White & Red
                let white_start = Color::WHITE as usize * N * N;
                let red_start = Color::RED as usize * N * N;
                for i in 0..N {
                    let white_idx = white_start + (N - 1) * N + i;
                    let red_idx = red_start + N * i;
                    self.faces.swap(white_idx, red_idx);
                }

                // Swap White & Yellow
                let yellow_start = Color::YELLOW as usize * N * N;
                for i in 0..N {
                    let white_idx = white_start + (N - 1) * N + i;
                    let yellow_idx = yellow_start + N - i - 1;
                    self.faces.swap(white_idx, yellow_idx);
                }

                // Swap White & Orange
                let orange_start = Color::ORANGE as usize * N * N;
                for i in 0..N {
                    let white_idx = white_start + (N - 1) * N + i;
                    let orange_idx = orange_start + N * (N - i) - 1;
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
                let green_start = Color::GREEN as usize * N * N;
                let white_start = Color::WHITE as usize * N * N;
                for i in 0..N {
                    let green_idx = green_start + N * (i + 1) - 1;
                    let white_idx = white_start + N * (i + 1) - 1;
                    self.faces.swap(green_idx, white_idx);
                }

                // Swap Green & Blue
                let blue_start = Color::BLUE as usize * N * N;
                for i in 0..N {
                    let green_idx = green_start + N * (i + 1) - 1;
                    let blue_idx = blue_start + N * (N - i - 1);
                    self.faces.swap(green_idx, blue_idx);
                }

                // Swap Green & Yellow
                let yellow_start = Color::YELLOW as usize * N * N;
                for i in 0..N {
                    let green_idx = green_start + N * (i + 1) - 1;
                    let yellow_idx = yellow_start + N * (i + 1) - 1;
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
                let green_start = Color::GREEN as usize * N * N;
                let orange_start = Color::ORANGE as usize * N * N;
                for i in 0..N {
                    let green_idx = green_start + i;
                    let orange_idx = orange_start + i;
                    self.faces.swap(green_idx, orange_idx);
                }

                // Swap Green & Blue
                let blue_start = Color::BLUE as usize * N * N;
                for i in 0..N {
                    let green_idx = green_start + i;
                    let blue_idx = blue_start + i;
                    self.faces.swap(green_idx, blue_idx);
                }

                // Swap Green & Red
                let red_start = Color::RED as usize * N * N;
                for i in 0..N {
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
                let white_start = Color::WHITE as usize * N * N;
                let orange_start = Color::ORANGE as usize * N * N;
                for i in 0..N {
                    let white_idx = white_start + i;
                    let orange_idx = orange_start + N * (N - i - 1);
                    self.faces.swap(white_idx, orange_idx);
                }

                // Swap White & Yellow
                let yellow_start = Color::YELLOW as usize * N * N;
                for i in 0..N {
                    let white_idx = white_start + i;
                    let yellow_idx = yellow_start + N * N - i - 1;
                    self.faces.swap(white_idx, yellow_idx);
                }

                // Swap White & Red
                let red_start = Color::RED as usize * N * N;
                for i in 0..N {
                    let white_idx = white_start + i;
                    let red_idx = red_start + N * (i + 1) - 1;
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
                let green_start = Color::GREEN as usize * N * N;
                let yellow_start = Color::YELLOW as usize * N * N;
                for i in 0..N {
                    let green_idx = green_start + N * i;
                    let yellow_idx = yellow_start + N * i;
                    self.faces.swap(green_idx, yellow_idx);
                }

                // Swap Green & Blue
                let blue_start = Color::BLUE as usize * N * N;
                for i in 0..N {
                    let green_idx = green_start + N * i;
                    let blue_idx = blue_start + N * (N - i) - 1;
                    self.faces.swap(green_idx, blue_idx);
                }

                // Swap Green & White
                let white_start = Color::WHITE as usize * N * N;
                for i in 0..N {
                    let green_idx = green_start + N * i;
                    let white_idx = white_start + N * i;
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
                let green_start = Color::GREEN as usize * N * N;
                let red_start = Color::RED as usize * N * N;
                for i in 0..N {
                    let green_idx = green_start + N * N - i - 1;
                    let red_idx = red_start + N * N - i - 1;
                    self.faces.swap(green_idx, red_idx);
                }

                // Swap Green & Blue
                let blue_start = Color::BLUE as usize * N * N;
                for i in 0..N {
                    let green_idx = green_start + N * N - i - 1;
                    let blue_idx = blue_start + N * N - i - 1;
                    self.faces.swap(green_idx, blue_idx);
                }

                // Swap Green & Orange
                let orange_start = Color::ORANGE as usize * N * N;
                for i in 0..N {
                    let green_idx = green_start + N * N - i - 1;
                    let orange_idx = orange_start + N * N - i - 1;
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

    fn solve(&self) -> Option<Vec<Move>> {
        match N {
            2 => Some(iddfs(self.to_cube2().unwrap())),
            3 => Some(premover(&mut self.to_cube3().unwrap(), zz)),
            _ => None,
        }
    }

    // Kind of sucks, but we can't implement the same method for Cube<2> and Cube<N >= 3> without nightly
    fn is_solved(&self) -> bool {
        if N == 2 {
            self.faces[16] == self.faces[18]
                && self.faces[17] == self.faces[18]
                && self.faces[19] == self.faces[18]
                && self.faces[20] == self.faces[23]
                && self.faces[21] == self.faces[23]
                && self.faces[22] == self.faces[23]
                && self.faces[12] == self.faces[14]
                && self.faces[13] == self.faces[14]
                && self.faces[15] == self.faces[14]
        } else {
            const ORDER: [Color; 6] = [
                Color::WHITE,
                Color::RED,
                Color::GREEN,
                Color::YELLOW,
                Color::ORANGE,
                Color::BLUE,
            ];
            self.faces
                .iter()
                .enumerate()
                .all(|(i, &col)| col == ORDER[i / (N * N)])
        }
    }

    fn get_faces(&self) -> &Vec<Color> {
        &self.faces
    }
}

impl<const N: usize> Display for Cube<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        fn format(face: &[Color], size: usize, line: usize) -> String {
            face[line * size..(line + 1) * size]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        }

        let face = self.get_face(0);
        for line in 0..N {
            writeln!(f, "{}{}", " ".repeat(N * 2), format(&face, N, line))?;
        }

        let faces: Vec<&[Color]> = vec![4, 2, 1, 5]
            .into_iter()
            .map(|f| self.get_face(f))
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

        let face = self.get_face(3);
        for line in 0..N {
            writeln!(f, "{}{}", " ".repeat(N * 2), format(&face, N, line))?;
        }

        Ok(())
    }
}

impl DFSAble for Cube<2> {
    const ALLOWED_MOVES: &'static [Move] = &[
        Move::R,
        Move::R2,
        Move::R3,
        Move::F,
        Move::F2,
        Move::F3,
        Move::U,
        Move::U2,
        Move::U3,
    ];
}

// TODO: impl Cube[Sticker]

#[cfg(test)]
mod tests {
    use super::Cube;
    use crate::{r#move::Move, Puzzle};

    #[test]
    fn test_is_solved_generic() {
        let mut cube = cub3!();
        // TODO: better loop iterator
        for &move_ in &[Move::U, Move::L, Move::F, Move::R, Move::B, Move::D] {
            assert!(cube.is_solved());
            cube.do_move(move_);
            assert!(!cube.is_solved());
            cube.do_move(move_);
            assert!(!cube.is_solved());
            cube.do_move(move_);
            assert!(!cube.is_solved());
            cube.do_move(move_);
            assert!(cube.is_solved());
        }
    }

    #[test]
    fn test_is_solved_sexy_moves() {
        let mut cube = cub3!();
        for i in 0..6 {
            cube.do_move(Move::R);
            cube.do_move(Move::U);
            cube.do_move(Move::R3);
            cube.do_move(Move::U3);
            assert!((i == 5) == cube.is_solved());
        }
    }

    #[test]
    fn test_is_solved_2x2x2() {
        let mut cube = cub2!();
        assert!(cube.is_solved());
        cube.do_move(Move::R);
        assert!(!cube.is_solved());
        cube.do_move(Move::R3);
        assert!(cube.is_solved());
        cube.do_move(Move::F);
        assert!(!cube.is_solved());
        cube.do_move(Move::B3);
        assert!(cube.is_solved());
        cube.do_move(Move::L2);
        assert!(!cube.is_solved());
        cube.do_move(Move::R2);
        assert!(cube.is_solved());
    }
}
