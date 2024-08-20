use rubik::{
    cub3,
    r#move::{Move, MOVES_RUF},
    solvers::NUM_1LLL,
    Cube,
};
pub const DUMMY_MOVE: Move = Move::D; // could be anything
const ARRAY_REPEAT_VALUE: Option<Vec<Move>> = None; // required because Vec<Move> is not Copy

fn dfs(
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
            solved_cases += dfs(solutions, cube, moves, max_depth);
            moves.pop();
            cube.do_move(move_.opposite());
        }
    }

    solved_cases
}

fn main() {
    let mut cube = cub3!();
    cube.scramble("R U R' U' D2");
    print!("{cube}");
    let n = cube.serialize();
    println!("{}", n);
    print!("{}", Cube::deserialize(n));
    print!("{}", Cube::deserialize(n).serialize());

    // let mut solutions: [Option<Vec<Move>>; NUM_1LLL] = [ARRAY_REPEAT_VALUE; NUM_1LLL];
    // let mut remaining_cases = NUM_1LLL;
    // for max_depth in 0.. {
    //     remaining_cases -= dfs(&mut solutions, &mut cub3!(), &mut Vec::new(), max_depth);
    //     println!(
    //         "max_depth: {}, remaining_cases: {}/{}",
    //         max_depth, remaining_cases, NUM_1LLL
    //     );
    //     if remaining_cases == 0 {
    //         return;
    //     }
    // }
}
