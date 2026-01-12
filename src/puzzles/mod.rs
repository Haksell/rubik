mod cube;
mod pyraminx;

#[expect(clippy::pub_use)]
pub use {cube::Cube, pyraminx::Pyraminx};

use {
    crate::{color::Color, cub2, cub3, r#move::Move},
    clap::ValueEnum,
    kiss3d::{camera::OrbitCamera3d, scene::SceneNode3d},
    rand::seq::IndexedRandom as _,
    std::fmt::Display,
};

// TODO: each Puzzle should have its own Move and Sticker enums
// and associated constants of all moves and all stickers
pub trait Puzzle: Display {
    fn solve(&self) -> Option<Vec<Move>>;

    fn is_solved(&self) -> bool;

    fn get_faces(&self) -> &[Color];

    fn do_move(&mut self, move_: Move);

    fn rand_scramble_moves(&self) -> &'static [Move];

    fn scramble(&mut self, sequence: &str) {
        for s in sequence.split_whitespace() {
            let move_ = self.parse_move(s).unwrap();
            self.do_move(move_);
        }
    }

    fn rand_scramble_iterations(&self) -> usize;

    fn rand_scramble(&mut self) -> Vec<Move> {
        let mut sequence: Vec<Move> = Vec::new();
        let iterations = self.rand_scramble_iterations();
        let mut rng = rand::rng();

        while sequence.len() < iterations {
            let move_ = *self.rand_scramble_moves().choose(&mut rng).unwrap();
            if sequence
                .last()
                .is_some_and(|last_move| last_move.same_face(&move_))
            {
                continue;
            }
            self.do_move(move_);
            sequence.push(move_);
        }

        sequence
    }

    fn draw(&self, scene: &mut SceneNode3d) -> Vec<SceneNode3d>;

    fn refresh_stickers(&self, stickers: &mut [SceneNode3d]) {
        stickers
            .iter_mut()
            .zip(self.get_faces().iter())
            .for_each(|(node, &color)| {
                node.set_color(color.as_rgba().into());
            });
    }

    fn default_cam(&self) -> OrbitCamera3d;

    fn opposite_move(&self, move_: Move) -> Move;

    fn parse_move(&self, value: &str) -> Result<Move, String>;
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum PuzzleArg {
    #[value(alias = "2", alias = "2x2", alias = "2x2x2")]
    Cube2,
    #[value(alias = "3", alias = "3x3", alias = "3x3x3")]
    Cube3,
    #[value(alias = "pyra")]
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
