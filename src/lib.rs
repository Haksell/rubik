pub mod color;
pub mod r#move;
pub mod puzzles;
pub mod solvers;
pub mod sticker;
pub mod tables;
pub mod trigger;
pub mod visualizer;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "rubik", about, long_about = None)]
pub struct Args {
    #[arg(long, short, help = "Specify a scramble sequence for the puzzle")]
    pub scramble: Option<String>,

    #[arg(long, short, value_enum, default_value_t = Mode::Cli)]
    pub mode: Mode,

    #[arg(long, short, value_enum, default_value_t = PuzzleArg::Cube3)]
    pub puzzle: PuzzleArg,
    // #[arg(long, short, help = "Show the different steps")]
    // explain: bool,
}

// TODO: help messages
#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    Cli,
    Gui,
    Karaoke,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum PuzzleArg {
    #[value(alias = "2", alias = "2x2", alias = "2x2x2")]
    Cube2,
    #[value(alias = "3", alias = "3x3", alias = "3x3x3")]
    Cube3,
    #[value(alias = "pyra")]
    Pyraminx,
}
