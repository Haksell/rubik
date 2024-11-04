mod cube;
mod pyraminx;

pub use cube::Cube;
pub use pyraminx::Pyraminx;

use crate::{color::Color, cub2, cub3, moves_runtime, r#move::Move};
use clap::ValueEnum;
use kiss3d::{camera::ArcBall, scene::SceneNode, window::Window};
use std::fmt::Display;

pub trait Puzzle: Display {
    fn solve(&self) -> Option<Vec<Move>>;

    fn is_solved(&self) -> bool;

    fn get_faces(&self) -> &Vec<Color>;

    fn do_move(&mut self, move_: Move);

    fn scramble(&mut self, sequence: &str) {
        moves_runtime!(sequence)
            .iter()
            .for_each(|&move_| self.do_move(move_));
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

    fn draw(&self, window: &mut Window) -> Vec<SceneNode>;
    fn default_cam(&self) -> ArcBall;
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
