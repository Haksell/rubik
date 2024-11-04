use clap::Parser;
use rubik::{puzzles::PuzzleArg, visualizer::visualize};

// TODO: explain and karaoke later
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = PuzzleArg::Cube3)]
    puzzle: PuzzleArg,
}

fn main() {
    let args = Args::parse();
    let mut puzzle = args.puzzle.build();
    visualize(&mut puzzle, &vec![], false);
}
