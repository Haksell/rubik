use {
    super::{
        last_layer::{oll_matcher, pll_matcher, solve_auf, solve_last_layer_step},
        reduce_moves,
    },
    crate::{
        Cube, EDGES, Puzzle,
        color::Color,
        r#move::Move,
        tables::{FILE_CROSSES, read_moves},
        trigger::{TRIGGERS_BY_SLOT, Trigger},
    },
    std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap},
    },
};

pub const NUM_CROSSES: usize = 24 * 22 * 20 * 18;

// TODO: think about Cube::<3>.cfop or cfop(&mut Cube<3>)
// (same for the other solvers)

pub fn cfop(cube: &mut Cube<3>) -> Vec<Move> {
    let mut solution = vec![];
    solution.extend(solve_cross(cube));
    solution.extend(solve_f2l(cube)); // TODO: optimize over several consecutive pairs
    solution.extend(solve_last_layer_step(cube, oll_matcher));
    solution.extend(solve_last_layer_step(cube, pll_matcher));
    solution.extend(solve_auf(cube));
    reduce_moves(&solution)
}

fn solve_cross(cube: &mut Cube<3>) -> Vec<Move> {
    let cross_moves = read_moves(FILE_CROSSES)
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
    let mut solution = vec![];
    let mut to_solve = (0..4).fold(0, |acc, slot| {
        acc | ((!cube.is_pair_solved(slot) as u8) << slot)
    }); // handles accidental x-crosses
    while to_solve != 0 {
        let mut triggers = vec![Trigger::U, Trigger::U2, Trigger::U3];
        for slot in 0..4 {
            if to_solve & (1 << slot) != 0 {
                triggers.extend(TRIGGERS_BY_SLOT[slot]);
            }
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

fn reconstruct_path<const N: usize>(
    cube: &mut Cube<N>,
    came_from: &HashMap<Cube<N>, Option<Trigger>>,
) -> Vec<Trigger> {
    let mut path: Vec<Trigger> = vec![];

    while let Some(trigger) = came_from[cube] {
        trigger
            .moves()
            .iter()
            .rev()
            .map(Move::opposite)
            .for_each(|move_| cube.do_move(move_));
        path.insert(0, trigger);
    }

    path
}

fn solve_pair(cube: &Cube<3>, triggers: &[Trigger]) -> Vec<Trigger> {
    let mut came_from: HashMap<Cube<3>, Option<Trigger>> = HashMap::new();
    let mut g_cost: HashMap<Cube<3>, usize> = HashMap::new();
    let mut heap: BinaryHeap<(Reverse<usize>, Cube<3>)> = BinaryHeap::new();

    came_from.insert(cube.clone(), None);
    g_cost.insert(cube.clone(), 0);

    heap.push((Reverse(0), cube.clone()));

    loop {
        let (Reverse(num_moves), mut cube) = heap.pop().unwrap();
        let slot = match came_from[&cube] {
            Some(trigger) => trigger.slot(),
            None => usize::MAX,
        };

        if slot != usize::MAX && cube.is_pair_solved(slot) {
            return reconstruct_path(&mut cube, &came_from);
        }

        for &trigger in triggers {
            if slot == usize::MAX || trigger.slot() != slot {
                let mut next_cube = cube.clone();

                next_cube.do_trigger(trigger);

                if *g_cost.get(&next_cube).unwrap_or(&usize::MAX) <= num_moves + trigger.len() {
                    continue;
                }

                g_cost.insert(next_cube.clone(), num_moves + trigger.len());
                came_from.insert(next_cube.clone(), Some(trigger));

                heap.push((Reverse(num_moves + trigger.len()), next_cube));
            }
        }
    }
}

impl Cube<3> {
    #[cfg(test)]
    fn is_cross_solved(&self) -> bool {
        use crate::Sticker::*;
        self.faces[DF as usize] == Color::YELLOW
            && self.faces[DR as usize] == Color::YELLOW
            && self.faces[DB as usize] == Color::YELLOW
            && self.faces[DL as usize] == Color::YELLOW
            && self.faces[FD as usize] == Color::GREEN
            && self.faces[RD as usize] == Color::RED
            && self.faces[BD as usize] == Color::BLUE
            && self.faces[LD as usize] == Color::ORANGE
    }

    pub fn cross_index(&self) -> usize {
        let mut yellow_green = usize::MAX;
        let mut yellow_blue = usize::MAX;
        let mut yellow_red = usize::MAX;
        let mut yellow_orange = usize::MAX;
        for (i, &(s1, s2)) in EDGES.iter().enumerate() {
            if self.faces[s1 as usize] == Color::YELLOW {
                match self.faces[s2 as usize] {
                    Color::GREEN => yellow_green = 2 * i,
                    Color::BLUE => yellow_blue = 2 * i,
                    Color::RED => yellow_red = 2 * i,
                    Color::ORANGE => yellow_orange = 2 * i,
                    _ => unreachable!(),
                }
            } else if self.faces[s2 as usize] == Color::YELLOW {
                match self.faces[s1 as usize] {
                    Color::GREEN => yellow_green = 2 * i + 1,
                    Color::BLUE => yellow_blue = 2 * i + 1,
                    Color::RED => yellow_red = 2 * i + 1,
                    Color::ORANGE => yellow_orange = 2 * i + 1,
                    _ => unreachable!(),
                }
            }
        }
        if yellow_blue > yellow_green {
            yellow_blue -= 2;
        }
        if yellow_red > yellow_green {
            yellow_red -= 2;
        }
        if yellow_red > yellow_blue {
            yellow_red -= 2;
        }
        if yellow_orange > yellow_green {
            yellow_orange -= 2;
        }
        if yellow_orange > yellow_blue {
            yellow_orange -= 2;
        }
        if yellow_orange > yellow_red {
            yellow_orange -= 2;
        }
        yellow_orange + 18 * yellow_red + 18 * 20 * yellow_blue + 18 * 20 * 22 * yellow_green
    }

    fn is_pair_solved(&self, index: usize) -> bool {
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
    use super::{NUM_CROSSES, cfop, solve_cross};
    use crate::{Cube, Puzzle as _, cub3, r#move::Move};

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
