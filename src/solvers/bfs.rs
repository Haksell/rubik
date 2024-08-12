use crate::{cube::Cube, r#move::Move};
use std::collections::{HashSet, VecDeque};

// TODO: cleanup
// TODO: accept Cube<2> or Pyraminx
pub fn bfs(start: Cube<2>) -> Option<Vec<Move>> {
    let goal = Cube::new();
    let mut queue: VecDeque<(Cube<2>, Vec<Move>)> = VecDeque::new();
    let mut seen: HashSet<Cube<2>> = HashSet::new();

    queue.push_back((start, Vec::new()));

    while !queue.is_empty() {
        let (cur, path) = queue.pop_front().unwrap();

        if cur == goal {
            return Some(path);
        }

        seen.insert(cur.clone());

        for _move in Move::iterator() {
            if !path.is_empty() && *path.last().unwrap() == _move {
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
