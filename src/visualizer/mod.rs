mod karaoke;

use {
    crate::{r#move::Move, visualizer::karaoke::draw_karaoke, Puzzle},
    karaoke::karaoke_format,
    kiss3d::{
        event::{Action, Key, WindowEvent},
        light::Light,
        scene::SceneNode3d,
        window::Window,
    },
};

const WINDOW_SIZE: u32 = 1000;

fn refresh_stickers(stickers: &mut Vec<SceneNode3d>, puzzle: &mut Box<dyn Puzzle>) {
    stickers
        .iter_mut()
        .zip(puzzle.get_faces().iter())
        .for_each(|(node, &color)| {
            let [r, g, b] = color.as_rgb();
            node.set_color(kiss3d::color::Color::new(r, g, b, 1.0));
        });
}

// TODO: flag for playground mode
pub async fn visualize(puzzle: &mut Box<dyn Puzzle>, moves: &Vec<Move>, karaoke: bool) {
    let mut window = Window::new_with_size("rubik", WINDOW_SIZE, WINDOW_SIZE).await;
    let mut scene = SceneNode3d::empty();

    scene.set_light(Some(Light::default())); // TODO Better light

    let mut cam = puzzle.default_cam();

    // lock zoom
    cam.set_dist_step(1.0);
    cam.set_dist(9.6); // TODO: depends on puzzle

    cam.rebind_drag_button(None);

    let mut i: usize = 0;

    let mut stickers = puzzle.draw(&mut scene);

    let text = if karaoke {
        karaoke_format(moves)
    } else {
        String::new()
    };

    while window.render_3d(&mut scene, &mut cam).await {
        if karaoke {
            draw_karaoke(&text, i, &mut window);
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
                            refresh_stickers(&mut stickers, puzzle);
                        }
                    }
                    Key::Right => {
                        if i < moves.len() {
                            puzzle.do_move(moves[i]);
                            i += 1;
                            refresh_stickers(&mut stickers, puzzle);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
