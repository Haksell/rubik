#![allow(dead_code)] // TODO: REMOVE

use rubik::{cub3, cube::Cube, solvers::zz};

fn main() {
    const TESTS: usize = 1;
    const SCRAMBLE_LENGTH: usize = 30;

    let mut total_moves = 0;
    for _ in 0..TESTS {
        let mut cube = cub3!();
        let scramble = cube.rand_scramble(SCRAMBLE_LENGTH);
        println!("{cube}");
        let solution = zz(&mut cube);
        assert!(cube.is_eo_line_solved());
        println!("{scramble:?}");
        println!("{solution:?}");
        println!("{cube}");
        total_moves += solution.len();
    }
    println!("{} avg", total_moves as f32 / TESTS as f32);
}
