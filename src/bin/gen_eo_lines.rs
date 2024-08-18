use rubik::{
    color::Color,
    cub3,
    cube::Cube,
    files::{self, FILE_EO_LINES},
    r#move::Move,
    Sticker, EDGES,
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
        let idx = eo_line_index(&cube);
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
        if remaining_eo_lines % 10000 == 0 || remaining_eo_lines < 1000 {
            println!("{remaining_eo_lines}");
        }
    }

    files::write_moves(FILE_EO_LINES, &moves)
}

// TODO: all this in impl Cube<3> block in solvers/zz.rs

#[inline]
fn is_edge_oriented(cube: &Cube<3>, s1: Sticker, s2: Sticker) -> bool {
    cube.faces[s1 as usize] == Color::WHITE
        || cube.faces[s1 as usize] == Color::YELLOW
        || cube.faces[s2 as usize] == Color::ORANGE
        || cube.faces[s2 as usize] == Color::RED
}

#[cfg(test)]
fn is_eo_line_solved(cube: &Cube<3>) -> bool {
    use rubik::color::Color;
    use rubik::Sticker::*;
    use rubik::EDGES;

    // TODO: remove 2 redundant edge checks
    EDGES.iter().all(|&(s1, s2)| is_edge_oriented(cube, s1, s2))
        && cube.faces[DF as usize] == Color::YELLOW
        && cube.faces[FD as usize] == Color::GREEN
        && cube.faces[DB as usize] == Color::YELLOW
        && cube.faces[BD as usize] == Color::BLUE
}

fn eo_line_index(cube: &Cube<3>) -> usize {
    let mut orientation_index = 0;
    for (i, &(s1, s2)) in EDGES[..11].into_iter().enumerate() {
        if !is_edge_oriented(cube, s1, s2) {
            orientation_index |= 1 << i;
        }
    }
    let mut yellow_green: usize = usize::MAX;
    let mut yellow_blue: usize = usize::MAX;
    for (i, &(s1, s2)) in EDGES.iter().enumerate() {
        match (cube.faces[s1 as usize], cube.faces[s2 as usize]) {
            (Color::YELLOW, Color::GREEN) | (Color::GREEN, Color::YELLOW) => yellow_green = i,
            (Color::YELLOW, Color::BLUE) | (Color::BLUE, Color::YELLOW) => yellow_blue = i,
            _ => {}
        }
    }
    if yellow_blue > yellow_green {
        yellow_blue -= 1;
    }
    orientation_index * NUM_LINES + yellow_blue + 11 * yellow_green
}

#[cfg(test)]
mod tests {
    use super::{eo_line_index, is_eo_line_solved, NUM_EO_LINES};
    use rubik::{cub3, cube::Cube, r#move::Move};

    #[test]
    fn test_is_eo_line_solved() {
        let mut cube = cub3!();
        assert!(is_eo_line_solved(&cube));
        cube.do_move(Move::D);
        assert!(!is_eo_line_solved(&cube));
        cube.do_move(Move::D);
        assert!(!is_eo_line_solved(&cube));
        cube.do_move(Move::D2);
        assert!(is_eo_line_solved(&cube));
        cube.do_move(Move::R);
        cube.do_move(Move::U);
        cube.do_move(Move::R3);
        cube.do_move(Move::U3);
        assert!(is_eo_line_solved(&cube));
        cube.do_move(Move::R3);
        cube.do_move(Move::F);
        cube.do_move(Move::R);
        cube.do_move(Move::F3);
        assert!(!is_eo_line_solved(&cube));
    }

    #[test]
    fn test_eo_line_index_solved() {
        assert_eq!(eo_line_index(&cub3!()), 0);
    }

    #[test]
    fn test_eo_line_index_random() {
        let mut cube = cub3!();
        for _ in 0..10000 {
            let move_ = Move::random();
            cube.do_move(move_);
            let idx = eo_line_index(&cube);
            println!("{:?} {}", move_, idx);
            assert!(idx < NUM_EO_LINES);
            if is_eo_line_solved(&cube) {
                assert_eq!(idx, 0);
            } else {
                assert!(0 < idx);
                assert!(idx < NUM_EO_LINES);
            }
        }
    }
}
