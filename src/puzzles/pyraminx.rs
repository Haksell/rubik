use crate::color::Color;
use crate::r#move::Move;
use crate::solvers::{iddfs, DFSAble};
use crate::visualizer::Drawable;
use crate::Puzzle;
use kiss3d::camera::ArcBall;
use kiss3d::nalgebra::{Point2, Point3, Vector3};
use kiss3d::ncollide3d::procedural::TriMesh;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Pyraminx {
    pub faces: Vec<Color>,
}

impl Pyraminx {
    pub fn new() -> Pyraminx {
        const ORDER: [Color; 4] = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW];

        Pyraminx {
            faces: (0..ORDER.len() * 9).map(|i| ORDER[i / 9]).collect(),
        }
    }

    pub fn to_pyraminx3(&self) -> Result<Pyraminx, &'static str> {
        Ok(Pyraminx {
            faces: self.faces.clone(),
        })
    }

    fn get_face(&self, face: usize) -> &[Color] {
        let start = face * 9;
        let end = (face + 1) * 9;
        &self.faces[start..end]
    }
}

impl Puzzle for Pyraminx {
    fn do_move(&mut self, _: Move) {
        // TODO
    }

    fn solve(&self) -> Option<Vec<Move>> {
        Some(iddfs(self.clone()))
    }

    // TODO Better check ?
    fn is_solved(&self) -> bool {
        const ORDER: [Color; 4] = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW];
        self.faces
            .iter()
            .enumerate()
            .all(|(i, &col)| col == ORDER[i / 9])
    }

    fn get_faces(&self) -> &Vec<Color> {
        &self.faces
    }
}

impl DFSAble for Pyraminx {
    const ALLOWED_MOVES: &'static [Move] = &[
        Move::R,
        Move::U,
        Move::B,
        Move::L,
        Move::R2,
        Move::U2,
        Move::B2,
        Move::L2,
    ];
}

impl Drawable for Pyraminx {
    fn draw(&self, window: &mut Window) -> Vec<SceneNode> {
        const CORE_MARGIN: f32 = 0.04;
        const STICKER_MARGIN: f32 = 0.1;

        fn render_core(window: &mut Window, mut vertices: [Point3<f32>; 4]) {
            let normals = vec![Vector3::z(), Vector3::z(), Vector3::z()];
            let indices = vec![Point2::new(0.0, 1.0)];
            let scale = Vector3::new(3.0, 3.0, 3.0);

            let middle = Point3::from(
                (vertices[0].coords + vertices[1].coords + vertices[2].coords + vertices[3].coords)
                    / 4.,
            );
            for v in vertices.iter_mut() {
                *v += (middle - *v) * CORE_MARGIN;
            }

            for i in 0..4 {
                let triplet = vec![vertices[i], vertices[i + 1 & 3], vertices[i + 2 & 3]];
                let trimesh =
                    TriMesh::new(triplet, Some(normals.clone()), Some(indices.clone()), None);
                let mut sticker = window.add_trimesh(trimesh, scale);
                sticker.set_color(0., 0., 0.);
                sticker.enable_backface_culling(false);
            }
        }

        fn render_pyra_face(
            window: &mut Window,
            v0: Point3<f32>,
            v6: Point3<f32>,
            v9: Point3<f32>,
            [r, g, b]: [f32; 3],
        ) {
            let normals = vec![Vector3::z(), Vector3::z(), Vector3::z()];
            let indices = vec![Point2::new(0.0, 1.0)];
            let scale = Vector3::new(3.0, 3.0, 3.0);

            let v1 = v0 + (v6 - v0) / 3.0;
            let v2 = v0 + (v9 - v0) / 3.0;
            let v3 = v0 + (v6 - v0) * 2.0 / 3.0;
            let v4 = Point3::from((v0.coords + v9.coords + v6.coords) / 3.);
            let v5 = v0 + (v9 - v0) * 2.0 / 3.0;
            let v7 = v9 + (v6 - v9) * 2.0 / 3.0;
            let v8 = v9 + (v6 - v9) / 3.0;

            for mut triplet in [
                vec![v0, v1, v2],
                vec![v1, v3, v4],
                vec![v1, v4, v2],
                vec![v2, v4, v5],
                vec![v3, v6, v7],
                vec![v7, v4, v3],
                vec![v7, v8, v4],
                vec![v8, v5, v4],
                vec![v8, v9, v5],
            ] {
                let middle =
                    Point3::from((triplet[0].coords + triplet[1].coords + triplet[2].coords) / 3.);
                for v in triplet.iter_mut() {
                    *v += (middle - *v) * STICKER_MARGIN;
                }

                let trimesh =
                    TriMesh::new(triplet, Some(normals.clone()), Some(indices.clone()), None);
                let mut sticker = window.add_trimesh(trimesh, scale);
                sticker.set_color(r, g, b);
                sticker.enable_backface_culling(false);
            }
        }

        let v1 = Point3::new(-0.5, -0.5, -0.5);
        let v2 = Point3::new(0.5, -0.5, 0.5);
        let v3 = Point3::new(-0.5, 0.5, 0.5);
        let v4 = Point3::new(0.5, 0.5, -0.5);
        render_core(window, [v1, v2, v3, v4]);
        render_pyra_face(window, v4, v3, v2, Color::RED.as_rgb());
        render_pyra_face(window, v4, v2, v1, Color::GREEN.as_rgb());
        render_pyra_face(window, v4, v1, v3, Color::BLUE.as_rgb());
        render_pyra_face(window, v1, v2, v3, Color::YELLOW.as_rgb());

        vec![]
    }

    fn default_cam(&self) -> ArcBall {
        ArcBall::new(Point3::new(0.5, -0.5, -0.5), Point3::new(0., 0., 0.))
    }
}

impl Display for Pyraminx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn format(face: &[Color], line: usize) -> String {
            let start: usize = line * line;
            face[start..start + (line * 2 + 1)]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        }

        let faces: Vec<&[Color]> = vec![0, 1, 2]
            .into_iter()
            .map(|f| self.get_face(f))
            .collect();

        for line in 0..3 {
            writeln!(
                f,
                "{}{}",
                " ".repeat((2 - line) * 2).as_str(),
                faces
                    .iter()
                    .map(|face| format(face, line))
                    .collect::<Vec<String>>()
                    .join(" ".repeat((2 - line) * 4 + 1).as_str())
            )?;
        }

        let face = self.get_face(3);
        for line in 0..3 {
            writeln!(
                f,
                "{}{}{}",
                " ".repeat((3 + 3 - 1) * 2).as_str(),
                " ".repeat((line) * 2).as_str(),
                format(&face, 3 - line - 1)
            )?;
        }

        Ok(())
    }
}