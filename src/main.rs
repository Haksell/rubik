// TODO: setup camera correctly at the start
// TODO: deactivate zoom/dezoom
use kiss3d::light::Light;
use kiss3d::nalgebra::{Translation3, UnitQuaternion, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use rubik::color::Color;
use rubik::cube::Cube;
use rubik::r#move::Move;
use rubik::{color, cub3};

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

fn draw_face<const N: usize>(cube: &Cube<N>, window: &mut Window, face: Color) {
    let cubie = Cubie {
        up: [1.0, 1.0, 1.0],        // White
        down: [1.0, 0.835, 0.0],    // Yellow
        front: [0.0, 0.608, 0.282], // Green
        back: [0.0, 0.275, 0.678],  // Blue
        left: [1.0, 0.345, 0.0],    // Orange
        right: [1.0, 0.071, 0.204], // Red
    };

    let display_color = |c: Color| match c {
        Color::WHITE => cubie.up,
        Color::RED => cubie.right,
        Color::GREEN => cubie.front,
        Color::YELLOW => cubie.down,
        Color::ORANGE => cubie.left,
        Color::BLUE => cubie.back,
    };

    let translation_addition = match face {
        Color::WHITE => -Vector3::y() * 0.5,
        Color::RED => todo!(),
        Color::GREEN => todo!(),
        Color::YELLOW => Vector3::y() * 0.5,
        Color::ORANGE => todo!(),
        Color::BLUE => todo!(),
    };

    let rotation = match face {
        Color::WHITE => {
            UnitQuaternion::from_axis_angle(&Vector3::x_axis(), std::f32::consts::FRAC_PI_2)
        }
        Color::RED => todo!(),
        Color::GREEN => todo!(),
        Color::YELLOW => {
            UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -std::f32::consts::FRAC_PI_2)
        }
        Color::ORANGE => todo!(),
        Color::BLUE => todo!(),
    };

    let face_y = match face {
        Color::WHITE => 3,
        Color::RED => todo!(),
        Color::GREEN => todo!(),
        Color::YELLOW => -1,
        Color::ORANGE => todo!(),
        Color::BLUE => todo!(),
    };

    let const_y = face == Color::WHITE || face == Color::YELLOW;
    let const_x = face == Color::WHITE || face == Color::YELLOW;
    let const_z = face == Color::WHITE || face == Color::YELLOW;

    let start = face as usize * N * N;
    for i in 0..N * N {
        let i = start + i;
        let y = if const_y {
            face_y
        } else {
            N - (i / (N * N)) % N
        };
        let z = 3 - (i / 3) % 3;
        let x = 3 - i % 3;

        let col = cube.faces[i];

        println!("x={x} y={y} z={z} col={}", col as u8);

        let translation = Translation3::new(
            (x as f32 - 2.0) * CUBIE_SIZE,
            (y as f32 - 1.0) * CUBIE_SIZE,
            (z as f32 - 2.0) * CUBIE_SIZE,
        );

        create_cubie_face(
            window,
            display_color(col),
            translation.vector + translation_addition,
            rotation,
        );
    }

    // if y == 2 {
    //     // TOP
    //     cubes.push(create_cubie_face(
    //         &mut window,
    //         to_col(col),
    //         translation.vector + Vector3::y() * 0.5,
    //         UnitQuaternion::from_axis_angle(&Vector3::x_axis(), std::f32::consts::FRAC_PI_2),
    //     ));
    // }
    // if y == 0 {
    //     // BOTTOM
    //     cubes.push(create_cubie_face(
    //         &mut window,
    //         to_col(col),
    //         translation.vector - Vector3::y() * 0.5,
    //         UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -std::f32::consts::FRAC_PI_2),
    //     ));
    // }
    // if z == 0 {
    //     // FRONT
    //     cubes.push(create_cubie_face(
    //         &mut window,
    //         to_col(col),
    //         translation.vector - Vector3::z() * 0.5,
    //         UnitQuaternion::identity(),
    //     ));
    // }
    // if z == 2 {
    //     // BACK
    //     cubes.push(create_cubie_face(
    //         &mut window,
    //         to_col(col),
    //         translation.vector + Vector3::z() * 0.5,
    //         UnitQuaternion::from_axis_angle(&Vector3::y_axis(), std::f32::consts::PI),
    //     ));
    // }
    // if x == 2 {
    //     // LEFT
    //     cubes.push(create_cubie_face(
    //         &mut window,
    //         to_col(col),
    //         translation.vector + Vector3::x() * 0.5,
    //         UnitQuaternion::from_axis_angle(&Vector3::y_axis(), std::f32::consts::FRAC_PI_2),
    //     ));
    // }
    // if x == 0 {
    //     // RIGHT
    //     cubes.push(create_cubie_face(
    //         &mut window,
    //         to_col(col),
    //         translation.vector - Vector3::x() * 0.5,
    //         UnitQuaternion::from_axis_angle(&Vector3::y_axis(), -std::f32::consts::FRAC_PI_2),
    //     ));
    // }
}

fn main() {
    let mut window = Window::new("Rubik's Cube");

    window.set_light(Light::StickToCamera);

    // let mut cam = FixedView::new();

    // window.render_with_camera(&mut cam);

    let mut core = window.add_cube(CORE_SIZE, CORE_SIZE, CORE_SIZE);
    core.set_color(198.0 / 255.0, 3.0 / 255.0, 252.0 / 255.0);

    let mut cube: Cube<3> = cub3!();

    cube.do_move(Move::R);

    println!("{cube}");
    println!("{:?}", cube.faces);

    draw_face(&cube, &mut window, Color::WHITE);
    draw_face(&cube, &mut window, Color::YELLOW);

    while window.render() {
        // println!("{}", cam.view_transform());
    }
}
