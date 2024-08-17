mod cfop;
mod iddfs;
mod kociemba;

use crate::{cube::Cube, r#move::Move};
pub use cfop::{cfop, NUM_CROSSES};
pub use iddfs::iddfs;
pub use kociemba::kociemba;

pub fn premover(cube: &mut Cube<3>) -> Vec<Move> {
    let normal_solve = cfop(&mut cube.clone());
    if normal_solve.is_empty() {
        return normal_solve;
    }
    let mut best = normal_solve.clone();
    for move_ in Move::iterator() {
        if move_.same_face(&normal_solve[0]) {
            continue;
        }
        let mut clone = cube.clone();
        clone.do_move(move_);
        let mut solution = vec![move_];
        solution.extend(cfop(&mut clone));
        if solution.len() < best.len() {
            best = solution;
        }
    }
    for &move_ in &best {
        cube.do_move(move_);
    }
    best
}
