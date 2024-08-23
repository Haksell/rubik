#![allow(dead_code)] // TODO: REMOVE

use clap::Parser;
use rubik::{
    cub2, cub3,
    r#move::Move,
    solvers::{self, premover, zz},
    tables::clear_cache,
    Cube, Puzzle, Pyraminx,
};

#[derive(Parser, Debug)]
#[command(name = "rubik", about, long_about = None)]
struct Args {
    #[arg(short, long)]
    visualize: bool,

    #[arg(long)]
    explain: bool, // TODO Comprendre

    #[arg(long, default_value_t = String::from("Cube"))]
    puzzle: String,

    #[arg(long, default_value_t = 3)]
    size: usize,

    #[arg(index(1))]
    scramble: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.size {
        2..=3 => (),
        _ => panic!("Size should be between 2 or 3 included"),
    };

    let mut puzzle: Box<dyn Puzzle> = match (args.puzzle.to_lowercase().as_str(), args.size) {
        ("cube", 2) => Box::new(cub2!()),
        ("cube", 3) => Box::new(cub3!()),
        ("pyraminx", 2) => panic!("Pyraminx can only be of size 3"),
        ("pyraminx", 3) => Box::new(Pyraminx::new()),
        _ => panic!(
            "Invalid puzzle '{}'. Expected 'Cube' or 'Pyraminx'",
            args.puzzle
        ),
    };

    if let Some(sequence) = args.scramble {
        puzzle.scramble(&sequence);
        println!("{puzzle}");
    } else {
        let sequence = puzzle.rand_scramble(50);
        println!(
            "No scramble sequence provided, using the following one:\n{}",
            sequence
                .iter()
                .map(|move_| format!("{:?}", move_))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    // TODO Use corresponding solver
    if let Some(solution) = Some(<Vec<Move>>::new()) {
        if args.visualize {
            // TODO Visualize solution
        }
    } else {
        println!("Failed to find solution");
    }
    clear_cache();
}

#[cfg(test)]
mod tests {
    use rubik::{
        cub3,
        solvers::{premover, zz},
        tables::clear_cache,
        Cube,
    };

    fn test_performances_n(n: usize) {
        const SCRAMBLE_LENGTH: usize = 200;

        let mut total_moves = 0;
        for _ in 0..n {
            let mut cube = cub3!();
            let _ = cube.rand_scramble(SCRAMBLE_LENGTH);
            // println!("{cube}");
            let solution = premover(&mut cube, zz);
            assert!(cube.is_solved());
            // println!("{scramble:?}");
            // println!("{solution:?}");
            // println!("{cube}");
            total_moves += solution.len();
        }
        println!("{} avg", total_moves as f32 / n as f32);
        clear_cache();
    }

    #[test]
    fn test_performances_1000() {
        test_performances_n(1000);
    }
}
