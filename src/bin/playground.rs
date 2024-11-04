use clap::Parser;
use rubik::{puzzles::PuzzleArg, visualizer::visualize};

// TODO: explain and karaoke later
#[derive(Parser, Debug)]
#[command(name = "playground", about, long_about = None)]
struct Args {
    #[arg(long, short, value_enum, default_value_t = PuzzleArg::Cube3)]
    puzzle: PuzzleArg,
}

fn main() {
    let args = Args::parse();
    let mut puzzle = args.puzzle.build();
    visualize(&mut *puzzle, &vec![], false);
}
