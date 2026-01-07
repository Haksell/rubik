use {
    crate::{
        color::Color,
        r#move::Move,
        solvers::{iddfs, DFSAble},
        Puzzle,
    },
    kiss3d::{
        camera::OrbitCamera3d,
        glamx::{Vec2, Vec3},
        prelude::GpuMesh3d,
        scene::SceneNode3d,
    },
    std::{
        cell::RefCell,
        f32,
        fmt::{Display, Formatter},
        hash::Hash,
        rc::Rc,
        vec,
    },
};

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
        match move_ {
            Move::R => {
                self.do_cycle(&[17, 22, 31]);
                self.do_cycle(&[12, 24, 33]);
                self.do_cycle(&[15, 19, 28]);
                self.do_cycle(&[16, 23, 32]);
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
    }

    fn solve(&self) -> Option<Vec<Move>> {
        Some(iddfs(self.clone()))
    }

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

    fn draw(&self, scene: &mut SceneNode3d) -> Vec<SceneNode3d> {
        fn draw_triangle(
            scene: &mut SceneNode3d,
            vertices: Vec<Vec3>,
            [r, g, b]: [f32; 3],
        ) -> SceneNode3d {
            let mesh = Rc::new(RefCell::new(GpuMesh3d::new(
                vertices,
                vec![[0, 1, 2]],
                Some(vec![Vec3::Z, Vec3::Z, Vec3::Z]),
                Some(vec![Vec2::new(0.0, 1.0)]),
                true,
            )));
            let mut sticker = scene.add_mesh(mesh, Vec3::new(3.0, 3.0, 3.0));
            sticker.set_color(kiss3d::color::Color::new(r, g, b, 1.0));
            sticker.enable_backface_culling(false);
            sticker
        }

        fn render_core(scene: &mut SceneNode3d, mut vertices: [Vec3; 4]) {
            const CORE_MARGIN: f32 = 0.04;
            let middle = Vec3::from((vertices[0] + vertices[1] + vertices[2] + vertices[3]) / 4.);
            for v in vertices.iter_mut() {
                *v += (middle - *v) * CORE_MARGIN;
            }

            for i in 0..4 {
                draw_triangle(
                    scene,
                    vec![vertices[i], vertices[i + 1 & 3], vertices[i + 2 & 3]],
                    [0., 0., 0.],
                );
            }
        }

        fn render_pyra_face(
            scene: &mut SceneNode3d,
            v0: Vec3,
            v6: Vec3,
            v9: Vec3,
            face: &[Color],
        ) -> Vec<SceneNode3d> {
            let v1 = v0 + (v6 - v0) / 3.0;
            let v2 = v0 + (v9 - v0) / 3.0;
            let v3 = v0 + (v6 - v0) * 2.0 / 3.0;
            let v4 = Vec3::from((v0 + v9 + v6) / 3.);
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
                let middle = Vec3::from((triplet[0] + triplet[1] + triplet[2]) / 3.);
                for v in triplet.iter_mut() {
                    *v += (middle - *v) * STICKER_MARGIN;
                }
                draw_triangle(scene, triplet, sticker.as_rgb())
            })
            .collect()
        }

        let v1 = Vec3::new(-0.5, -0.5, -0.5);
        let v2 = Vec3::new(0.5, -0.5, 0.5);
        let v3 = Vec3::new(-0.5, 0.5, 0.5);
        let v4 = Vec3::new(0.5, 0.5, -0.5);

        render_core(scene, [v1, v2, v3, v4]);
        [
            render_pyra_face(scene, v4, v3, v2, self.get_face(0)),
            render_pyra_face(scene, v4, v2, v1, self.get_face(1)),
            render_pyra_face(scene, v4, v1, v3, self.get_face(2)),
            render_pyra_face(scene, v3, v1, v2, self.get_face(3)),
        ]
        .concat()
    }

    fn default_cam(&self) -> OrbitCamera3d {
        OrbitCamera3d::new(Vec3::new(0.5, 0.5, -0.5), Vec3::new(0., 0., 0.))
    }

    fn available_moves(&self) -> Vec<Move> {
        vec![
            Move::R,
            Move::U,
            Move::B,
            Move::L,
            Move::R2,
            Move::U2,
            Move::B2,
            Move::L2,
            Move::TR,
            Move::TU,
            Move::TB,
            Move::TL,
            Move::TR2,
            Move::TU2,
            Move::TB2,
            Move::TL2,
        ]
    }

    fn opposite_move(&self, move_: Move) -> Move {
        match move_ {
            Move::R => Move::R2,
            Move::U => Move::U2,
            Move::B => Move::B2,
            Move::L => Move::L2,
            Move::R2 => Move::R,
            Move::U2 => Move::U,
            Move::B2 => Move::B,
            Move::L2 => Move::L,
            Move::TR => Move::TR2,
            Move::TU => Move::TU2,
            Move::TB => Move::TB2,
            Move::TL => Move::TL2,
            Move::TR2 => Move::TR,
            Move::TU2 => Move::TU,
            Move::TB2 => Move::TB,
            Move::TL2 => Move::TL,
            _ => unreachable!(),
        }
    }

    fn parse_move(&self, value: &str) -> Result<Move, String> {
        match value {
            "R" => Ok(Move::R),
            "R'" | "R’" => Ok(Move::R2),
            "U" => Ok(Move::U),
            "U'" | "U’" => Ok(Move::U2),
            "B" => Ok(Move::B),
            "B'" | "B’" => Ok(Move::B2),
            "L" => Ok(Move::L),
            "L'" | "L’" => Ok(Move::L2),
            "r" => Ok(Move::TR),
            "u" => Ok(Move::TU),
            "b" => Ok(Move::TB),
            "l" => Ok(Move::TL),
            "r'" | "r’" => Ok(Move::TR2),
            "u'" | "u’" => Ok(Move::TU2),
            "b'" | "b’" => Ok(Move::TB2),
            "l'" | "l’" => Ok(Move::TL2),
            _ => Err(format!("Invalid move '{value}'")),
        }
    }
}

// TODO: better scramble for pyra

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

    fn presolve(&mut self) -> Vec<Move> {
        let mut moves = vec![];
        for (tip, single_move, single_color, double_move, double_color) in [
            (9, Move::TU, 2, Move::TU2, 20),
            (17, Move::TR, 23, Move::TR2, 32),
            (13, Move::TL, 34, Move::TL2, 7),
            (27, Move::TB, 25, Move::TB2, 5),
        ] {
            if self.faces[tip] == self.faces[single_color] {
                moves.push(single_move);
            } else if self.faces[tip] == self.faces[double_color] {
                moves.push(double_move);
            }
        }
        for &move_ in &moves {
            self.do_move(move_);
        }
        moves
    }
}

impl Display for Pyraminx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO: make less ugly
        fn draw_triangle_line(face: &[Color], line: usize, flip: bool) -> String {
            let start: usize = line * line;
            if flip {
                face[start..start + (line * 2 + 1)]
                    .iter()
                    .rev()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(" ")
            } else {
                face[start..start + (line * 2 + 1)]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(" ")
            }
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
                    .map(|face| draw_triangle_line(face, line, false))
                    .collect::<Vec<String>>()
                    .join(" ".repeat((2 - line) * 4 + 1).as_str())
            )?;
        }

        let face = self.get_face(3);
        for line in 0..3 {
            writeln!(
                f,
                "{}{}",
                " ".repeat((3 + 3 - 1 + line) * 2).as_str(),
                draw_triangle_line(&face, 3 - line - 1, true),
            )?;
        }

        Ok(())
    }
}
