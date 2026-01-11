// TODO: clean this mod

pub mod karaoke;

use {
    crate::{r#move::Move, puzzles::Puzzle},
    karaoke::{draw_karaoke, karaoke_format},
    kiss3d::{
        event::{Action, Key, WindowEvent},
        light::Light,
        scene::SceneNode3d,
        window::Window,
    },
};

pub const WINDOW_SIZE: u32 = 1000;
