mod cfop;
mod iddfs;
mod kociemba;
mod last_layer;
mod zz;

pub use {
    cfop::{cfop, NUM_CROSSES},
    iddfs::{iddfs, DFSAble},
    kociemba::kociemba,
    zz::{zz, NUM_EO_LINES, NUM_ZZ_LEFT, NUM_ZZ_RIGHT},
};

use crate::{
    r#move::{Move, MOVES},
    Cube, Puzzle,
};

// TODO: depends on PyraMove or CubeMove
fn reduce_moves(moves: &Vec<Move>) -> Vec<Move> {
    // TODO: handle L R L'
    let mut simplified: Vec<Move> = vec![];
    for &move_ in moves {
        let mut push_move = true;
        if let Some(&last) = simplified.last() {
            if last.same_face(&move_) {
                push_move = false;
                simplified.pop();
                let repetitions = (last.repetitions() + move_.repetitions()) % 4;
                if repetitions != 0 {
                    simplified
                        .push(Move::try_from(move_.as_int() % 6 + 6 * (repetitions - 1)).unwrap());
                }
            }
        }
        if push_move {
            simplified.push(move_);
        }
    }
    simplified
}

pub fn premover(cube: &mut Cube<3>, solver: fn(&mut Cube<3>) -> Vec<Move>) -> Vec<Move> {
    let solution = MOVES
        .into_iter()
        .map(|move_| {
            let mut clone = cube.clone();
            clone.do_move(move_);
            let mut solution = vec![move_];
            solution.extend(solver(&mut clone));
            reduce_moves(&solution)
        })
        .min_by_key(Vec::len)
        .unwrap();
    for &move_ in &solution {
        cube.do_move(move_);
    }
    solution
}

#[cfg(test)]
mod tests {
    use super::reduce_moves;
    use move_macro::moves;

    #[test]
    fn test_moves_reduction() {
        assert_eq!(reduce_moves(&moves!["R L"]), moves!["R L"]);
        assert_eq!(reduce_moves(&moves!["L D"]), moves!["L D"]);
        assert_eq!(reduce_moves(&moves!["U U"]), moves!["U2"]);
        assert_eq!(reduce_moves(&moves!["L L2"]), moves!["L'"]);
        assert_eq!(reduce_moves(&moves!["U U'"]), moves![""]);
        assert_eq!(reduce_moves(&moves!["L2 L"]), moves!["L'"]);
        assert_eq!(reduce_moves(&moves!["R2 R2"]), moves![""]);
        assert_eq!(reduce_moves(&moves!["B2 B'"]), moves!["B"]);
        assert_eq!(reduce_moves(&moves!["D' D"]), moves![""]);
        assert_eq!(reduce_moves(&moves!["B' B2"]), moves!["B"]);
        assert_eq!(reduce_moves(&moves!["F' F'"]), moves!["F2"]);
        assert_eq!(reduce_moves(&moves!["R R R R"]), moves![""]);
        assert_eq!(reduce_moves(&moves!["U2 R R R R U U"]), moves![""]);
    }
}
