use rubik::{
    cub3,
    r#move::Move,
    solvers::{MOVES_RUL, NUM_ZZ_LEFT},
    tables::{write_moves, FILE_ZZ_LEFT},
    Cube,
};
use std::{collections::VecDeque, io};

fn main() -> io::Result<()> {
    let cube = cub3!();
    let mut moves: [Option<Move>; NUM_ZZ_LEFT] = [None; NUM_ZZ_LEFT];
    let mut queue = VecDeque::new();
    queue.push_back((cube, Move::D));
    let mut remaining_cases = NUM_ZZ_LEFT;
    while remaining_cases > 0 {
        if queue.is_empty() {
            println!("{remaining_cases}");
            for (i, opt) in moves.iter().enumerate() {
                if opt.is_none() {
                    print!("{i} ");
                }
            }
            println!();
        }
        let (cube, last_move) = queue.pop_front().unwrap();
        let idx = cube.zz_left_index();
        if moves[idx].is_some() {
            continue;
        }
        moves[idx] = Some(last_move.opposite());
        remaining_cases -= 1;
        if remaining_cases % 1000 == 0 {
            println!("{remaining_cases}");
        }
        for &move_ in &MOVES_RUL {
            if remaining_cases == NUM_ZZ_LEFT - 1 || !move_.same_face(&last_move) {
                let mut next_cube = cube.clone();
                next_cube.do_move(move_);
                queue.push_back((next_cube, move_));
            }
        }
    }

    write_moves(FILE_ZZ_LEFT, &moves)
}
