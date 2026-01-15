mod cube;
mod pyraminx;

#[expect(clippy::pub_use)]
pub use {cube::Cube, pyraminx::Pyraminx};

use {
    crate::{
        Args, Mode,
        color::Color,
        r#move::{Move, MoveTrait},
        visualizer::{
            WINDOW_SIZE,
            karaoke::{draw_karaoke, karaoke_format},
        },
    },
    kiss3d::{
        camera::OrbitCamera3d,
        event::{Action, Key, WindowEvent},
        light::Light,
        scene::SceneNode3d,
        window::Window,
    },
    rand::seq::IndexedRandom as _,
    std::fmt::Display,
};

// TODO: each Puzzle should have its own Move and Sticker enums
// and associated constants of all moves and all stickers
pub trait Puzzle: Display {
    type Move: MoveTrait;

    fn solve(&self) -> Option<Vec<Move>>;

    fn is_solved(&self) -> bool;

    fn get_faces(&self) -> &[Color];

    fn do_move(&mut self, move_: Move);

    fn rand_scramble_moves(&self) -> &'static [Move];

    fn scramble(&mut self, sequence: &str) {
        for s in sequence.split_whitespace() {
            let move_ = self.parse_move(s).unwrap();
            self.do_move(move_);
        }
    }

    fn rand_scramble_iterations(&self) -> usize;

    fn rand_scramble(&mut self) -> Vec<Move> {
        let mut sequence: Vec<Move> = Vec::new();
        let iterations = self.rand_scramble_iterations();
        let mut rng = rand::rng();

        while sequence.len() < iterations {
            let move_ = *self.rand_scramble_moves().choose(&mut rng).unwrap();
            if sequence
                .last()
                .is_some_and(|last_move| last_move.same_face(&move_))
            {
                continue;
            }
            self.do_move(move_);
            sequence.push(move_);
        }

        sequence
    }

    fn draw(&self, scene: &mut SceneNode3d) -> Vec<SceneNode3d>;

    fn refresh_stickers(&self, stickers: &mut [SceneNode3d]) {
        stickers
            .iter_mut()
            .zip(self.get_faces().iter())
            .for_each(|(node, &color)| {
                node.set_color(color.as_rgba().into());
            });
    }

    fn default_cam(&self) -> OrbitCamera3d;

    fn opposite_move(&self, move_: Move) -> Move;

    fn parse_move(&self, value: &str) -> Result<Move, String>;

    // TODO: flag for playground mode
    #[expect(async_fn_in_trait)] // horrible
    async fn visualize(&mut self, moves: &[Move], karaoke: bool) {
        let mut window = Window::new_with_size("rubik", WINDOW_SIZE, WINDOW_SIZE).await;
        let mut scene = SceneNode3d::empty();

        scene.set_light(Some(Light::default())); // TODO Better light

        let mut cam = self.default_cam();

        // lock zoom
        cam.set_dist_step(1.0);
        cam.set_dist(9.6); // TODO: depends on puzzle

        cam.rebind_drag_button(None);

        let mut stickers = self.draw(&mut scene);

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
                                let inverse_move = self.opposite_move(moves[i]);
                                self.do_move(inverse_move);
                                self.refresh_stickers(&mut stickers);
                            }
                        }
                        Key::Right => {
                            if i < moves.len() {
                                self.do_move(moves[i]);
                                i += 1;
                                self.refresh_stickers(&mut stickers);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    #[expect(async_fn_in_trait)] // horrible
    async fn main(&mut self, args: Args) {
        if let Some(sequence) = args.scramble {
            self.scramble(&sequence);
        } else {
            let sequence = self.rand_scramble();
            println!(
                "No scramble sequence provided, using the following one:\n{}",
                Move::format_sequence(&sequence)
            );
        }

        println!("{self}");

        let solution = self
            .solve()
            .expect("a valid solution should always be found");

        if solution.is_empty() {
            println!("The puzzle was already solved!");
        } else {
            println!("Solution of {} moves found:", solution.len());
            println!("{}", Move::format_sequence(&solution));
        }

        if args.mode != Mode::Cli {
            self.visualize(
                &solution,
                args.mode == Mode::Karaoke,
                // TODO: no playground bool
            )
            .await;
        }
    }
}
