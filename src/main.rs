#![allow(dead_code)] // TODO: REMOVE

use clap::Parser;
use rubik::{cub2, cub3, tables::clear_cache, visualize, Cube, Puzzle, Pyraminx};

#[derive(Parser, Debug)]
#[command(name = "rubik", about, long_about = None)]
struct Args {
    #[arg(short, long)]
    visualize: bool,

    #[arg(long)]
    explain: bool, // TODO Comprendre

    #[arg(long)]
    puzzle: Option<String>,

    #[arg(long)]
    size: Option<usize>,

    #[arg(index(1))]
    scramble: Option<String>,
}

fn main() {
    let args = Args::parse();

    let size: usize = args.size.unwrap_or(3);
    let puzzle_name: String = args.puzzle.unwrap_or("Cube".to_string());

    match size {
        2..=3 => (),
        _ => panic!("Size should be between 2 or 3 included"),
    };

    let mut puzzle: Box<dyn Puzzle> = match (puzzle_name.to_lowercase().as_str(), size) {
        ("cube", 2) => Box::new(cub2!()),
        ("cube", 3) => Box::new(cub3!()),
        ("pyraminx", 2) => panic!("Pyraminx can only be of size 3"),
        ("pyraminx", 3) => Box::new(Pyraminx::<3>::new()),
        _ => panic!(
            "Invalid puzzle '{}'. Expected 'Cube' or 'Pyraminx'",
            puzzle_name
        ),
    };

    if let Some(sequence) = args.scramble {
        puzzle.scramble(&sequence);
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

    println!("{puzzle}");

    if let Some(solution) = puzzle.solve() {
        if solution.is_empty() {
            println!("The {} was already solved!", puzzle_name.to_lowercase());
        } else {
            println!("Solution of {} moves found:", solution.len());
            println!(
                "{}",
                solution
                    .iter()
                    .map(|move_| format!("{:?}", move_))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }
        if args.visualize {
            // TODO Visualize solution
            puzzle.visualize(&solution);
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
        Cube, Puzzle,
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

    #[test]
    fn test_performances_100000() {
        test_performances_n(100000);
    }
}
