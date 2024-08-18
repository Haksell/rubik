// TODO: less pub

pub mod color;
mod cube;
pub mod r#move;
pub mod solvers;
mod sticker;
pub mod tables;
pub mod trigger;

pub use cube::Cube;
pub use sticker::{Sticker, EDGES};
