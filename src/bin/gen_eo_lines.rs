// TODO: refacotr with gen_crosses

use rubik::{
    cub3,
    cube::Cube,
    files::{self, FILE_EO_LINES},
    r#move::Move,
};
use std::{collections::VecDeque, io};

const DUMMY_MOVE: Move = Move::U; // could be anything

const NUM_LINES: usize = 12 * 11;
const NUM_EO_LINES: usize = (1 << 11) * NUM_LINES;

fn main() -> io::Result<()> {
    let cube = cub3!();
    let mut moves: [Option<Move>; NUM_EO_LINES] = [None; NUM_EO_LINES];
    let mut queue = VecDeque::new();
    queue.push_back((cube, DUMMY_MOVE));
    let mut remaining_eo_lines = NUM_EO_LINES;
    while remaining_eo_lines > 0 {
        let (cube, last_move) = queue.pop_front().unwrap();
        let idx = cube.eo_line_index();
        if moves[idx].is_some() {
            continue;
        }
        remaining_eo_lines -= 1;
        moves[idx] = Some(last_move.opposite());
        for move_ in Move::iterator() {
            let mut next_cube = cube.clone();
            next_cube.do_move(move_);
            queue.push_back((next_cube, move_));
        }
    }

    files::write_moves(FILE_EO_LINES, &moves)
}
