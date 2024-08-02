// TODO: setup camera correctly at the start
// TODO: deactivate zoom/dezoom

use kiss3d::light::Light;
use kiss3d::nalgebra::{Translation3, UnitQuaternion, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

const CUBIE_SIZE: f32 = 1.0;
const MARGIN: f32 = 0.05;
const STICKER_SIZE: f32 = CUBIE_SIZE * (1.0 - MARGIN);
const CORE_SIZE: f32 = CUBIE_SIZE * (3.0 - 2.0 * MARGIN);

// TODO: Point3<f32>
struct Cubie {
    up: [f32; 3],
    down: [f32; 3],
    left: [f32; 3],
    right: [f32; 3],
    front: [f32; 3],
    back: [f32; 3],
}

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

fn main() {
    let mut window = Window::new("Rubik's Cube");
    window.set_light(Light::StickToCamera);

    let mut core = window.add_cube(CORE_SIZE, CORE_SIZE, CORE_SIZE);
    core.set_color(0.0, 0.0, 0.0);

    let mut cubes = Vec::new();

    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                let translation = Translation3::new(
                    (x as f32 - 1.0) * CUBIE_SIZE,
                    (y as f32 - 1.0) * CUBIE_SIZE,
                    (z as f32 - 1.0) * CUBIE_SIZE,
                );

                let cubie = Cubie {
                    up: [1.0, 1.0, 1.0],        // White
                    down: [1.0, 0.835, 0.0],    // Yellow
                    front: [0.0, 0.608, 0.282], // Green
                    back: [0.0, 0.275, 0.678],  // Blue
                    left: [1.0, 0.345, 0.0],    // Orange
                    right: [1.0, 0.071, 0.204], // Red
                };

                if y == 2 {
                    cubes.push(create_cubie_face(
                        &mut window,
                        cubie.up,
                        translation.vector + Vector3::y() * 0.5,
                        UnitQuaternion::from_axis_angle(
                            &Vector3::x_axis(),
                            std::f32::consts::FRAC_PI_2,
                        ),
                    ));
                }
                if y == 0 {
                    cubes.push(create_cubie_face(
                        &mut window,
                        cubie.down,
                        translation.vector - Vector3::y() * 0.5,
                        UnitQuaternion::from_axis_angle(
                            &Vector3::x_axis(),
                            -std::f32::consts::FRAC_PI_2,
                        ),
                    ));
                }
                if z == 0 {
                    cubes.push(create_cubie_face(
                        &mut window,
                        cubie.front,
                        translation.vector - Vector3::z() * 0.5,
                        UnitQuaternion::identity(),
                    ));
                }
                if z == 2 {
                    cubes.push(create_cubie_face(
                        &mut window,
                        cubie.back,
                        translation.vector + Vector3::z() * 0.5,
                        UnitQuaternion::from_axis_angle(&Vector3::y_axis(), std::f32::consts::PI),
                    ));
                }
                if x == 2 {
                    cubes.push(create_cubie_face(
                        &mut window,
                        cubie.left,
                        translation.vector + Vector3::x() * 0.5,
                        UnitQuaternion::from_axis_angle(
                            &Vector3::y_axis(),
                            std::f32::consts::FRAC_PI_2,
                        ),
                    ));
                }
                if x == 0 {
                    cubes.push(create_cubie_face(
                        &mut window,
                        cubie.right,
                        translation.vector - Vector3::x() * 0.5,
                        UnitQuaternion::from_axis_angle(
                            &Vector3::y_axis(),
                            -std::f32::consts::FRAC_PI_2,
                        ),
                    ));
                }
            }
        }
    }

    while window.render() {}
}
