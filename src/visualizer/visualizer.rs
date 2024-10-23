use std::time::SystemTime;

use kiss3d::camera::ArcBall;
use kiss3d::event::{Action, WindowEvent};
use kiss3d::light::Light;
use kiss3d::nalgebra::{coordinates, Point2, Point3, Translation3, UnitQuaternion, Vector3};
use kiss3d::ncollide3d::procedural::TriMesh;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

use crate::color::Color;
use crate::r#move::Move;
use crate::visualizer::karaoke::{draw_karaoke, karaoke_format};
use crate::visualizer::{MOVE_INTERVAL_MS, WINDOW_SIZE, ZOOM};
use crate::{Cube, Puzzle, Pyraminx};

pub trait Drawable {
    fn draw(&self, window: &mut Window) -> Vec<SceneNode>;
}

impl<const N: usize> Drawable for Cube<N> {
    fn draw(&self, window: &mut Window) -> Vec<SceneNode> {
        const CUBIE_SIZE: f32 = 1.0;
        const MARGIN: f32 = 0.05;

        const STICKER_SIZE: f32 = CUBIE_SIZE * (1.0 - MARGIN);

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
                Color::WHITE => -Vector3::y(),
                Color::RED => -Vector3::x(),
                Color::GREEN => -Vector3::z(),
                Color::YELLOW => Vector3::y(),
                Color::ORANGE => Vector3::x(),
                Color::BLUE => Vector3::z(),
            } / 2.0;

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
            (N - 1) as f32 / 2.0 + 1.0,
            (N - 1) as f32 / 2.0,
            (N - 1) as f32 / 2.0 + 1.0,
        ));
        core.set_color(0.0, 0.0, 0.0);

        (0..6)
            .map(|i| Color::try_from(i).unwrap())
            .flat_map(|face| draw_face(self, window, face))
            .collect::<Vec<SceneNode>>()
    }
}

impl<const N: usize> Drawable for Pyraminx<N> {
    fn draw(&self, window: &mut Window) -> Vec<SceneNode> {
        // TODO WIP

        // Core
        let size = N as f32;
        let scale = Vector3::new(1.0, 1.0, 1.0);
        let translation = Translation3::new(1.7, 0.7, 1.0);

        let v1 = Point3::new(0.0, 0.0, size * 3.0f32.sqrt() / 2.0); // Top vertex
        let v2 = Point3::new(-size / 2.0, 0.0, 0.0); // Bottom-left vertex
        let v3 = Point3::new(size / 2.0, 0.0, 0.0); // Bottom-right vertex
        let normals = vec![Vector3::z(), Vector3::z(), Vector3::z()];
        let indices = vec![Point2::new(0.0, 1.0)];

        let trimesh = TriMesh::new(
            vec![v1, v2, v3],
            Some(normals.clone()),
            Some(indices.clone()),
            None,
        );
        let mut down = window.add_trimesh(trimesh, scale);
        down.set_local_translation(translation);
        down.set_color(1.0, 1.0, 0.25);

        let v1 = Point3::new(0.0, size * 3.0f32.sqrt() / 2.0, size * 3.0f32.sqrt() / 6.0);
        let v2 = Point3::new(size / 2.0, 0.0, 0.0);
        let v3 = Point3::new(-size / 2.0, 0.0, 0.0);

        let trimesh = TriMesh::new(
            vec![v1, v2, v3],
            Some(normals.clone()),
            Some(indices.clone()),
            None,
        );
        let mut front = window.add_trimesh(trimesh, scale);
        front.set_local_translation(translation);
        front.set_color(0.25, 1.0, 0.25);

        let v1 = Point3::new(0.0, size * 3.0f32.sqrt() / 2.0, size * 3.0f32.sqrt() / 6.0);
        let v2 = Point3::new(0.0, 0.0, size * 3.0f32.sqrt() / 2.0);
        let v3 = Point3::new(size / 2.0, 0.0, 0.0);

        let trimesh = TriMesh::new(
            vec![v1, v2, v3],
            Some(normals.clone()),
            Some(indices.clone()),
            None,
        );
        let mut left = window.add_trimesh(trimesh, scale);
        left.set_local_translation(translation);
        left.set_color(1.0, 0.25, 0.25);

        let v1 = Point3::new(0.0, size * 3.0f32.sqrt() / 2.0, size * 3.0f32.sqrt() / 6.0);
        let v2 = Point3::new(-size / 2.0, 0.0, 0.0);
        let v3 = Point3::new(0.0, 0.0, size * 3.0f32.sqrt() / 2.0);

        let trimesh = TriMesh::new(
            vec![v1, v2, v3],
            Some(normals.clone()),
            Some(indices.clone()),
            None,
        );
        let mut right = window.add_trimesh(trimesh, scale);
        right.set_local_translation(translation);
        right.set_color(0.25, 0.25, 1.0);

        let mut core: [SceneNode; 4] = [down, front, left, right];
        vec![]
    }
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

pub fn visualize(puzzle: &mut Box<dyn Puzzle>, moves: &Vec<Move>, karaoke: bool) {
    let mut window = Window::new_with_size("rubik", WINDOW_SIZE, WINDOW_SIZE);

    window.set_light(Light::StickToCamera);

    let mut cam = ArcBall::new(Point3::new(-2.5, 6.0, -6.0), Point3::new(1.75, 1.5, 1.5));

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
