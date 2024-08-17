mod cfop;
mod iddfs;
mod kociemba;

use crate::{cube::Cube, r#move::Move};
pub use cfop::{cfop, NUM_CROSSES};
pub use iddfs::iddfs;
pub use kociemba::kociemba;

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

pub fn premover(cube: &mut Cube<3>) -> Vec<Move> {
    let solution = Move::iterator()
        .into_iter()
        .map(|move_| {
            let mut clone = cube.clone();
            clone.do_move(move_);
            let mut solution = vec![move_];
            solution.extend(cfop(&mut clone));
            reduce_moves(&solution)
        })
        .min_by_key(|solution| solution.len())
        .unwrap();
    for &move_ in &solution {
        cube.do_move(move_);
    }
    solution
}

#[cfg(test)]
mod tests {
    use super::reduce_moves;
    use crate::{moves, r#move::Move};

    #[test]
    fn test_moves_reduction() {
        assert_eq!(reduce_moves(&moves!("R L")), moves!("R L"));
        assert_eq!(
            reduce_moves(&vec![Move::L, Move::D]),
            vec![Move::L, Move::D]
        );
        assert_eq!(reduce_moves(&vec![Move::U, Move::U]), vec![Move::U2]);
        assert_eq!(reduce_moves(&vec![Move::L, Move::L2]), vec![Move::L3]);
        assert_eq!(reduce_moves(&vec![Move::U, Move::U3]), vec![]);
        assert_eq!(reduce_moves(&vec![Move::L2, Move::L]), vec![Move::L3]);
        assert_eq!(reduce_moves(&vec![Move::R2, Move::R2]), vec![]);
        assert_eq!(reduce_moves(&vec![Move::B2, Move::B3]), vec![Move::B]);
        assert_eq!(reduce_moves(&vec![Move::D3, Move::D]), vec![]);
        assert_eq!(reduce_moves(&vec![Move::B3, Move::B2]), vec![Move::B]);
        assert_eq!(reduce_moves(&vec![Move::F3, Move::F3]), vec![Move::F2]);
        assert_eq!(
            reduce_moves(&vec![Move::R, Move::R, Move::R, Move::R]),
            vec![]
        );
        assert_eq!(
            reduce_moves(&vec![
                Move::U,
                Move::U,
                Move::R,
                Move::R,
                Move::R,
                Move::R,
                Move::U,
                Move::U
            ]),
            vec![]
        );
    }
}
