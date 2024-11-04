use crate::puzzles::Move;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read as _, Write as _},
    path::Path,
    rc::Rc,
};

// TODO: add Mutex and Arc if multithreading
static mut MOVE_CACHE: Option<HashMap<String, Rc<Vec<Move>>>> = None;

pub fn read_moves(filename: &str) -> io::Result<Rc<Vec<Move>>> {
    unsafe {
        if MOVE_CACHE.is_none() {
            MOVE_CACHE = Some(HashMap::new());
        }

        let cache = MOVE_CACHE.as_mut().unwrap();

        if let Some(moves) = cache.get(filename) {
            return Ok(Rc::clone(moves));
        }

        let mut file = File::open(filename)?;
        let file_size = file.metadata()?.len() as usize;
        let mut moves = Vec::with_capacity(file_size);
        moves.set_len(file_size);
        let buffer = std::slice::from_raw_parts_mut(moves.as_mut_ptr() as *mut u8, file_size);
        file.read_exact(buffer)?;

        let moves_rc = Rc::new(moves);
        cache.insert(filename.to_string(), Rc::clone(&moves_rc));

        Ok(moves_rc)
    }
}

pub fn write_moves(filename: &str, moves: &[Option<Move>]) -> io::Result<()> {
    let buffer: Vec<u8> = moves.iter().map(|move_| move_.unwrap().as_int()).collect();
    let parent_dir = Path::new(filename).parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::Other, "Failed to determine parent directory")
    })?;
    fs::create_dir_all(parent_dir)?;
    let mut file = File::create(filename)?;
    file.write_all(&buffer)?;
    Ok(())
}

// TODO: use at each std::process::exit (useless)
// fn clear_cache() {
//     unsafe {
//         if let Some(cache) = MOVE_CACHE.as_mut() {
//             cache.clear();
//             MOVE_CACHE = None;
//         }
//     }
// }
