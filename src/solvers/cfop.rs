use crate::files::{self, FILE_CROSSES};
use crate::moves;
use crate::trigger::Trigger;
use crate::{color::Color, cube::Cube, r#move::Move, EDGES};
use std::collections::{HashMap, VecDeque};

pub const NUM_CROSSES: usize = 24 * 22 * 20 * 18;

// TODO: think about Cube::<3>.cfop or cfop(&mut Cube<3>)
// (same for the other solvers)

pub fn cfop(cube: &mut Cube<3>) -> Vec<Move> {
    let mut solution = vec![];
    solution.extend(solve_cross(cube));
    solution.extend(solve_f2l(cube));
    // TODO: refactor rotation logic for OLL and PLL
    solution.extend(solve_oll(cube));
    solution.extend(solve_pll(cube));
    reduce_moves(&solution)
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
    let mut to_solve = (0..4).fold(0, |acc, slot| {
        acc | (!cube.is_pair_solved(slot) as u8) << slot
    }); // handles accidental x-crosses
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
    let mut came_from: HashMap<Cube<3>, Option<Trigger>> = HashMap::new();
    queue.push_back((cube.clone(), vec![]));
    came_from.insert(cube.clone(), None);
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
                came_from.insert(next_cube.clone(), Some(trigger));
                next_cube.do_trigger(trigger);
                next_vec.push(trigger);
                queue.push_back((next_cube, next_vec));
            }
        }
    }
}

fn solve_oll(cube: &mut Cube<3>) -> Vec<Move> {
    use crate::Sticker::*;
    let mut u_moves = 0;
    for _ in 0..4 {
        let moves = match ((cube.faces[BUL as usize] == Color::WHITE) as u16) << 8
            | ((cube.faces[LBU as usize] == Color::WHITE) as u16) << 7
            | ((cube.faces[LU as usize] == Color::WHITE) as u16) << 6
            | ((cube.faces[LUF as usize] == Color::WHITE) as u16) << 5
            | ((cube.faces[FLU as usize] == Color::WHITE) as u16) << 4
            | ((cube.faces[FU as usize] == Color::WHITE) as u16) << 3
            | ((cube.faces[FUR as usize] == Color::WHITE) as u16) << 2
            | ((cube.faces[RFU as usize] == Color::WHITE) as u16) << 1
            | ((cube.faces[RU as usize] == Color::WHITE) as u16) << 0
        {
            0b000000000 => Some(vec![]),
            0b011101011 => Some(moves!("R U2 R2 F R F' U2 R' F R F'")),
            0b011011101 => Some(moves!("R U' R2 D' L F L' D R2 U R'")),
            0b101001101 => Some(moves!("L' R2 B R' B L U2 L' B L R'")),
            0b001011011 => Some(moves!("L R2 F' R F' L' U2 L F' L' R")),
            0b101100000 => Some(moves!("L' B2 R B R' B L")),
            0b011011010 => Some(moves!("L F2 R' F' R F' L'")),
            0b100001101 => Some(moves!("L F R' F R F2 L'")),
            0b000010011 => Some(moves!("L' B' R B' R' B2 L")),
            0b011001010 => Some(moves!("F' U' F L F' L' U L F L'")), // bad
            0b100101001 => Some(moves!("F U F' R' F R U' R' F' R")), // bad
            0b101000100 => Some(moves!("L' R2 B R' B R B2 R' B L R'")), // bad
            0b001011010 => Some(moves!("L R2 F' R F' R' F2 R F' L' R")), // bad
            0b100001100 => Some(moves!("L F' L' U' L F L' F' U F")),
            0b010011000 => Some(moves!("R' F R U R' F' R F U' F'")),
            0b100101000 => Some(moves!("L' B' L R' U' R U L' B L")),
            0b010011010 => Some(moves!("L F L' R U R' U' L F' L'")),
            0b001101001 => Some(moves!("F' L F L' U2 L' U B' U B L")), // bad
            0b101001001 => Some(moves!("F R U R' U F' U2 F' L F L'")), // bad
            0b001101011 => Some(moves!("R' U2 F R U R' U' F2 U2 F R")), // bad
            0b001001001 => Some(moves!("L' R' F' U2 L2 U2 L2 U2 L2 F L R")), // bad
            0b010100010 => Some(moves!("R U R' U R U' R' U R U2 R'")),
            0b010100100 => Some(moves!("R U2 R2 U' R2 U' R2 U2 R")),
            0b000010100 => Some(moves!("R2 D R' U2 R D' R' U2 R'")),
            0b010000000 => Some(moves!("R U R D R' U' R D' R2")),
            0b010000100 => Some(moves!("F' L F R' F' L' F R")),
            0b000010010 => Some(moves!("R' U' R U' R' U2 R")),
            0b100000100 => Some(moves!("R U R' U R U2 R'")),
            0b000001001 => Some(moves!("R2 F2 L F L' F2 R F' R")), // bad
            0b011001000 => Some(moves!("F R' U' R2 U' R2 U2 R U' F'")), // bad
            0b010001001 => Some(moves!("F' L U L2 U L2 U2 L' U F")), // bad
            0b101011000 => Some(moves!("R' U' F U R U' R' F' R")),
            0b000001101 => Some(moves!("L U F' U' L' U L F L'")), // bad
            0b100011000 => Some(moves!("R U R' U' R' F R F'")),
            0b010001000 => Some(moves!("R U R' U' B' R' F R F' B")),
            0b001010000 => Some(moves!("R U2 R2 F R F' R U2 R'")),
            0b001101000 => Some(moves!("R' F' U' F2 U R U' R' F' R")),
            0b000011001 => Some(moves!("F R' F' R U R U' R'")),
            0b100001011 => Some(moves!("L F U F2 U' L' U L F L'")), // bad
            0b100001010 => Some(moves!("L F' L' U' L U F U' L'")),
            0b000101000 => Some(moves!("R' F R U R' U' F' U R")),
            0b100001001 => Some(moves!("F U R2 D R' U' R D' R2 F'")),
            0b101001000 => Some(moves!("F' U' L2 D' L U L' D L2 F")), // bad
            0b000011101 => Some(moves!("R' U' F' U F R")),
            0b000001011 => Some(moves!("F U R U' R' F'")),
            0b010101000 => Some(moves!("F R U R' U' F'")),
            0b001000011 => Some(moves!("R' U' R' F R F' U R")),
            0b100101011 => Some(moves!("R' F' U' F U F' U' F U R")),
            0b010101101 => Some(moves!("F R U R' U' R U R' U' F'")),
            0b011101100 => Some(moves!("L F' L2 B L2 F L2 B' L")),
            0b011100100 => Some(moves!("L' B L2 F' L2 B' L2 F L'")),
            0b100011010 => Some(moves!("F U R U' R' U R U' R' F'")),
            0b011100101 => Some(moves!("R' F' U' F U' R U R' U R")),
            0b011100010 => Some(moves!("L' B' R B' R' B R B' R' B2 L")),
            0b011101010 => Some(moves!("L F R' F R F' R' F R F2 L'")),
            0b011100011 => Some(moves!("R U2 R2 U' R U' R' U2 F R F'")), // bad
            0b010101010 => Some(moves!("R B L B' R2 B U L' U' B' R")),   // bad
            0b001000001 => Some(moves!("F' B U' F U F B' R' F' R")),
            _ => None,
        };
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

fn solve_pll(cube: &mut Cube<3>) -> Vec<Move> {
    use crate::Sticker::*;
    let mut u_moves = 0;
    for _ in 0..4 {
        let moves = match (
            (cube.faces[FLU as usize].side() - cube.faces[FU as usize].side()) & 3,
            (cube.faces[FLU as usize].side() - cube.faces[FUR as usize].side()) & 3,
            (cube.faces[FLU as usize].side() - cube.faces[RU as usize].side()) & 3,
            (cube.faces[FLU as usize].side() - cube.faces[RUB as usize].side()) & 3,
        ) {
            (0, 0, 1, 1) => Some(vec![]),
            (1, 1, 2, 0) => Some(moves!("L2 B2 L' F' L B2 L' F L'")),
            (2, 2, 3, 0) => Some(moves!("L F' L B2 L' F L B2 L2")),
            (3, 2, 0, 1) => Some(moves!("L U' R D2 R' U R L' U' L D2 L' U R'")),
            (0, 0, 3, 2) => Some(moves!("R' U R U' R2 F' U' F U R F R' F' R2")),
            (1, 1, 3, 0) => Some(moves!("R2 D B' U B' U' B D' R2 F' U F")),
            (2, 2, 1, 0) => Some(moves!("F' U' F R2 D B' U B U' B D' R2")),
            (2, 1, 3, 0) => Some(moves!("R2 D' F U' F U F' D R2 B U' B'")),
            (3, 1, 2, 0) => Some(moves!("R U R' F2 D' L U' L' U L' D F2")),
            (2, 0, 3, 1) => Some(moves!("R2 F2 B2 L2 D R2 F2 B2 L2")),
            (0, 0, 1, 2) => Some(moves!("R' U L' U2 R U' R' U2 R L")),
            (1, 1, 0, 0) => Some(moves!("L R U2 R' U' R U2 L' U R'")),
            (2, 2, 1, 1) => Some(moves!("R U' L U2 R' U L' R U' L U2 R' U L'")),
            (0, 2, 3, 1) => Some(moves!("R' U L' U2 R U' L R' U L' U2 R U' L")),
            (2, 1, 1, 2) => Some(moves!("L2 F' L' U' L' U L F L' U2 L U2 L'")),
            (3, 1, 1, 0) => Some(moves!("R2 F R U R U' R' F' R U2 R' U2 R")),
            (0, 1, 3, 0) => Some(moves!("R2 D B2 D' R2 F2 D' L2 D F2")),
            (1, 0, 2, 1) => Some(moves!("R2 U' F B' R2 F' B U' R2")),
            (2, 0, 0, 1) => Some(moves!("R2 U F B' R2 F' B U R2")),
            (3, 2, 2, 1) => Some(moves!("F' U F' U' R' F' R2 U' R' U R' F R F")),
            (0, 2, 1, 1) => Some(moves!("R2 U' R2 U' R2 U F U F' R2 F U' F'")),
            (1, 0, 0, 1) => Some(moves!("R B' R' B F R' F B' R' B R F2")),
            _ => None,
        };
        if let Some(moves) = moves {
            for _ in 0..u_moves {
                cube.do_move(Move::U3);
            }
            let mut rotated_moves: Vec<Move> = moves
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
            let auf = match cube.faces[FU as usize] {
                Color::GREEN => None,
                Color::ORANGE => Some(Move::U),
                Color::BLUE => Some(Move::U2),
                Color::RED => Some(Move::U3),
                _ => unreachable!(),
            };
            if let Some(move_) = auf {
                rotated_moves.push(move_);
                cube.do_move(move_);
            }
            return rotated_moves;
        }
        u_moves += 1;
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
    use super::{cfop, solve_cross, Cube, NUM_CROSSES};
    use crate::{cub3, r#move::Move, solvers::cfop::reduce_moves};

    #[test]
    fn test_is_cross_solved() {
        let mut cube = cub3!();
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
        let mut cube = cub3!();
        assert_eq!(cube.cross_index(), 0);
        cube.do_move(Move::R);
        cube.do_move(Move::U);
        cube.do_move(Move::R3);
        cube.do_move(Move::U3);
        assert_eq!(cube.cross_index(), 0);
    }

    #[test]
    fn test_cross_index_random() {
        let mut cube = cub3!();
        for _ in 0..100 {
            cube.do_move(Move::random());
            assert!(cube.cross_index() < NUM_CROSSES);
        }
    }

    #[test]
    fn test_solve_cross() {
        for _ in 0..10 {
            let mut cube = cub3!();
            cube.rand_scramble(100);
            let solution = solve_cross(&mut cube);
            assert!(cube.is_cross_solved());
            assert!(solution.len() <= 8);
        }
    }

    #[test]
    fn test_moves_reduction() {
        assert_eq!(
            reduce_moves(&vec![Move::R, Move::L]),
            vec![Move::R, Move::L]
        );
        assert_eq!(
            reduce_moves(&vec![Move::L, Move::D]),
            vec![Move::L, Move::D]
        );
        assert_eq!(reduce_moves(&vec![Move::U, Move::U]), vec![Move::U2]);
        assert_eq!(reduce_moves(&vec![Move::L, Move::L2]), vec![Move::L3]);
        assert_eq!(reduce_moves(&vec![Move::U, Move::U3]), vec![]);
        assert_eq!(reduce_moves(&vec![Move::L2, Move::L]), vec![Move::L3]);
        assert_eq!(reduce_moves(&vec![Move::R2, Move::R2]), vec![]);
        assert_eq!(reduce_moves(&vec![Move::B2, Move::B3]), vec![Move::B]);
        assert_eq!(reduce_moves(&vec![Move::D3, Move::D]), vec![]);
        assert_eq!(reduce_moves(&vec![Move::B3, Move::B2]), vec![Move::B]);
        assert_eq!(reduce_moves(&vec![Move::F3, Move::F3]), vec![Move::F2]);
        assert_eq!(
            reduce_moves(&vec![Move::R, Move::R, Move::R, Move::R]),
            vec![]
        );
        assert_eq!(
            reduce_moves(&vec![
                Move::U,
                Move::U,
                Move::R,
                Move::R,
                Move::R,
                Move::R,
                Move::U,
                Move::U
            ]),
            vec![]
        );
    }

    #[test]
    fn test_cfop_solves_cube() {
        for _ in 0..100 {
            let mut cube = cub3!();
            let scramble = cube.rand_scramble(100);
            let solution = cfop(&mut cube);
            assert!(
                cube.is_solved(),
                "SCRAMBLE: {scramble:?}\nSOLUTION: {solution:?}\n{cube}"
            );
        }
    }
}
