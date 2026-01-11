use {
    crate::r#move::Move,
    std::{
        collections::HashMap,
        fs::{self, File},
        io::{self, Write as _},
        path::Path,
        sync::{Arc, LazyLock, Mutex},
    },
};

static MOVE_CACHE: LazyLock<Mutex<HashMap<String, Arc<[Move]>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[expect(clippy::significant_drop_tightening, clippy::unwrap_in_result)]
pub fn read_moves(filename: &str) -> io::Result<Arc<[Move]>> {
    let mut cache = MOVE_CACHE.lock().unwrap();

    if let Some(moves) = cache.get(filename) {
        return Ok(Arc::clone(moves));
    }

    let mut bytes = std::mem::ManuallyDrop::new(std::fs::read(filename)?);
    let ptr = bytes.as_mut_ptr().cast();
    let len = bytes.len();
    let cap = bytes.capacity();

    let moves: Arc<[Move]> =
        Arc::from(unsafe { Vec::from_raw_parts(ptr, len, cap) }.into_boxed_slice());

    cache.insert(filename.to_owned(), Arc::clone(&moves));

    Ok(moves)
}

pub fn write_moves(filename: &str, moves: &[Option<Move>]) -> io::Result<()> {
    let buffer: Vec<u8> = moves.iter().map(|move_| move_.unwrap().as_int()).collect();
    let parent_dir = Path::new(filename)
        .parent()
        .ok_or_else(|| io::Error::other("Failed to determine parent directory"))?;
    fs::create_dir_all(parent_dir)?;
    let mut file = File::create(filename)?;
    file.write_all(&buffer)
}
