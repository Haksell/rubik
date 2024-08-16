use std::collections::VecDeque;

use crate::files::{self, FILE_CROSSES};
use crate::moves;
use crate::trigger::Trigger;
use crate::{color::Color, cube::Cube, r#move::Move, EDGES};

pub const NUM_CROSSES: usize = 24 * 22 * 20 * 18;

// TODO: think about Cube::<3>.cfop or cfop(&mut Cube<3>)
// (same for the other solvers)

pub fn cfop(cube: &mut Cube<3>) -> Vec<Move> {
    let mut solution = vec![];
    solution.extend(solve_cross(cube));
    solution.extend(solve_f2l(cube));
    println!("{cube}");
    solution.extend(solve_oll(cube));
    // solution.extend(solve_pll(cube));
    // TODO: reduce solution (between steps)
    solution
}

fn solve_cross(cube: &mut Cube<3>) -> Vec<Move> {
    let cross_moves = files::read_moves(FILE_CROSSES)
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

fn solve_f2l(cube: &mut Cube<3>) -> Vec<Move> {
    use crate::trigger::{TRIGGERS_SLOT_0, TRIGGERS_SLOT_1, TRIGGERS_SLOT_2, TRIGGERS_SLOT_3};

    let mut solution = vec![];
    let mut to_solve = 0b1111; // TODO: check accidental X-cross
    while to_solve != 0 {
        let mut triggers = vec![Trigger::U, Trigger::U2, Trigger::U3];
        if to_solve & 1 != 0 {
            triggers.extend(TRIGGERS_SLOT_0);
        }
        if to_solve & 2 != 0 {
            triggers.extend(TRIGGERS_SLOT_1);
        }
        if to_solve & 4 != 0 {
            triggers.extend(TRIGGERS_SLOT_2);
        }
        if to_solve & 8 != 0 {
            triggers.extend(TRIGGERS_SLOT_3);
        }
        let pair_solution = solve_pair(cube, &triggers);
        to_solve &= !(1 << pair_solution.last().unwrap().slot());
        for trigger in pair_solution {
            cube.do_trigger(trigger);
            solution.extend(trigger.moves());
        }
    }
    solution
}

fn solve_pair(cube: &Cube<3>, triggers: &[Trigger]) -> Vec<Trigger> {
    // TODO: came_from like n_puzzle
    let mut queue: VecDeque<(Cube<3>, Vec<Trigger>)> = VecDeque::new();
    queue.push_back((cube.clone(), vec![]));
    loop {
        let (cube, pair_solution) = queue.pop_front().unwrap();
        let slot = match pair_solution.last() {
            Some(trigger) => trigger.slot(),
            None => usize::MAX,
        };
        if slot != usize::MAX && cube.is_pair_solved(slot) {
            return pair_solution;
        }
        for &trigger in triggers {
            if pair_solution.is_empty() || trigger.slot() != slot {
                let mut next_cube = cube.clone();
                let mut next_vec = pair_solution.clone();
                next_cube.do_trigger(trigger);
                next_vec.push(trigger);
                queue.push_back((next_cube, next_vec));
            }
        }
    }
}

fn solve_oll(cube: &mut Cube<3>) -> Vec<Move> {
    use crate::Sticker::*;
    let mut oll_solution = vec![];
    for _ in 0..4 {
        let moves = match ((cube.faces[LBU as usize] == Color::WHITE) as u16) << 8
            | ((cube.faces[LU as usize] == Color::WHITE) as u16) << 7
            | ((cube.faces[LUF as usize] == Color::WHITE) as u16) << 6
            | ((cube.faces[FLU as usize] == Color::WHITE) as u16) << 5
            | ((cube.faces[FU as usize] == Color::WHITE) as u16) << 4
            | ((cube.faces[FUR as usize] == Color::WHITE) as u16) << 3
            | ((cube.faces[RFU as usize] == Color::WHITE) as u16) << 2
            | ((cube.faces[RU as usize] == Color::WHITE) as u16) << 1
            | ((cube.faces[RUB as usize] == Color::WHITE) as u16) << 0
        {
            0b000000000 => Some(vec![]),
            0b111010111 => Some(moves!("R U2 R2 F R F' U2 R' F R F'")),
            0b110111011 => Some(moves!("R U' R2 D' L F L' D R2 U R'")),
            0b010011011 => Some(moves!("L' R2 B R' B L U2 L' B L R'")),
            0b010110110 => Some(moves!("L R2 F' R F' L' U2 L F' L' R")),
            0b011000001 => Some(moves!("L' B2 R B R' B L")),
            0b110110100 => Some(moves!("L F2 R' F' R F' L'")),
            0b000011011 => Some(moves!("L F R' F R F2 L'")),
            0b000100110 => Some(moves!("L' B' R B' R' B2 L")),
            0b110010100 => Some(moves!("F' U' F L F' L' U L F L'")), // bad
            0b001010011 => Some(moves!("F U F' R' F R U' R' F' R")), // bad
            0b010001001 => Some(moves!("L' R2 B R' B R B2 R' B L R'")), // bad
            0b010110100 => Some(moves!("L R2 F' R F' R' F2 R F' L' R")), // bad
            0b000011001 => Some(moves!("L F' L' U' L F L' F' U F")),
            0b100110000 => Some(moves!("R' F R U R' F' R F U' F'")),
            0b001010001 => Some(moves!("L' B' L R' U' R U L' B L")),
            0b100110100 => Some(moves!("L F L' R U R' U' L F' L'")),
            0b011010010 => Some(moves!("F' L F L' U2 L' U B' U B L")), // bad
            0b010010010 => Some(moves!("F R U R' U F' U2 F' L F L'")), // bad
            0b011010110 => Some(moves!("R' U2 F R U R' U' F2 U2 F R")), // bad
            0b000000000 => Some(moves!("")),                           // standard 13
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            0b000000000 => Some(moves!("")),
            _ => None,
        };
        if let Some(moves) = moves {
            for move_ in moves {
                oll_solution.push(move_);
                cube.do_move(move_);
            }
            return oll_solution;
        }
        oll_solution.push(Move::U);
        cube.do_move(Move::U);
    }
    unreachable!();
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

    pub fn is_pair_solved(&self, index: usize) -> bool {
        use crate::Sticker::*;
        match index {
            0 => {
                self.faces[BR as usize] == Color::BLUE
                    && self.faces[RB as usize] == Color::RED
                    && self.faces[DRB as usize] == Color::YELLOW
                    && self.faces[RBD as usize] == Color::RED
                    && self.faces[BDR as usize] == Color::BLUE
            }
            1 => {
                self.faces[FR as usize] == Color::GREEN
                    && self.faces[RF as usize] == Color::RED
                    && self.faces[DFR as usize] == Color::YELLOW
                    && self.faces[FRD as usize] == Color::GREEN
                    && self.faces[RDF as usize] == Color::RED
            }
            2 => {
                self.faces[FL as usize] == Color::GREEN
                    && self.faces[LF as usize] == Color::ORANGE
                    && self.faces[DLF as usize] == Color::YELLOW
                    && self.faces[LFD as usize] == Color::ORANGE
                    && self.faces[FDL as usize] == Color::GREEN
            }
            3 => {
                self.faces[BL as usize] == Color::BLUE
                    && self.faces[LB as usize] == Color::ORANGE
                    && self.faces[DBL as usize] == Color::YELLOW
                    && self.faces[BLD as usize] == Color::BLUE
                    && self.faces[LDB as usize] == Color::ORANGE
            }
            _ => unreachable!(),
        }
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
