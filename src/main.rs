mod color;
mod cube;
mod r#move;
mod solvers;

use cube::Cube;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("Usage: ./rubik <scramble>");
    }

    let mut cube = Cube::<2>::new();

	println!("{cube}");

    if args.len() == 2 {
        cube.scramble(&args[1]);
    } else {
        let scramble = cube.rand_scramble(5);
		println!("Scramble sequence: {scramble:?}");
    }

    println!("{cube}");

    let solution = solvers::iddfs(cube.clone());

    match solution {
        None => {
            println!("Solution not found");
        }
        Some(moves) => {
            println!("Solution found with {} moves:", moves.len());
            println!("{moves:?}");
        }
    }
}
