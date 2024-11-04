use clap::{Parser, ValueEnum};
use rubik::{
    puzzles::{Move, PuzzleArg},
    visualizer::visualize,
};

// TODO: help messages
#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum Mode {
    Cli,
    Gui,
    Karaoke,
}

#[derive(Parser, Debug)]
#[command(name = "rubik", about, long_about = None)]
struct Args {
    #[arg(long, help = "Show the different steps")]
    explain: bool,

    #[arg(long, short, default_value_t = PuzzleArg::Cube3)]
    puzzle: PuzzleArg,

    #[arg(long, short, value_enum, default_value_t = Mode::Cli)]
    mode: Mode,

    #[arg(long, short, help = "Specify a scramble sequence for the puzzle")]
    scramble: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut puzzle = args.puzzle.build();

    if let Some(sequence) = args.scramble {
        puzzle.scramble(&sequence);
    } else {
        let sequence = puzzle.rand_scramble(50);
        println!(
            "No scramble sequence provided, using the following one:\n{}",
            Move::format_sequence(&sequence)
        );
    }

    println!("{puzzle}");

    let solution = puzzle.solve().unwrap_or_else(|| {
        println!("Failed to find solution.");
        std::process::exit(1);
    });

    if solution.is_empty() {
        println!("The puzzle was already solved!");
    } else {
        println!("Solution of {} moves found:", solution.len());
        println!("{}", Move::format_sequence(&solution));
    }

    if args.mode != Mode::Cli {
        visualize(
            &mut puzzle,
            &solution,
            args.mode == Mode::Karaoke,
            // TODO: no playground bool
        );
    }
}

#[cfg(test)]
mod tests {
    use rubik::{
        cub3,
        solvers::{premover, zz},
        Cube, Puzzle,
    };
    use serial_test::serial;

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
    }

    #[test]
    #[serial]
    fn test_performances_1000() {
        test_performances_n(1000);
    }

    #[test]
    #[serial]
    fn test_performances_100000() {
        test_performances_n(100000);
    }
}
