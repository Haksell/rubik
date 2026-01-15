use {
    clap::Parser as _,
    rubik::{
        Args, PuzzleArg, cub2, cub3,
        puzzles::{Puzzle as _, Pyraminx},
    },
};

#[kiss3d::main]
async fn main() {
    let args = Args::parse();

    match args.puzzle {
        PuzzleArg::Cube2 => cub2!().main(args).await,
        PuzzleArg::Cube3 => cub3!().main(args).await,
        PuzzleArg::Pyraminx => Pyraminx::new().main(args).await,
    }
}

#[cfg(test)]
mod tests {
    use {
        rubik::{
            cub3,
            r#move::Move,
            puzzles::Puzzle as _,
            solvers::{premover, zz},
        },
        serial_test::serial,
    };

    fn test_performances_n(n: usize) {
        let mut total_moves = 0;
        for _ in 0..n {
            let mut cube = cub3!();
            let _: Vec<Move> = cube.rand_scramble();
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
        test_performances_n(100_000);
    }
}
