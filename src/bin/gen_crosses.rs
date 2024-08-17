use rubik::{
    cub3,
    cube::Cube,
    files::{self, FILE_CROSSES},
    r#move::Move,
    solvers::NUM_CROSSES,
};
use std::{collections::VecDeque, io};

const DUMMY_MOVE: Move = Move::U; // could be anything

fn main() -> io::Result<()> {
    let cube = cub3!();
    let mut moves: [Option<Move>; NUM_CROSSES] = [None; NUM_CROSSES];
    let mut queue = VecDeque::new();
    queue.push_back((cube, DUMMY_MOVE));
    let mut remaining_crosses = NUM_CROSSES;
    while remaining_crosses > 0 {
        let (cube, last_move) = queue.pop_front().unwrap();
        let idx = cube.cross_index();
        if moves[idx].is_some() {
            continue;
        }
        remaining_crosses -= 1;
        moves[idx] = Some(last_move.opposite());
        for move_ in Move::iterator() {
            let mut next_cube = cube.clone();
            next_cube.do_move(move_);
            queue.push_back((next_cube, move_));
        }
    }

    files::write_moves(FILE_CROSSES, &moves)
}
