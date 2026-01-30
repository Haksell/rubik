mod karaoke;

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

const WINDOW_SIZE: u32 = 1000;

// TODO: flag for playground mode
pub async fn visualize(puzzle: &mut Box<dyn Puzzle>, moves: &[Move], karaoke: bool) {
    let mut window = Window::new_with_size("rubik", WINDOW_SIZE, WINDOW_SIZE).await;
    let mut scene = SceneNode3d::empty();

    scene.set_light(Some(Light::default())); // TODO: better light

    let mut cam = puzzle.default_cam();

    // lock zoom
    cam.set_dist_step(1.0);
    cam.set_dist(9.6); // TODO: depends on puzzle

    cam.rebind_drag_button(None);

    let mut stickers = puzzle.draw(&mut scene);

    let karaoke_text = karaoke.then(|| karaoke_format(moves));

    let mut i: usize = 0;

    while window.render_3d(&mut scene, &mut cam).await {
        if let Some(text) = &karaoke_text {
            draw_karaoke(text, i, &mut window);
        }

        for mut event in window.events().iter() {
            if let WindowEvent::Key(button, Action::Press, _) = event.value {
                event.inhibited = true;
                match button {
                    Key::Left => {
                        if i > 0 {
                            i -= 1;
                            let inverse_move = puzzle.opposite_move(moves[i]);
                            puzzle.do_move(inverse_move);
                            puzzle.refresh_stickers(&mut stickers);
                        }
                    }
                    Key::Right => {
                        if i < moves.len() {
                            puzzle.do_move(moves[i]);
                            i += 1;
                            puzzle.refresh_stickers(&mut stickers);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
