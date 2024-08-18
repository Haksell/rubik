mod cfop;
mod iddfs;
mod kociemba;
mod zz;

use crate::{r#move::Move, Cube};
pub use cfop::{cfop, NUM_CROSSES};
pub use iddfs::iddfs;
pub use kociemba::kociemba;
pub use zz::{zz, NUM_EO_LINES, NUM_ZZ_LEFT, NUM_ZZ_RIGHT};

fn solve_last_layer_step(
    cube: &mut Cube<3>,
    alg_matcher: fn(&Cube<3>) -> Option<Vec<Move>>,
) -> Vec<Move> {
    let mut u_moves = 0;
    for _ in 0..4 {
        let moves = alg_matcher(cube);
        if let Some(moves) = moves {
            for _ in 0..u_moves {
                cube.do_move(Move::U3);
            }
            let rotated_moves: Vec<Move> = moves
                .into_iter()
                .map(|mut move_| {
                    for _ in 0..u_moves {
                        move_ = move_.rotate_y();
                    }
                    move_
                })
                .collect();
            for &move_ in &rotated_moves {
                cube.do_move(move_);
            }
            return rotated_moves;
        }
        u_moves += 1;
        cube.do_move(Move::U);
    }
    unreachable!();
}

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
    let solution = Move::iterator()
        .into_iter()
        .map(|move_| {
            let mut clone = cube.clone();
            clone.do_move(move_);
            let mut solution = vec![move_];
            solution.extend(solver(&mut clone));
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
        assert_eq!(reduce_moves(&moves!("L D")), moves!("L D"));
        assert_eq!(reduce_moves(&moves!("U U")), moves!("U2"));
        assert_eq!(reduce_moves(&moves!("L L2")), moves!("L'"));
        assert_eq!(reduce_moves(&moves!("U U'")), vec![]);
        assert_eq!(reduce_moves(&moves!("L2 L")), moves!("L'"));
        assert_eq!(reduce_moves(&moves!("R2 R2")), vec![]);
        assert_eq!(reduce_moves(&moves!("B2 B'")), moves!("B"));
        assert_eq!(reduce_moves(&moves!("D' D")), vec![]);
        assert_eq!(reduce_moves(&moves!("B' B2")), moves!("B"));
        assert_eq!(reduce_moves(&moves!("F' F'")), moves!("F2"));
        assert_eq!(reduce_moves(&moves!("R R R R")), vec![]);
        assert_eq!(reduce_moves(&moves!("U2 R R R R U U")), vec![]);
    }
}
