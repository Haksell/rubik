mod color;
mod cube;
mod r#move;
mod solvers;

use cube::Cube;
use std::env;
use std::fmt::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./rubik <scramble>");
        return Err(Error); // TODO Better error handling
    }

    let mut cube = Cube::<2>::new();
    cube.scramble(&args[1])?;

    println!("{}", cube);
    let solution = solvers::bfs(cube.clone());
    println!("{solution:?}");

    Ok(())
}
