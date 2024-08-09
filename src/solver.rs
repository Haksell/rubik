use std::collections::{HashSet, VecDeque};

use crate::{cube::{self, Cube}, r#move::Move};

pub fn bfs_solve(start: Cube) -> Option<String> {
	let goal = cube::Cube::new(start.size);
	let mut queue: VecDeque<(u32, Cube, String)> = VecDeque::new();
	let mut seen: HashSet<Cube> = HashSet::new();

	queue.push_back((0, start, String::new()));

	
	while !queue.is_empty() {
		let (d, cur, path) = queue.pop_front().unwrap();
		if cur == goal {
			return Some(path);
		}

		if seen.contains(&cur) {
			continue;
		}

		seen.insert(cur.clone());

		for m in 0..18 {
			let _move = Move::from_int(m).unwrap();
			let mut new = cur.clone();
			new.do_move(_move);

			queue.push_back((d + 1, new, path.clone() + " " + &_move.to_string()));
		}
	}
	None
}
