#![allow(dead_code)] // TODO: REMOVE

use rubik::cube::Cube;
use rubik::{cub3, solvers};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("Usage: ./rubik <scramble>");
    }
    let mut cube = cub3!();

    println!("{cube}");
    if args.len() == 2 {
        cube.scramble(&args[1]);
    } else {
        let scramble = cube.rand_scramble(40);
        println!("Scramble sequence: {scramble:?}");
    }
    println!("{cube}");
    let solution = solvers::cfop(&mut cube);
    println!("Solution found with {} moves:", solution.len());
    println!("{solution:?}");
    println!("{cube}");
}
