use crate::{r#move::Move, solvers::DFSAble};

pub fn iddfs<T: DFSAble>(mut start: T) -> Vec<Move> {
    fn search<T: DFSAble>(
        cur: &mut T,
        path: &mut Vec<Move>,
        max_depth: usize,
    ) -> Option<Vec<Move>> {
        if cur.is_solved() {
            return Some(path.clone());
        }

        if path.len() == max_depth {
            return None;
        }

        for &move_ in T::ALLOWED_MOVES {
            if path
                .last()
                .is_some_and(|last_move| last_move.same_face(&move_))
            {
                continue;
            }

            cur.do_move(move_);
            path.push(move_);

            if let Some(moves) = search(cur, path, max_depth) {
                return Some(moves);
            }

            path.pop();
            cur.do_move(cur.opposite_move(move_));
        }
        None
    }

    let mut max_depth = 1;
    let premoves = start.presolve();
    let mut path = vec![];
    loop {
        if let Some(moves) = search(&mut start, &mut path, max_depth) {
            return [premoves, moves].concat();
        }
        max_depth += 1;
    }
}
