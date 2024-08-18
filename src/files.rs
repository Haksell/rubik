use std::{
    fs::{self, File},
    io::{self, Read as _, Write as _},
    path::Path,
};

use crate::{r#move::Move, solvers::NUM_CROSSES};

pub const FILE_CROSSES: &'static str = "tables/cfop/crosses.bin";
pub const FILE_EO_LINES: &'static str = "tables/zz/eo_lines.bin";

// TODO: don't depend on NUM_CROSSES to be generic

// TODO: read only once in benchmark tests
pub fn read_moves(filename: &str) -> io::Result<[Move; NUM_CROSSES]> {
    let mut file = File::open(filename)?;
    let mut moves = [Move::U; NUM_CROSSES];
    let buffer =
        unsafe { std::slice::from_raw_parts_mut(moves.as_mut_ptr() as *mut u8, NUM_CROSSES) };
    file.read_exact(buffer)?;
    Ok(moves)
}

pub fn write_moves(filename: &str, moves: &[Option<Move>]) -> io::Result<()> {
    if let Some(parent_dir) = Path::new(filename).parent() {
        fs::create_dir_all(parent_dir)?;
    }
    let mut file = File::create(filename)?;
    for opt_move in moves {
        let move_byte = match opt_move {
            Some(m) => *m as u8,
            None => unreachable!(),
        };
        file.write_all(&[move_byte])?;
    }
    Ok(())
}
