use crate::{cube::Cube, r#move::Move};

pub fn kociemba(start: Cube<3>) -> Option<Vec<Move>> {
    if start.is_solved() {
        Some(vec![])
    } else {
        None
    }
}