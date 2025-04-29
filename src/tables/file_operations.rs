use {
    crate::r#move::Move,
    std::{
        collections::HashMap,
        fs::{self, File},
        io::{self, Read as _, Write as _},
        path::Path,
        sync::{Arc, LazyLock, Mutex},
    },
};

static MOVE_CACHE: LazyLock<Mutex<HashMap<String, Arc<Vec<Move>>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn read_moves(filename: &str) -> io::Result<Arc<Vec<Move>>> {
    let mut cache = MOVE_CACHE.lock().unwrap();

    if let Some(moves) = cache.get(filename) {
        return Ok(Arc::clone(moves));
    }

    let mut file = File::open(filename)?;
    let file_size = file.metadata()?.len() as usize;
    let mut moves = Vec::with_capacity(file_size);
    unsafe {
        moves.set_len(file_size);
    }
    let buffer =
        unsafe { std::slice::from_raw_parts_mut(moves.as_mut_ptr() as *mut u8, file_size) };
    file.read_exact(buffer)?;

    let moves_arc = Arc::new(moves);
    cache.insert(filename.to_string(), Arc::clone(&moves_arc));

    Ok(moves_arc)
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
