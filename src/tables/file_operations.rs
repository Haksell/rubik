use crate::r#move::Move;
use std::{
    fs::{self, File},
    io::{self, Read as _, Write as _},
    path::Path,
};

// TODO: read only once in benchmark tests
pub fn read_moves(filename: &str) -> io::Result<Vec<Move>> {
    let mut file = File::open(filename)?;
    let file_size = file.metadata()?.len() as usize;
    let mut moves = Vec::with_capacity(file_size);
    unsafe {
        moves.set_len(file_size);
        let buffer = std::slice::from_raw_parts_mut(moves.as_mut_ptr() as *mut u8, file_size);
        file.read_exact(buffer)?;
    }
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
