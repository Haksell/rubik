use crate::{r#move::Move, Cube, Puzzle};

pub fn iddfs(mut start: Cube<2>) -> Vec<Move> {
    fn search(cur: &mut Cube<2>, path: &mut Vec<Move>, max_depth: usize) -> Option<Vec<Move>> {
        if cur.is_solved() {
            return Some(path.clone());
        }

        if path.len() == max_depth {
            return None;
        }

        for &move_ in &[
            Move::R,
            Move::R2,
            Move::R3,
            Move::F,
            Move::F2,
            Move::F3,
            Move::U,
            Move::U2,
            Move::U3,
        ] {
            if !path.is_empty() && path.last().unwrap().same_face(&move_) {
                continue;
            }

            cur.do_move(move_);
            path.push(move_);

            if let Some(moves) = search(cur, path, max_depth) {
                return Some(moves);
            }

            path.pop();
            cur.do_move(move_.opposite());
        }
        None
    }

    let mut max_depth = 1;
    loop {
        if let Some(path) = search(&mut start, &mut Vec::new(), max_depth) {
            return path;
        }
        max_depth += 1;
    }
}
