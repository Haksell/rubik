use kiss3d::light::Light;
use kiss3d::nalgebra::{Translation3, UnitQuaternion, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

const CUBE_SIZE: f32 = 1.0;

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
    let mut face = window.add_quad(CUBE_SIZE, CUBE_SIZE, 1, 1); // TODO: fix aliasing
    face.set_local_translation(Translation3::from(translation));
    face.set_local_rotation(rotation);
    face.set_color(color[0], color[1], color[2]);
    face
}

fn main() {
    let mut window = Window::new("Rubik's Cube");
    window.set_light(Light::StickToCamera);

    let mut cubes = Vec::new();

    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                let translation = Translation3::new(
                    (x as f32 - 1.5) * CUBE_SIZE,
                    (y as f32 - 1.5) * CUBE_SIZE,
                    (z as f32 - 1.5) * CUBE_SIZE,
                );

                let cubie = Cubie {
                    up: [1.0, 1.0, 1.0],          // White
                    down: [1.000, 0.835, 0.000],  // Yellow
                    front: [0.000, 0.608, 0.282], // Green
                    back: [0.000, 0.275, 0.678],  // Blue
                    left: [1.000, 0.345, 0.000],  // Orange
                    right: [0.718, 0.071, 0.204], // Red
                };

                cubes.push(create_cubie_face(
                    &mut window,
                    cubie.front,
                    translation.vector + Vector3::z() * 0.5,
                    UnitQuaternion::identity(),
                ));
                cubes.push(create_cubie_face(
                    &mut window,
                    cubie.back,
                    translation.vector - Vector3::z() * 0.5,
                    UnitQuaternion::from_axis_angle(&Vector3::y_axis(), std::f32::consts::PI),
                ));
                cubes.push(create_cubie_face(
                    &mut window,
                    cubie.up,
                    translation.vector + Vector3::y() * 0.5,
                    UnitQuaternion::from_axis_angle(
                        &Vector3::x_axis(),
                        std::f32::consts::FRAC_PI_2,
                    ),
                ));
                cubes.push(create_cubie_face(
                    &mut window,
                    cubie.down,
                    translation.vector - Vector3::y() * 0.5,
                    UnitQuaternion::from_axis_angle(
                        &Vector3::x_axis(),
                        -std::f32::consts::FRAC_PI_2,
                    ),
                ));
                cubes.push(create_cubie_face(
                    &mut window,
                    cubie.left,
                    translation.vector - Vector3::x() * 0.5,
                    UnitQuaternion::from_axis_angle(
                        &Vector3::y_axis(),
                        std::f32::consts::FRAC_PI_2,
                    ),
                ));
                cubes.push(create_cubie_face(
                    &mut window,
                    cubie.right,
                    translation.vector + Vector3::x() * 0.5,
                    UnitQuaternion::from_axis_angle(
                        &Vector3::y_axis(),
                        -std::f32::consts::FRAC_PI_2,
                    ),
                ));
            }
        }
    }

    while window.render() {}
}
