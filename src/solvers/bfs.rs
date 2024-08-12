use crate::{cube::Cube, r#move::Move};
use std::collections::{HashSet, VecDeque};

// TODO: cleanup
// TODO: accept Cube<2> or Pyraminx
#[allow(dead_code)]
pub fn bfs(start: Cube<2>) -> Option<Vec<Move>> {
    let mut queue: VecDeque<(Cube<2>, Vec<Move>)> = VecDeque::new();
    let mut seen: HashSet<Cube<2>> = HashSet::new();

    queue.push_back((start, Vec::new()));

    while !queue.is_empty() {
        let (cur, path) = queue.pop_front().unwrap();

        if cur.is_solved() {
            return Some(path);
        }

        seen.insert(cur.clone());

        for _move in Move::iterator() {
            if !path.is_empty() && *path.last().unwrap() == _move.opposite() {
                continue;
            }

            let mut new = cur.clone();
            new.do_move(_move);

            if seen.contains(&new) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(_move);

            queue.push_back((new, new_path));
        }
    }
    None
}

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

            match search(cur, new_path, max_depth) {
                Some(moves) => return Some(moves),
                None => (),
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
