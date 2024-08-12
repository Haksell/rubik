use std::env;
use std::fmt::Error;

mod cube;
mod r#move;
mod solver;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: ./rubik <mixing sequence>");
        return Err(Error); // TODO Better error handling
    }

    let mut cube = cube::Cube::new(2);

    if args.len() == 2 {
        cube.scramble(&args[1])?;
    } else {
        cube.rand_scramble(10);
    }

    println!("{}", cube);

    let solution = solver::bfs_solve(cube.clone());

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
