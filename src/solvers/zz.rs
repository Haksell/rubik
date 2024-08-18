use super::reduce_moves;
use crate::color::Color;
use crate::tables::{read_moves, FILE_EO_LINES};
use crate::{r#move::Move, Cube};
use crate::{Sticker, EDGES};

const NUM_LINES: usize = 12 * 11;
pub const NUM_EO_LINES: usize = (1 << 11) * NUM_LINES;

pub fn zz(cube: &mut Cube<3>) -> Vec<Move> {
    let mut solution = vec![];
    solution.extend(solve_eo_line(cube));
    reduce_moves(&solution)
}

fn solve_eo_line(cube: &mut Cube<3>) -> Vec<Move> {
    let eo_line_moves = read_moves(FILE_EO_LINES)
        .unwrap_or_else(|err| panic!("Failed to read {FILE_EO_LINES}: {err}"));
    let mut solution = vec![];
    let mut idx = cube.eo_line_index();
    while idx != 0 {
        let move_ = eo_line_moves[idx];
        cube.do_move(move_);
        solution.push(move_);
        idx = cube.eo_line_index();
    }
    solution
}

impl Cube<3> {
    #[inline]
    fn is_edge_oriented(&self, s1: Sticker, s2: Sticker) -> bool {
        self.faces[s1 as usize] == Color::WHITE
            || self.faces[s1 as usize] == Color::YELLOW
            || self.faces[s2 as usize] == Color::ORANGE
            || self.faces[s2 as usize] == Color::RED
    }

    pub fn is_eo_line_solved(&self) -> bool {
        use crate::color::Color;
        use crate::EDGES;
        use Sticker::*;

        // TODO: remove 2 redundant edge checks
        EDGES.iter().all(|&(s1, s2)| self.is_edge_oriented(s1, s2))
            && self.faces[DF as usize] == Color::YELLOW
            && self.faces[FD as usize] == Color::GREEN
            && self.faces[DB as usize] == Color::YELLOW
            && self.faces[BD as usize] == Color::BLUE
    }

    pub fn eo_line_index(&self) -> usize {
        let mut orientation_index = 0;
        for (i, &(s1, s2)) in EDGES[..11].into_iter().enumerate() {
            if !self.is_edge_oriented(s1, s2) {
                orientation_index |= 1 << i;
            }
        }
        let mut yellow_green: usize = usize::MAX;
        let mut yellow_blue: usize = usize::MAX;
        for (i, &(s1, s2)) in EDGES.iter().enumerate() {
            match (self.faces[s1 as usize], self.faces[s2 as usize]) {
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

    pub fn is_zz_left_solved(&self) -> bool {
        // We could eliminate some checks by assuming the EO-line is solved
        use Sticker::*;
        self.faces[FL as usize] == Color::GREEN
            && self.faces[LF as usize] == Color::ORANGE
            && self.faces[FDL as usize] == Color::GREEN
            && self.faces[LFD as usize] == Color::ORANGE
            && self.faces[DL as usize] == Color::YELLOW
            && self.faces[LD as usize] == Color::ORANGE
            && self.faces[BLD as usize] == Color::BLUE
            && self.faces[LDB as usize] == Color::ORANGE
            && self.faces[BL as usize] == Color::BLUE
            && self.faces[LB as usize] == Color::ORANGE
    }
}

#[cfg(test)]
mod tests {
    use super::NUM_EO_LINES;
    use crate::{cub3, r#move::Move, Cube};

    #[test]
    fn test_is_eo_line_solved() {
        let mut cube = cub3!();
        assert!(cube.is_eo_line_solved());
        cube.do_move(Move::D);
        assert!(!cube.is_eo_line_solved());
        cube.do_move(Move::D);
        assert!(!cube.is_eo_line_solved());
        cube.do_move(Move::D2);
        assert!(cube.is_eo_line_solved());
        cube.do_move(Move::R);
        cube.do_move(Move::U);
        cube.do_move(Move::R3);
        cube.do_move(Move::U3);
        assert!(cube.is_eo_line_solved());
        cube.do_move(Move::R3);
        cube.do_move(Move::F);
        cube.do_move(Move::R);
        cube.do_move(Move::F3);
        assert!(!cube.is_eo_line_solved());
    }

    #[test]
    fn test_eo_line_index_solved() {
        assert_eq!(cub3!().eo_line_index(), 0);
    }

    #[test]
    fn test_eo_line_index_random() {
        let mut cube = cub3!();
        for _ in 0..10000 {
            let move_ = Move::random();
            cube.do_move(move_);
            let idx = cube.eo_line_index();
            println!("{:?} {}", move_, idx);
            assert!(idx < NUM_EO_LINES);
            if cube.is_eo_line_solved() {
                assert_eq!(idx, 0);
            } else {
                assert!(0 < idx);
                assert!(idx < NUM_EO_LINES);
            }
        }
    }

    #[test]
    fn test_is_zz_left_solved() {
        let mut cube = cub3!();
        assert!(cube.is_zz_left_solved());
        cube.scramble("R U' L' U R' U' L R2 U R2 U");
        assert!(cube.is_zz_left_solved());

        cube.do_move(Move::D);
        assert!(!cube.is_zz_left_solved());
        cube.do_move(Move::D3);

        cube.do_move(Move::F);
        assert!(!cube.is_zz_left_solved());
        cube.do_move(Move::F3);

        cube.do_move(Move::B);
        assert!(!cube.is_zz_left_solved());
        cube.do_move(Move::B3);

        cube.scramble("R D U R U' R' D'");
        assert!(!cube.is_zz_left_solved());
    }
}
