// TODO: less pub

pub mod color;
mod cube;
pub mod r#move;
pub mod puzzles;
mod pyraminx;
pub mod solvers;
mod sticker;
pub mod tables;
pub mod trigger;
pub mod visualizer;

pub use cube::Cube;
pub use puzzles::Puzzle;
pub use pyraminx::Pyraminx;
pub use sticker::{Sticker, EDGES};
