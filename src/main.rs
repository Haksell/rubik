#![allow(dead_code)] // TODO: REMOVE

use rubik::solvers::cfop;
use rubik::{cub3, cube::Cube};

fn main() {
    const TESTS: usize = 100;
    let mut total_moves = 0;
    for _ in 0..TESTS {
        let mut cube = cub3!();
        let scramble = cube.rand_scramble(100);
        let solution = cfop(&mut cube);
        assert!(
            cube.is_solved(),
            "SCRAMBLE: {scramble:?}\nSOLUTION: {solution:?}\n{cube}"
        );
        total_moves += solution.len();
    }
    println!("{} avg", total_moves as f32 / TESTS as f32);
}
