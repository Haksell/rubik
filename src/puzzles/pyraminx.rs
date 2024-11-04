use crate::color::Color;
use crate::r#move::Move;
use crate::solvers::{iddfs, DFSAble};
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

    fn get_face(&self, face: usize) -> &[Color] {
        let start = face * 9;
        let end = (face + 1) * 9;
        &self.faces[start..end]
    }

    // TODO: in Puzzle
    fn do_cycle(&mut self, stickers: &[usize]) {
        let last = self.faces[stickers[stickers.len() - 1]];
        for i in (1..stickers.len()).rev() {
            self.faces[stickers[i]] = self.faces[stickers[i - 1]];
        }
        self.faces[stickers[0]] = last;
    }
}

impl Puzzle for Pyraminx {
    fn do_move(&mut self, move_: Move) {
        // TODO: per puzzle moves enum
        // TODO: per puzzle sticker enum
        // TODO: reuse same cycles (R/R2/r/r2)
        // TODO: test pyraminx scrambles
        println!("{:?}", move_);
        match move_ {
            Move::R => {
                self.do_cycle(&[17, 22, 31]);
                self.do_cycle(&[12, 24, 33]);
                self.do_cycle(&[16, 19, 28]);
                self.do_cycle(&[15, 23, 32]);
            }
            Move::U => {
                self.do_cycle(&[9, 0, 18]);
                self.do_cycle(&[10, 1, 19]);
                self.do_cycle(&[11, 2, 20]);
                self.do_cycle(&[12, 3, 21]);
            }
            Move::B => {
                self.do_cycle(&[26, 4, 27]);
                self.do_cycle(&[21, 6, 28]);
                self.do_cycle(&[25, 5, 29]);
                self.do_cycle(&[24, 1, 30]);
            }
            Move::L => {
                self.do_cycle(&[8, 13, 35]);
                self.do_cycle(&[10, 33, 6]);
                self.do_cycle(&[14, 34, 7]);
                self.do_cycle(&[15, 30, 3]);
            }
            Move::TR => {
                self.do_cycle(&[17, 22, 31]);
            }
            Move::TU => {
                self.do_cycle(&[9, 0, 18]);
            }
            Move::TB => {
                self.do_cycle(&[26, 4, 27]);
            }
            Move::TL => {
                self.do_cycle(&[8, 13, 35]);
            }
            Move::R2 => {
                for _ in 0..2 {
                    self.do_move(Move::R);
                }
            }
            Move::U2 => {
                for _ in 0..2 {
                    self.do_move(Move::U);
                }
            }
            Move::B2 => {
                for _ in 0..2 {
                    self.do_move(Move::B);
                }
            }
            Move::L2 => {
                for _ in 0..2 {
                    self.do_move(Move::L);
                }
            }
            Move::TR2 => {
                self.do_cycle(&[31, 22, 17]);
            }
            Move::TU2 => {
                self.do_cycle(&[18, 0, 9]);
            }
            Move::TB2 => {
                self.do_cycle(&[27, 4, 26]);
            }
            Move::TL2 => {
                self.do_cycle(&[35, 13, 8]);
            }
            _ => panic!("Invalid move for pyraminx {:?}", move_),
        }
        println!("{:?}", self.faces);
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

    fn draw(&self, window: &mut Window) -> Vec<SceneNode> {
        fn draw_triangle(
            window: &mut Window,
            vertices: Vec<Point3<f32>>,
            [r, g, b]: [f32; 3],
        ) -> SceneNode {
            let trimesh = TriMesh::new(
                vertices,
                Some(vec![Vector3::z(), Vector3::z(), Vector3::z()]),
                Some(vec![Point2::new(0.0, 1.0)]),
                None,
            );
            let mut sticker = window.add_trimesh(trimesh, Vector3::new(3.0, 3.0, 3.0));
            sticker.set_color(r, g, b);
            sticker.enable_backface_culling(false);
            sticker
        }

        fn render_core(window: &mut Window, mut vertices: [Point3<f32>; 4]) -> Vec<SceneNode> {
            const CORE_MARGIN: f32 = 0.04;
            let middle = Point3::from(
                (vertices[0].coords + vertices[1].coords + vertices[2].coords + vertices[3].coords)
                    / 4.,
            );
            for v in vertices.iter_mut() {
                *v += (middle - *v) * CORE_MARGIN;
            }

            (0..4)
                .map(|i| {
                    draw_triangle(
                        window,
                        vec![vertices[i], vertices[i + 1 & 3], vertices[i + 2 & 3]],
                        [0., 0., 0.],
                    )
                })
                .collect()
        }

        fn render_pyra_face(
            window: &mut Window,
            v0: Point3<f32>,
            v6: Point3<f32>,
            v9: Point3<f32>,
            face: &[Color],
        ) -> Vec<SceneNode> {
            let v1 = v0 + (v6 - v0) / 3.0;
            let v2 = v0 + (v9 - v0) / 3.0;
            let v3 = v0 + (v6 - v0) * 2.0 / 3.0;
            let v4 = Point3::from((v0.coords + v9.coords + v6.coords) / 3.);
            let v5 = v0 + (v9 - v0) * 2.0 / 3.0;
            let v7 = v9 + (v6 - v9) * 2.0 / 3.0;
            let v8 = v9 + (v6 - v9) / 3.0;

            [
                vec![v0, v1, v2],
                vec![v1, v3, v4],
                vec![v1, v4, v2],
                vec![v2, v4, v5],
                vec![v3, v6, v7],
                vec![v7, v4, v3],
                vec![v7, v8, v4],
                vec![v8, v5, v4],
                vec![v8, v9, v5],
            ]
            .into_iter()
            .zip(face)
            .map(|(mut triplet, sticker)| {
                const STICKER_MARGIN: f32 = 0.1;
                let middle =
                    Point3::from((triplet[0].coords + triplet[1].coords + triplet[2].coords) / 3.);
                for v in triplet.iter_mut() {
                    *v += (middle - *v) * STICKER_MARGIN;
                }
                draw_triangle(window, triplet, sticker.as_rgb())
            })
            .collect()
        }

        let v1 = Point3::new(-0.5, -0.5, -0.5);
        let v2 = Point3::new(0.5, -0.5, 0.5);
        let v3 = Point3::new(-0.5, 0.5, 0.5);
        let v4 = Point3::new(0.5, 0.5, -0.5);

        let mut scene_nodes = vec![];
        scene_nodes.extend_from_slice(&render_pyra_face(window, v4, v3, v2, self.get_face(0)));
        scene_nodes.extend_from_slice(&render_pyra_face(window, v4, v2, v1, self.get_face(1)));
        scene_nodes.extend_from_slice(&render_pyra_face(window, v4, v1, v3, self.get_face(2)));
        scene_nodes.extend_from_slice(&render_pyra_face(window, v3, v1, v2, self.get_face(3)));
        scene_nodes.extend_from_slice(&render_core(window, [v1, v2, v3, v4]));
        scene_nodes
    }

    fn default_cam(&self) -> ArcBall {
        ArcBall::new(Point3::new(0.5, -0.5, -0.5), Point3::new(0., 0., 0.))
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
