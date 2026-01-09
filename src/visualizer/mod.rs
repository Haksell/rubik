mod karaoke;

use {
    crate::{Puzzle, r#move::Move},
    karaoke::{draw_karaoke, karaoke_format},
    kiss3d::{
        event::{Action, Key, WindowEvent},
        light::Light,
        scene::SceneNode,
        window::Window,
    },
};

const WINDOW_SIZE: u32 = 1000;

fn refresh_stickers(stickers: &mut [SceneNode], puzzle: &mut Box<dyn Puzzle>) {
    stickers
        .iter_mut()
        .zip(puzzle.get_faces().iter())
        .for_each(|(node, &color)| {
            let [r, g, b] = color.as_rgb();
            node.set_color(r, g, b);
        });
}

// TODO: flag for playground mode
pub fn visualize(puzzle: &mut Box<dyn Puzzle>, moves: &[Move], karaoke: bool) {
    let mut window = Window::new_with_size("rubik", WINDOW_SIZE, WINDOW_SIZE);

    window.set_light(Light::StickToCamera);

    let mut cam = puzzle.default_cam();

    // lock zoom
    cam.set_dist_step(1.0);
    cam.set_dist(9.6); // TODO: depends on puzzle

    cam.rebind_drag_button(None);

    let mut i: usize = 0;

    let mut stickers = puzzle.draw(&mut window);

    let karaoke_text = karaoke.then(|| karaoke_format(moves));

    while window.render_with_camera(&mut cam) {
        if let Some(text) = &karaoke_text {
            draw_karaoke(text, i, &mut window);
        }

        for mut event in window.events().iter() {
            if let WindowEvent::Key(button, Action::Press, _) = event.value {
                match button {
                    Key::Left => {
                        if i > 0 {
                            i -= 1;
                            let inverse_move = puzzle.opposite_move(moves[i]);
                            puzzle.do_move(inverse_move);
                            refresh_stickers(&mut stickers, puzzle);
                        }
                        event.inhibited = true;
                    }
                    Key::Right => {
                        if i < moves.len() {
                            puzzle.do_move(moves[i]);
                            i += 1;
                            refresh_stickers(&mut stickers, puzzle);
                        }
                        event.inhibited = true;
                    }
                    _ => {}
                }
            }
        }
    }
}
