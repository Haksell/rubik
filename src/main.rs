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

    let cube = cube::Cube::new(2);
    println!("{}", cube.faces.len());

    cube.scramble(&args[1])?;
    println!("{}", cube);

    Ok(())
}
