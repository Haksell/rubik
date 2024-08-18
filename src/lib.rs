// TODO: less pub

pub mod color;
mod cube;
pub mod files;
mod generate_table;
pub mod r#move;
pub mod solvers;
mod sticker;
pub mod trigger;

pub use cube::Cube;
pub use generate_table::generate_table;
pub use sticker::{Sticker, EDGES};
