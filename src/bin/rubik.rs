use clap::Parser;
use rubik::{
    cub2, cub3, r#move::Move, tables::clear_cache, visualizer::visualize, Cube, Puzzle, Pyraminx,
};

#[derive(Parser, Debug)]
#[command(name = "rubik", about, long_about = None)]
struct Args {
    /// Enable visualizer window after solve.
    #[arg(short, long)]
    visualize: bool,

    /// Display solving movements colored gradually in the visualizer.
    #[arg(long)]
    karaoke: bool,

    /// Start a movable visualizer with the specified or default puzzle and size.
    #[arg(long)]
    playground: bool,

    #[arg(long)]
    explain: bool, // TODO Comprendre

    /// Select the puzzle between "Pyraminx" and "Cube". Defaults to Cube.
    #[arg(long)]
    puzzle: Option<String>,

    /// Size of the puzzle (2 <= size <= 3). Defaults to 3.
    #[arg(long)]
    size: Option<usize>,

    /// Specify a scramble sequence for the puzzle
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
        ("pyraminx", 3) => Box::new(Pyraminx::new()), // TODO: different pyraminx sizes
        _ => panic!(
            "Invalid puzzle '{}'. Expected 'Cube' or 'Pyraminx'",
            puzzle_name
        ),
    };

    if args.playground {
        visualize(&mut puzzle, &vec![], false);
        return;
    }

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

    if let Some(solution) = puzzle.solve() {
        if solution.is_empty() {
            println!("The {} was already solved!", puzzle_name.to_lowercase());
        } else {
            println!("Solution of {} moves found:", solution.len());
            println!("{}", Move::format_sequence(&solution));
        }
        if args.visualize {
            visualize(&mut puzzle, &solution, args.karaoke);
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
        clear_cache();
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
