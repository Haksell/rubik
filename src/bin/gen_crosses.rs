use rubik::{cube::Cube, filenames::FILE_CROSSES, r#move::Move, solvers::NUM_CROSSES};
use std::{
    collections::VecDeque,
    fs::File,
    io::{self, Write as _},
};

fn main() -> io::Result<()> {
    let cube = Cube::<3>::new();
    let mut moves: [Option<Move>; NUM_CROSSES] = [None; NUM_CROSSES];
    let mut queue = VecDeque::new();
    queue.push_back((cube, Move::R));
    let mut remaining_crosses = NUM_CROSSES;
    while remaining_crosses > 0 {
        let (cube, last_move) = queue.pop_front().unwrap();
        let idx = cube.cross_index();
        if moves[idx].is_some() {
            continue;
        }
        remaining_crosses -= 1;
        moves[idx] = Some(last_move.opposite());
        for move_ in Move::iterator() {
            let mut next_cube = cube.clone();
            next_cube.do_move(move_);
            queue.push_back((next_cube, move_));
        }
    }

    write_file(&moves)
}

fn write_file(moves: &[Option<Move>; NUM_CROSSES]) -> io::Result<()> {
    let mut file = File::create(FILE_CROSSES)?;
    for opt_move in moves {
        let move_byte = match opt_move {
            Some(m) => *m as u8,
            None => unreachable!(),
        };
        file.write_all(&[move_byte])?;
    }
    Ok(())
}
