use rubik::{
    cub3,
    cube::Cube,
    files::{self, FILE_EO_LINES},
    r#move::Move,
    solvers::NUM_CROSSES,
};
use std::{collections::VecDeque, io};

const DUMMY_MOVE: Move = Move::U; // could be anything
const NUM_EO_LINES: usize = (1 << 11) * 24 * 22;

fn main() -> io::Result<()> {
    let cube = cub3!();
    let mut moves: [Option<Move>; NUM_CROSSES] = [None; NUM_CROSSES];
    let mut queue = VecDeque::new();
    queue.push_back((cube, DUMMY_MOVE));
    let mut remaining_eo_lines = NUM_CROSSES;
    while remaining_eo_lines > 0 {
        let (cube, last_move) = queue.pop_front().unwrap();
        let idx = cube.cross_index();
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

fn is_eo_line_solved(cube: &Cube<3>) -> bool {
    false
}

fn eo_line_index(cube: &Cube<3>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::{eo_line_index, is_eo_line_solved, NUM_EO_LINES};
    use rubik::{cub3, cube::Cube, r#move::Move};

    #[test]
    fn test_is_eo_line_solved() {
        let mut cube = cub3!();
        assert!(is_eo_line_solved(&cube));
        // TODO
    }

    #[test]
    fn test_eo_line_index_solved() {
        let mut cube = cub3!();
        assert_eq!(eo_line_index(&cube), 0);
        cube.do_move(Move::R);
        cube.do_move(Move::U);
        cube.do_move(Move::R3);
        cube.do_move(Move::U3);
        assert_eq!(eo_line_index(&cube), 0);
    }

    #[test]
    fn test_eo_line_index_random() {
        let mut cube = cub3!();
        for _ in 0..100 {
            cube.do_move(Move::random());
            assert!(eo_line_index(&cube) < NUM_EO_LINES);
        }
    }
}
