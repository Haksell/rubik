use {
    super::file_operations::write_moves,
    crate::{Cube, Puzzle, cub3, r#move::Move},
    std::{collections::VecDeque, io},
};

pub const DUMMY_MOVE: Move = Move::U; // could be anything

pub fn generate_table(
    filename: &str,
    num_cases: usize,
    calc_index: fn(&Cube<3>) -> usize,
    move_set: &[Move],
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
        for &move_ in move_set {
            if remaining_cases == num_cases - 1 || !move_.same_face(&last_move) {
                let mut next_cube = cube.clone();
                next_cube.do_move(move_);
                queue.push_back((next_cube, move_));
            }
        }
    }

    write_moves(filename, &moves)
}
