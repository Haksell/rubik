use std::fmt;

use crate::{moves_runtime, r#move::Move};

pub trait Puzzle: fmt::Display {
    fn solve(&self) -> Option<Vec<Move>>;

    fn is_solved(&self) -> bool;

    fn do_move(&mut self, move_: Move);

    fn scramble(&mut self, sequence: &str) {
        moves_runtime!(sequence)
            .iter()
            .for_each(|&move_| self.do_move(move_));
    }

    fn rand_scramble(&mut self, iterations: usize) -> Vec<Move> {
        // TODO Better scrambler
        let mut sequence: Vec<Move> = Vec::new();

        while sequence.len() < iterations {
            let move_ = Move::random();
            if !sequence.is_empty() && move_ == sequence.last().unwrap().opposite() {
                continue;
            }
            self.do_move(move_);
            sequence.push(move_);
        }
        sequence
    }
}
