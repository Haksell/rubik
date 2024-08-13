use crate::{cube::Cube, r#move::Move};

#[allow(dead_code)]
pub fn iddfs(mut start: Cube<2>) -> Option<Vec<Move>> {
    fn search(cur: &mut Cube<2>, path: Vec<Move>, max_depth: usize) -> Option<Vec<Move>> {
        if cur.is_solved() {
            return Some(path);
        }

        if path.len() == max_depth {
            return None;
        }

        for _move in Move::iterator() {
            //if ![Move::R, Move::U, Move::F].contains(&_move) {
            //    continue;
            //}
            if !path.is_empty() && *path.last().unwrap() == _move.opposite() {
                continue;
            }

            cur.do_move(_move);

            let mut new_path = path.clone();
            new_path.push(_move);

			if let Some(moves) = search(cur, new_path, max_depth) {
				return Some(moves);
			}

            cur.do_move(_move.opposite());
        }
        None
    }

    let mut max_depth = 1;
    while max_depth < 10000 {
        match search(&mut start, Vec::new(), max_depth) {
            Some(moves) => return Some(moves),
            None => (),
        }
        max_depth += 1;
    }
    None
}
