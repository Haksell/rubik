use crate::color::Color;
use crate::r#move::Move;
use colored::*;
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};
use std::hash::Hash;

// TODO: handle N > 3
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Cube<const N: usize> {
    pub faces: Vec<Color>,
}

// Always fronting Green face
impl<const N: usize> Cube<N> {
    pub fn new() -> Cube<N> {
        // TODO: one-liner
        // let mut faces = Vec::new();
        // for i in 0..6 * N * N {
        //     let face = i / (N * N);
        //     let color = (face as u8).try_into();
        //     faces.push(color.unwrap());
        // }
        Cube {
            faces: (0..6 * N * N)
                .map(|i| Color::try_from((i / (N * N)) as u8).unwrap())
                .collect(),
        }
    }

    // TODO: from_scramble

    // Kind of sucks, but we can't implement the same method for Cube<2> and Cube<N >= 3> without nightly
    pub fn is_solved(&self) -> bool {
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
            (0..6 * N * N).all(|i| self.faces[i] == Color::try_from((i / (N * N)) as u8).unwrap())
        }
    }

    pub fn do_move(&mut self, move_: Move) {
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

    pub fn scramble(&mut self, sequence: &str) {
        let as_moves = sequence.split_whitespace().map(Move::try_from);

        for mov in as_moves {
            match mov {
                Ok(m) => self.do_move(m),
                Err(_) => panic!("Invalid move in scramble sequence"),
            }
        }
    }

    pub fn rand_scramble(&mut self, iterations: usize) -> Vec<Move> {
        let mut sequence: Vec<Move> = Vec::new();

        while sequence.len() < iterations {
            let move_ = Move::random();
            if !sequence.is_empty() && move_ == sequence.last().unwrap().opposite() {
                continue;
            }
            self.do_move(move_);
            sequence.push(move_);
        }
        sequence
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

    pub fn adjency_faces(&self, face: Color) -> Vec<Color> {
        let opposite = self.opposite_face(face);
        (0..6)
            .map(|x| Color::try_from(x).unwrap())
            .filter(|f| *f != face && *f != opposite)
            .collect()
    }

    pub fn get_face(&self, face: Color) -> Vec<Color> {
        let start = face as usize * N * N;
        let end = (face as usize + 1) * N * N;
        self.faces[start..end].to_vec()
    }
}

impl<const N: usize> Display for Cube<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        //for i in 0..6 {
        //    println!("{:?}", &self.faces[i * N * N..(i + 1) * N * N]);
        //}

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

// TODO: impl Cube[Sticker]
// TODO: macro cub2! cub3! cub4! cub5! cub6! cub7!

#[cfg(test)]
mod tests {
    use super::Cube;
    use crate::r#move::Move;

    #[test]
    fn test_is_solved_generic() {
        let mut cube = Cube::<3>::new();
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
        let mut cube = Cube::<3>::new();
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
        let mut cube = Cube::<2>::new();
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
