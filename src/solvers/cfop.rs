use crate::filenames::FILE_CROSSES;
use crate::{color::Color, cube::Cube, r#move::Move, EDGES};
use std::fs::File;
use std::io::{self, Read as _};

pub const NUM_CROSSES: usize = 24 * 22 * 20 * 18;

// TODO: think about Cube::<3>.cfop or cfop(&mut Cube<3>)
// (same for the other solvers)

pub fn cfop(cube: &mut Cube<3>) -> Vec<Move> {
    let mut solution = vec![];
    solution.extend(solve_cross(cube));
    // TODO: solution.extend(solve_f2l(cube));
    // TODO: solution.extend(solve_oll(cube));
    // TODO: solution.extend(solve_pll(cube));
    solution
}

fn solve_cross(cube: &mut Cube<3>) -> Vec<Move> {
    let cross_moves = read_cross_moves_from_file()
        .unwrap_or_else(|err| panic!("Failed to read {FILE_CROSSES}: {err}"));
    let mut solution = vec![];
    let mut idx = cube.cross_index();
    while idx != 0 {
        let move_ = cross_moves[idx];
        cube.do_move(move_);
        solution.push(move_);
        idx = cube.cross_index();
    }
    solution
}

// TODO: don't return a copy
fn read_cross_moves_from_file() -> io::Result<[Move; NUM_CROSSES]> {
    let mut file = File::open(FILE_CROSSES)?;
    let mut moves = [Move::U; NUM_CROSSES];
    let buffer =
        unsafe { std::slice::from_raw_parts_mut(moves.as_mut_ptr() as *mut u8, NUM_CROSSES) };
    file.read_exact(buffer)?;
    Ok(moves)
}

impl Cube<3> {
    #[cfg(test)]
    fn is_cross_solved(&self) -> bool {
        use crate::Sticker::*;
        return self.faces[DF as usize] == Color::YELLOW
            && self.faces[DR as usize] == Color::YELLOW
            && self.faces[DB as usize] == Color::YELLOW
            && self.faces[DL as usize] == Color::YELLOW
            && self.faces[FD as usize] == Color::GREEN
            && self.faces[RD as usize] == Color::RED
            && self.faces[BD as usize] == Color::BLUE
            && self.faces[LD as usize] == Color::ORANGE;
    }

    pub fn cross_index(&self) -> usize {
        let mut yellow_green: usize = usize::MAX;
        let mut yellow_red: usize = usize::MAX;
        let mut yellow_blue: usize = usize::MAX;
        let mut yellow_orange: usize = usize::MAX;
        for (i, &(s1, s2)) in EDGES.iter().enumerate() {
            if self.faces[s1 as usize] == Color::YELLOW {
                match self.faces[s2 as usize] {
                    Color::GREEN => yellow_green = 2 * i,
                    Color::RED => yellow_red = 2 * i,
                    Color::BLUE => yellow_blue = 2 * i,
                    Color::ORANGE => yellow_orange = 2 * i,
                    _ => unreachable!(),
                }
            } else if self.faces[s2 as usize] == Color::YELLOW {
                match self.faces[s1 as usize] {
                    Color::GREEN => yellow_green = 2 * i + 1,
                    Color::RED => yellow_red = 2 * i + 1,
                    Color::BLUE => yellow_blue = 2 * i + 1,
                    Color::ORANGE => yellow_orange = 2 * i + 1,
                    _ => unreachable!(),
                }
            }
        }
        if yellow_red > yellow_green {
            yellow_red -= 2;
        }
        if yellow_blue > yellow_green {
            yellow_blue -= 2;
        }
        if yellow_blue > yellow_red {
            yellow_blue -= 2;
        }
        if yellow_orange > yellow_green {
            yellow_orange -= 2;
        }
        if yellow_orange > yellow_red {
            yellow_orange -= 2;
        }
        if yellow_orange > yellow_blue {
            yellow_orange -= 2;
        }
        yellow_orange + 18 * yellow_blue + 18 * 20 * yellow_red + 18 * 20 * 22 * yellow_green
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_cross, Cube, NUM_CROSSES};
    use crate::r#move::Move;

    #[test]
    fn test_is_cross_solved() {
        let mut cube = Cube::<3>::new();
        assert!(cube.is_cross_solved());

        // sexy move
        cube.do_move(Move::R);
        cube.do_move(Move::U);
        cube.do_move(Move::R3);
        cube.do_move(Move::U3);
        assert!(cube.is_cross_solved());

        // 3G
        cube.do_move(Move::L3);
        cube.do_move(Move::U2);
        cube.do_move(Move::L);
        assert!(cube.is_cross_solved());

        // D4
        cube.do_move(Move::D);
        assert!(!cube.is_cross_solved());
        cube.do_move(Move::D);
        assert!(!cube.is_cross_solved());
        cube.do_move(Move::D);
        assert!(!cube.is_cross_solved());
        cube.do_move(Move::D);
        assert!(cube.is_cross_solved());

        // PLL I
        cube.do_move(Move::L2);
        cube.do_move(Move::R2);
        cube.do_move(Move::U2);
        cube.do_move(Move::L2);
        cube.do_move(Move::R2);
        cube.do_move(Move::D2);
        assert!(!cube.is_cross_solved());
    }

    #[test]
    fn test_cross_index_solved() {
        let mut cube = Cube::<3>::new();
        assert_eq!(cube.cross_index(), 0);
        cube.do_move(Move::R);
        cube.do_move(Move::U);
        cube.do_move(Move::R3);
        cube.do_move(Move::U3);
        assert_eq!(cube.cross_index(), 0);
    }

    #[test]
    fn test_cross_index_random() {
        let mut cube = Cube::<3>::new();
        for _ in 0..100 {
            cube.do_move(Move::random());
            assert!(cube.cross_index() < NUM_CROSSES);
        }
    }

    #[test]
    fn test_solve_cross() {
        for _ in 0..10 {
            let mut cube = Cube::<3>::new();
            cube.rand_scramble(100);
            let solution = solve_cross(&mut cube);
            assert!(cube.is_cross_solved());
            assert!(solution.len() <= 8);
        }
    }
}
