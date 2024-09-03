use std::time::SystemTime;

use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point2, Point3, Translation3, UnitQuaternion, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::text::Font;
use kiss3d::window::Window;

use crate::color::Color;
use crate::r#move::Move;
use crate::{Cube, Puzzle, Pyraminx};
const CUBIE_SIZE: f32 = 1.0;
const MARGIN: f32 = 0.05;
const STICKER_SIZE: f32 = CUBIE_SIZE * (1.0 - MARGIN);
const ZOOM: f32 = 3.2;
const WINDOW_SIZE: u32 = 800;
const MOVE_INTERVAL_MS: usize = 200;
const TEXT_SCALE: f32 = 100.0;

pub trait Drawable {
    fn draw(&self, window: &mut Window) -> Vec<SceneNode>;
}

impl<const N: usize> Drawable for Cube<N> {
    fn draw(&self, window: &mut Window) -> Vec<SceneNode> {
        fn create_cubie_face(
            window: &mut Window,
            color: &[f32; 3],
            translation: Vector3<f32>,
            rotation: UnitQuaternion<f32>,
        ) -> SceneNode {
            let mut face = window.add_quad(STICKER_SIZE, STICKER_SIZE, 1, 1);
            face.set_local_translation(Translation3::from(translation));
            face.set_local_rotation(rotation);
            face.set_color(color[0], color[1], color[2]);
            face
        }

        fn draw_face<const N: usize>(
            cube: &Cube<N>,
            window: &mut Window,
            face: Color,
        ) -> Vec<SceneNode> {
            let mut squares: Vec<SceneNode> = Vec::new();

            let translation_addition = match face {
                Color::WHITE => -Vector3::y() * 0.5,
                Color::RED => -Vector3::x() * 0.5,
                Color::GREEN => -Vector3::z() * 0.5,
                Color::YELLOW => Vector3::y() * 0.5,
                Color::ORANGE => Vector3::x() * 0.5,
                Color::BLUE => Vector3::z() * 0.5,
            };

            let rotation = match face {
                Color::WHITE => {
                    UnitQuaternion::from_axis_angle(&Vector3::x_axis(), std::f32::consts::FRAC_PI_2)
                }
                Color::RED => {
                    UnitQuaternion::from_axis_angle(&Vector3::y_axis(), std::f32::consts::FRAC_PI_2)
                }
                Color::GREEN => UnitQuaternion::identity(),
                Color::YELLOW => UnitQuaternion::from_axis_angle(
                    &Vector3::x_axis(),
                    -std::f32::consts::FRAC_PI_2,
                ),
                Color::ORANGE => UnitQuaternion::from_axis_angle(
                    &Vector3::y_axis(),
                    -std::f32::consts::FRAC_PI_2,
                ),
                Color::BLUE => UnitQuaternion::identity(),
            };

            let start = face as usize * N * N;
            for i in 0..N * N {
                let i = start + i;
                let color = cube.faces[i];
                let (x, y, z) = get_coords(i, N, face);
                let translation = Translation3::new(
                    (x as f32) * CUBIE_SIZE,
                    (y as f32) * CUBIE_SIZE,
                    (z as f32) * CUBIE_SIZE,
                );

                squares.push(create_cubie_face(
                    window,
                    &color.as_rgb(),
                    translation.vector + translation_addition,
                    rotation,
                ));
            }
            squares
        }

        // TODO Cleanup
        fn get_coords(i: usize, n: usize, face: Color) -> (f32, f32, f32) {
            let n = n as f32;
            let i = i as f32;
            if face == Color::YELLOW || face == Color::WHITE {
                let x = n - i % n;
                let y = if face == Color::WHITE { n } else { -1.0 };
                let z = if face == Color::WHITE {
                    n - (i.div_euclid(n)) % n
                } else {
                    (i.div_euclid(n)) % n + 1.0
                };
                (x, y, z)
            } else if face == Color::GREEN || face == Color::BLUE {
                let x = if face == Color::GREEN {
                    n - i % n
                } else {
                    i % n + 1.0
                };
                let y = n - (i.div_euclid(n)) % n - 1.0;
                let z = if face == Color::GREEN { 1.0 } else { n };
                (x, y, z)
            } else {
                let x = if face == Color::ORANGE { n } else { 1.0 };
                let y = n - (i.div_euclid(n)) % n - 1.0;
                let z = if face == Color::ORANGE {
                    n - i % n
                } else {
                    i % n + 1.0
                };
                (x, y, z)
            }
        }

        let core_size: f32 = CUBIE_SIZE * (N as f32 - 2.0 * MARGIN);

        let mut core = window.add_cube(core_size, core_size, core_size);
        core.set_local_translation(Translation3::new(
            (N - 1) as f32 * 0.5 + 1.0,
            (N - 1) as f32 * 0.5,
            (N - 1) as f32 * 0.5 + 1.0,
        ));
        core.set_color(0.0, 0.0, 0.0);

        (0..6)
            .map(|i| Color::try_from(i).unwrap())
            .flat_map(|face| draw_face(self, window, face))
            .collect::<Vec<SceneNode>>()
    }
}

impl<const N: usize> Drawable for Pyraminx<N> {
    fn draw(&self, _window: &mut Window) -> Vec<SceneNode> {
        todo!()
    }
}

fn display_size(text: &str) -> f32 {
    text.chars()
        .map(|c| {
            Font::default()
                .font()
                .glyph(c)
                .scaled(rusttype::Scale::uniform(TEXT_SCALE))
                .h_metrics()
                .advance_width
        })
        .sum()
}

fn draw_karaoke(text: &str, start: &SystemTime, total: usize, window: &mut Window) {
    let font = Font::default();
    let elapsed = start.elapsed().unwrap().as_millis() as f64;
    let end = total as f64 * MOVE_INTERVAL_MS as f64;

    let mut idx = ((elapsed * text.chars().count() as f64) / end).floor() as usize;
    if idx > text.chars().count() {
        idx = text.chars().count();
    }
    let cur_line = text[..idx].chars().filter(|&c| c == '\n').count();
    let vmetrics = font.font().v_metrics(rusttype::Scale::uniform(TEXT_SCALE));
    let line_height = vmetrics.ascent - vmetrics.descent;
    let mut char_sum = 0;

    text.lines().enumerate().for_each(|(i, line)| {
        let starty = i as f32 * line_height;
        let centerx = ((WINDOW_SIZE * 2) as f32 - display_size(line)) / 2.0;
        if i == cur_line {
            window.draw_text(
                &line[..idx - char_sum],
                &Point2::new(centerx, starty),
                TEXT_SCALE,
                &font,
                &Point3::new(0.0, 1.0, 0.0),
            );

            let startx = display_size(&line[..idx - char_sum]);

            window.draw_text(
                &line[idx - char_sum..],
                &Point2::new(centerx + startx, starty),
                TEXT_SCALE,
                &font,
                &Point3::new(1.0, 0.0, 0.0),
            );
        } else {
            let color = if i < cur_line {
                Point3::new(0.0, 1.0, 0.0)
            } else {
                Point3::new(1.0, 0.0, 0.0)
            };
            window.draw_text(
                &line,
                &Point2::new(centerx, starty),
                TEXT_SCALE,
                &font,
                &color,
            );
            char_sum += line.chars().count() + 1;
        }
    });
}

pub fn visualize(mut puzzle: Box<dyn Puzzle>, moves: &Vec<Move>, karaoke: bool) {
    let mut window = Window::new_with_size("rubik", WINDOW_SIZE, WINDOW_SIZE);

    window.set_light(Light::StickToCamera);

    let mut cam = ArcBall::new(Point3::new(-2.5, 6.0, -6.0), Point3::new(1.5, 1.5, 1.5));

    // Lock zoom
    cam.set_dist_step(1.0);
    const N: usize = 3;
    cam.set_dist(ZOOM * N as f32); // TODO Real N maybe need for puzzle<N> ?

    let start = SystemTime::now();

    let mut i: usize = 0;

    let mut stickers = puzzle.draw(&mut window);

    let mut text = String::new();

    if karaoke {
        let mut chars_width = 0.0;
        moves.iter().enumerate().for_each(|(i, &move_)| {
            let mut move_str = format!("{:?}", move_);
            if i > 0 {
                move_str.insert(0, ' ');
            }
            let mut move_display_size = display_size(&move_str);
            if chars_width + move_display_size > (WINDOW_SIZE * 2 - 5) as f32 {
                text.push('\n');
                if move_str.starts_with(" ") {
                    move_str.remove(0);
                    move_display_size = display_size(&move_str);
                }
                chars_width = 0.0;
            }
            chars_width += move_display_size;
            text.push_str(&move_str);
        });
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

                // Refresh colors
                stickers
                    .iter_mut()
                    .zip(puzzle.get_faces().iter())
                    .for_each(|(node, &color)| {
                        let [r, g, b] = color.as_rgb();
                        node.set_color(r, g, b)
                    });
            }
        }
    }
}
