// TODO: less pub

pub mod color;
pub mod r#move;
pub mod puzzles;
pub mod solvers;
mod sticker;
pub mod tables;
pub mod trigger;
pub mod visualizer;

pub use {
    puzzles::{Cube, Puzzle, Pyraminx},
    sticker::{EDGES, Sticker},
};
