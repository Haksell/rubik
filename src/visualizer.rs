use std::time::SystemTime;

use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Translation3, UnitQuaternion, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

use crate::color::Color;
use crate::r#move::Move;
use crate::{Cube, Puzzle};
const CUBIE_SIZE: f32 = 1.0;
const MARGIN: f32 = 0.05;
const STICKER_SIZE: f32 = CUBIE_SIZE * (1.0 - MARGIN);
const ZOOM: f32 = 3.2;
const N: usize = 3;
const CORE_SIZE: f32 = CUBIE_SIZE * (N as f32 - 2.0 * MARGIN);
const WINDOW_SIZE: u32 = 800;
const MOVE_INTERVAL_MS: usize = 200;

fn create_cubie_face(
    window: &mut Window,
    color: [f32; 3],
    translation: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
) -> SceneNode {
    let mut face = window.add_quad(STICKER_SIZE, STICKER_SIZE, 1, 1);
    face.set_local_translation(Translation3::from(translation));
    face.set_local_rotation(rotation);
    face.set_color(color[0], color[1], color[2]);
    face
}

fn display_color(color: Color) -> [f32; 3] {
    match color {
        Color::WHITE => [1.0, 1.0, 1.0],
        Color::RED => [1.0, 0.071, 0.204],
        Color::GREEN => [0.0, 0.608, 0.282],
        Color::YELLOW => [1.0, 0.835, 0.0],
        Color::ORANGE => [1.0, 0.345, 0.0],
        Color::BLUE => [0.0, 0.275, 0.678],
    }
}

fn draw_face<const N: usize>(cube: &Cube<N>, window: &mut Window, face: Color) -> Vec<SceneNode> {
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
        Color::YELLOW => {
            UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -std::f32::consts::FRAC_PI_2)
        }
        Color::ORANGE => {
            UnitQuaternion::from_axis_angle(&Vector3::y_axis(), -std::f32::consts::FRAC_PI_2)
        }
        Color::BLUE => UnitQuaternion::identity(),
    };

    let start = face as usize * N * N;
    for i in 0..N * N {
        let i = start + i;
        let col = cube.faces[i];
        let (x, y, z) = get_coords(i, N, face);
        let translation = Translation3::new(
            (x as f32 - 2.0) * CUBIE_SIZE,
            (y as f32 - 1.0) * CUBIE_SIZE,
            (z as f32 - 2.0) * CUBIE_SIZE,
        );

        squares.push(create_cubie_face(
            window,
            display_color(col),
            translation.vector + translation_addition,
            rotation,
        ));
    }
    squares
}

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

fn draw_cube<const N: usize>(cube: &Cube<N>, window: &mut Window) -> Vec<SceneNode> {
    (0..6)
        .map(|i| Color::try_from(i).unwrap())
        .flat_map(|face| draw_face(cube, window, face))
        .collect::<Vec<SceneNode>>()
}

// TODO Generic for Pyraminx (maybe Visualizable trait)
pub fn visualize<const N: usize>(mut cube: Cube<N>, moves: &Vec<Move>) {
    let mut window = Window::new_with_size("rubik", WINDOW_SIZE, WINDOW_SIZE);

    window.set_light(Light::StickToCamera);

    let mut cam = ArcBall::new(Point3::new(-4.0, 6.0, -10.0), Point3::origin());

    // Lock zoom
    cam.set_dist_step(1.0);
    cam.set_dist(ZOOM * N as f32);

    let mut core = window.add_cube(CORE_SIZE, CORE_SIZE, CORE_SIZE);
    // core.set_local_translation(Translation3::new(0.0, 0.0, 0.0)); // TODO Move it to the right place (so that it works for any cube)
    core.set_color(0.0, 0.0, 0.0);

    let start = SystemTime::now();

    let mut i: usize = 0;

    let mut visualized = draw_cube(&cube, &mut window);

    while window.render_with_camera(&mut cam) {
        if i < moves.len() {
            let elapsed = start.elapsed().unwrap().as_millis();
            let idx = elapsed as usize / MOVE_INTERVAL_MS;

            if idx > i {
                cube.do_move(moves[i]);
                i = idx;

                // Refresh colors
                visualized
                    .iter_mut()
                    .zip(cube.faces.iter())
                    .for_each(|(node, &color)| {
                        let [r, g, b] = display_color(color);
                        node.set_color(r, g, b)
                    });
            }
        }
    }
}
