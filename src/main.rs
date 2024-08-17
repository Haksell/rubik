#![allow(dead_code)] // TODO: REMOVE

use rubik::{cub3, cube::Cube, solvers::premover};

fn main() {
    const TESTS: usize = 25;
    let mut total_moves = 0;
    for _ in 0..TESTS {
        let mut cube = cub3!();
        let scramble = cube.rand_scramble(200);
        let solution = premover(&mut cube);
        assert!(
            cube.is_solved(),
            "SCRAMBLE: {scramble:?}\nSOLUTION: {solution:?}\n{cube}"
        );
        // println!("{scramble:?}");
        // println!("{solution:?}");
        // println!("{cube}");
        total_moves += solution.len();
    }
    println!("{} avg", total_moves as f32 / TESTS as f32);
}
