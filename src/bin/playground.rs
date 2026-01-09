use {
    clap::Parser,
    rubik::{puzzles::PuzzleArg, visualizer::visualize},
};

// Completely useless for now (keyboard turns not implemented)

// TODO: explain and karaoke later
#[derive(Parser, Debug)]
#[command(name = "playground", about, long_about = None)]
struct Args {
    #[arg(long, short, value_enum, default_value_t = PuzzleArg::Cube3)]
    puzzle: PuzzleArg,
}

#[kiss3d::main]
async fn main() {
    let args = Args::parse();
    let mut puzzle = args.puzzle.build();
    visualize(&mut puzzle, &[], false).await;
}
