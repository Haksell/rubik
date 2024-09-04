mod karaoke;
mod visualizer;

pub use visualizer::{visualize, Drawable};

const ZOOM: f32 = 3.2;
const WINDOW_SIZE: u32 = 800;
const MOVE_INTERVAL_MS: usize = 200;
