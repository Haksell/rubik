mod karaoke;

use crate::r#move::Move;
use crate::visualizer::karaoke::{draw_karaoke, karaoke_format};
use crate::Puzzle;
use kiss3d::camera::ArcBall;
use kiss3d::event::{Action, WindowEvent};
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use std::time::SystemTime;

const ZOOM: f32 = 3.2;
const WINDOW_SIZE: u32 = 800;
const MOVE_INTERVAL_MS: usize = 200;

pub trait Drawable {
    fn draw(&self, window: &mut Window) -> Vec<SceneNode>;
    fn default_cam(&self) -> ArcBall;
}

fn refresh_stickers(stickers: &mut Vec<SceneNode>, puzzle: &Box<dyn Puzzle>) {
    stickers
        .iter_mut()
        .zip(puzzle.get_faces().iter())
        .for_each(|(node, &color)| {
            let [r, g, b] = color.as_rgb();
            node.set_color(r, g, b)
        });
}

// TODO: disable translation with right-click
// TODO: flag for playground mode
pub fn visualize(puzzle: &mut Box<dyn Puzzle>, moves: &Vec<Move>, karaoke: bool) {
    let mut window = Window::new_with_size("rubik", WINDOW_SIZE, WINDOW_SIZE);

    window.set_light(Light::StickToCamera);

    let mut cam = puzzle.default_cam();

    // Lock zoom
    cam.set_dist_step(1.0);
    const N: usize = 3;
    cam.set_dist(ZOOM * N as f32); // TODO Real N maybe need for puzzle<N> ?

    let start = SystemTime::now();

    let mut i: usize = 0;

    let mut stickers = puzzle.draw(&mut window);

    let mut text = String::new();

    if karaoke {
        text = karaoke_format(moves);
    }

    while window.render_with_camera(&mut cam) {
        if karaoke {
            draw_karaoke(&text, &start, moves.len(), &mut window);
        }

        if i < moves.len() {
            let elapsed = start.elapsed().unwrap().as_millis();
            let idx = elapsed as usize / MOVE_INTERVAL_MS;

            if idx > i {
                puzzle.do_move(moves[i]);
                i = idx;

                refresh_stickers(&mut stickers, &puzzle);
            }
        } else {
            for mut event in window.events().iter() {
                if let WindowEvent::Key(button, Action::Release, mods) = event.value {
                    if let Ok(move_) = Move::try_from((button, mods)) {
                        puzzle.do_move(move_);
                        refresh_stickers(&mut stickers, &puzzle);
                        event.inhibited = true
                    }
                }
            }
        }
    }
}
