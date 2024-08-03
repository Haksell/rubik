use std::env;
use std::fmt::Error;
mod cube;
mod r#move;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./rubik <mixing sequence>");
        return Err(Error); // TODO Better error handling
    }

    let mut cube = cube::Cube::new(5);

    cube.scramble(&args[1])?;

    Ok(())
}
