#![allow(dead_code)] // TODO: REMOVE

use clap::Parser;
use rubik::{
    cub3,
    solvers::{premover, zz},
    tables::clear_cache,
    Cube,
};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long)]
    visualize: bool,

    #[arg(long)]
    explain: bool, // TODO Comprendre

    #[arg(long)]
    pyraminx: bool,

    #[arg(index(1))]
    scramble: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut cube = cub3!();

    if let Some(sequence) = args.scramble {
        cube.scramble(&sequence);
        println!("{cube}");
    } else {
        let sequence = cube.rand_scramble(50);
        println!(
            "No scramble sequence provided, using the following one:\n{}",
            sequence
                .iter()
                .map(|move_| format!("{:?}", move_))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    if args.visualize {
        // TODO Visualize solution
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
