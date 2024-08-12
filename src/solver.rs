use crate::{
    cube::{self, Cube},
    r#move::Move,
};
use std::collections::{HashSet, VecDeque};

// TODO Cleanup
pub fn bfs_solve(start: Cube) -> Option<Vec<Move>> {
    let goal = cube::Cube::new(start.size);
    let mut queue: VecDeque<(Cube, Vec<Move>)> = VecDeque::new();
    let mut seen: HashSet<Cube> = HashSet::new();

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
