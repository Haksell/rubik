mod cube;
mod r#move;
mod pyraminx;

pub use cube::Cube;
pub use pyraminx::Pyraminx;
pub use r#move::{Move, MOVES, MOVES_RU, MOVES_RUL};

use crate::{color::Color, cub2, cub3, visualizer::Drawable};
use clap::ValueEnum;
use std::fmt::{self, Display};

pub trait Puzzle: Display + Drawable {
    fn solve(&self) -> Option<Vec<Move>>;

    fn is_solved(&self) -> bool;

    fn get_faces(&self) -> &Vec<Color>;

    fn do_move(&mut self, move_: Move);

    fn scramble(&mut self, sequence: &str) {
        sequence
            .split_whitespace()
            .map(|s| crate::puzzles::r#move::Move::try_from(s).unwrap())
            .for_each(|move_| self.do_move(move_));
    }

    fn rand_scramble(&mut self, iterations: usize) -> Vec<Move> {
        // TODO Better scrambler
        let mut sequence: Vec<Move> = Vec::new();

        while sequence.len() < iterations {
            let move_ = Move::random();
            if !sequence.is_empty() && move_.same_face(sequence.last().unwrap()) {
                continue;
            }
            self.do_move(move_);
            sequence.push(move_);
        }
        sequence
    }
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum PuzzleArg {
    Cube2,
    Cube3,
    Pyraminx,
}

impl PuzzleArg {
    pub fn build(&self) -> Box<dyn Puzzle> {
        match self {
            PuzzleArg::Cube2 => Box::new(cub2!()),
            PuzzleArg::Cube3 => Box::new(cub3!()),
            PuzzleArg::Pyraminx => Box::new(Pyraminx::new()),
        }
    }
}

impl fmt::Display for PuzzleArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PuzzleArg::Cube2 => write!(f, "cube2"),
            PuzzleArg::Cube3 => write!(f, "cube3"),
            PuzzleArg::Pyraminx => write!(f, "pyraminx"),
        }
    }
}
