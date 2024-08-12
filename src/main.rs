mod color;
mod cube;
mod r#move;
mod solvers;

use cube::Cube;
use std::env;
use std::fmt::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: ./rubik <scramble>");
        return Err(Error); // TODO Better error handling
    }

    let mut cube = Cube::<2>::new();

    if args.len() == 2 {
        cube.scramble(&args[1])?;
    } else {
        cube.rand_scramble(10);
    }

    println!("{}", cube);

    let solution = solvers::bfs(cube.clone());

    match solution {
        None => {
            println!("Solution not found");
        }
        Some(moves) => {
            println!("Solution found with {} moves:", moves.len());
            println!("{:?}", moves);
        }
    }

    Ok(())
}
