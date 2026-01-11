mod cube;
mod pyraminx;

#[expect(clippy::pub_use)]
pub use {cube::Cube, pyraminx::Pyraminx};

use {
    crate::{color::Color, cub2, cub3, r#move::Move},
    clap::ValueEnum,
    kiss3d::{camera::OrbitCamera3d, scene::SceneNode3d},
    std::fmt::Display,
};

// TODO: each Puzzle should have its own Move and Sticker enums
pub trait Puzzle: Display + Send {
    fn solve(&self) -> Option<Vec<Move>>;

    fn is_solved(&self) -> bool;

    fn get_faces(&self) -> &Vec<Color>;

    fn do_move(&mut self, move_: Move);

    fn rand_scramble_moves(&self) -> Vec<Move>; // TODO New vec every time :(

    fn scramble(&mut self, sequence: &str) {
        for s in sequence.split_whitespace() {
            let move_ = self.parse_move(s).unwrap();
            self.do_move(move_);
        }
    }

    fn rand_scramble_iterations(&self) -> usize;

    fn rand_scramble(&mut self) -> Vec<Move> {
        use rand::{prelude::*, rng};

        // TODO Better scrambler
        let mut sequence: Vec<Move> = Vec::new();
        let iterations = self.rand_scramble_iterations();

        while sequence.len() < iterations {
            let move_ = *self.rand_scramble_moves().choose(&mut rng()).unwrap();
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

    fn parse_move(&self, value: &str) -> Result<Move, String>;
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum PuzzleArg {
    Cube2,
    Cube3,
    Pyraminx,
}

impl PuzzleArg {
    pub fn build(&self) -> Box<dyn Puzzle> {
        match self {
            Self::Cube2 => Box::new(cub2!()),
            Self::Cube3 => Box::new(cub3!()),
            Self::Pyraminx => Box::new(Pyraminx::new()),
        }
    }
}
