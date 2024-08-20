// TODO: only ZBLL?

use rubik::{
    cub3,
    r#move::{Move, MOVES_RUF},
    solvers::NUM_1LLL,
    Cube,
};
use std::collections::{HashMap, VecDeque};

pub const DUMMY_MOVE: Move = Move::D; // could be anything
const ARRAY_REPEAT_VALUE: Option<Vec<Move>> = None; // required because Vec<Move> is not Copy

fn generate_nearly_solved(max_depth: usize) -> HashMap<u128, Move> {
    let mut table: HashMap<u128, Move> = HashMap::new();
    let solved_serialized = cub3!().serialize();
    let mut level = vec![(solved_serialized, DUMMY_MOVE)];
    for depth in 0..=max_depth {
        let mut next_level = Vec::new();
        for (serialized, last_move) in level {
            let cube = Cube::deserialize(serialized);
            // TODO: only find once in HashMap
            if table.get(&serialized).is_some() {
                continue;
            }
            table.insert(serialized, last_move.opposite());

            if depth == max_depth {
                continue;
            }

            for &move_ in &MOVES_RUF {
                if serialized == solved_serialized || !move_.same_face(&last_move) {
                    let mut next_cube = cube.clone();
                    next_cube.do_move(move_);
                    next_level.push((next_cube.serialize(), move_));
                }
            }
        }
        level = next_level
    }
    table
}

fn get_sequence(table: &HashMap<u128, Move>, mut serialized: u128) -> Vec<Move> {
    let mut sequence = vec![];
    let mut cube = Cube::deserialize(serialized);
    while !cube.is_solved() {
        let move_ = *table.get(&serialized).unwrap();
        cube.do_move(move_);
        sequence.push(move_);
        serialized = cube.serialize();
    }
    sequence
}

fn dfs(
    table: &HashMap<u128, Move>,
    solutions: &mut [Option<Vec<Move>>; NUM_1LLL],
    cube: &mut Cube<3>,
    moves: &mut Vec<Move>,
    max_depth: usize,
) -> usize {
    let mut solved_cases = 0;

    if cube.is_f2l_solved() {
        let idx = cube.last_layer_index();
        if solutions[idx].is_none() {
            solutions[idx] = Some(moves.clone());
            solved_cases += 1;
        }
    }

    if moves.len() == max_depth {
        return solved_cases;
    }

    for &move_ in &MOVES_RUF {
        if moves.is_empty() || !moves.last().unwrap().same_face(&move_) {
            cube.do_move(move_);
            moves.push(move_);
            solved_cases += dfs(table, solutions, cube, moves, max_depth);
            moves.pop();
            cube.do_move(move_.opposite());
        }
    }

    solved_cases
}

fn main() {
    let table = generate_nearly_solved(8);
    let mut solutions: [Option<Vec<Move>>; NUM_1LLL] = [ARRAY_REPEAT_VALUE; NUM_1LLL];
    let mut remaining_cases = NUM_1LLL;
    for max_depth in 0.. {
        remaining_cases -= dfs(
            &table,
            &mut solutions,
            &mut cub3!(),
            &mut Vec::new(),
            max_depth,
        );
        println!(
            "max_depth: {}, remaining_cases: {}/{}",
            max_depth, remaining_cases, NUM_1LLL
        );
        if remaining_cases == 0 {
            return;
        }
    }
}
