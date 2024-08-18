use crate::{cub3, r#move::Move, Cube};
use std::{collections::VecDeque, io};

use super::file_operations::write_moves;

const DUMMY_MOVE: Move = Move::U; // could be anything

pub fn generate_start_table(
    filename: &str,
    num_cases: usize,
    calc_index: fn(&Cube<3>) -> usize,
) -> io::Result<()> {
    let cube = cub3!();
    let mut moves: Vec<Option<Move>> = vec![None; num_cases];
    let mut queue = VecDeque::new();
    queue.push_back((cube, DUMMY_MOVE));
    let mut remaining_cases = num_cases;
    while remaining_cases > 0 {
        let (cube, last_move) = queue.pop_front().unwrap();
        let idx = calc_index(&cube);
        if moves[idx].is_some() {
            continue;
        }
        remaining_cases -= 1;
        moves[idx] = Some(last_move.opposite());
        for move_ in Move::iterator() {
            // TODO: skip if same face, but don't forget about DUMMY_MOVE
            let mut next_cube = cube.clone();
            next_cube.do_move(move_);
            queue.push_back((next_cube, move_));
        }
    }

    write_moves(filename, &moves)
}
