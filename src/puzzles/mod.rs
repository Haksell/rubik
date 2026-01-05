mod cube;
mod pyraminx;

pub use {cube::Cube, pyraminx::Pyraminx};

use {
    crate::{color::Color, cub2, cub3, r#move::Move},
    clap::ValueEnum,
    kiss3d::{camera::OrbitCamera3d, scene::SceneNode3d},
    std::fmt::Display,
};

pub trait Puzzle: Display {
    fn solve(&self) -> Option<Vec<Move>>;

    fn is_solved(&self) -> bool;

    fn get_faces(&self) -> &Vec<Color>;

    fn do_move(&mut self, move_: Move);

    fn available_moves(&self) -> Vec<Move>; // TODO New vec every time :(

    fn scramble(&mut self, sequence: &str) {
        for s in sequence.split_whitespace() {
            let move_ = self.parse_move(s).unwrap();
            self.do_move(move_);
        }
    }

    fn rand_scramble(&mut self, iterations: usize) -> Vec<Move> {
        // TODO Better scrambler
        let mut sequence: Vec<Move> = Vec::new();

        while sequence.len() < iterations {
            let move_ = Move::choice(&self.available_moves());
            if !sequence.is_empty() && move_.same_face(sequence.last().unwrap()) {
                continue;
            }
            self.do_move(move_);
            sequence.push(move_);
        }
        sequence
    }

    fn draw(&self, scene: &mut SceneNode3d) -> Vec<SceneNode3d>;

    fn default_cam(&self) -> OrbitCamera3d;

    fn opposite_move(&self, move_: Move) -> Move;

    fn parse_move(&self, str: &str) -> Result<Move, String>;
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
