use move_macro::moves;

use crate::{color::Color, r#move::Move, Cube};

pub(super) fn solve_last_layer_step(
    cube: &mut Cube<3>,
    alg_matcher: fn(&Cube<3>) -> Option<Vec<Move>>,
) -> Vec<Move> {
    let mut u_moves = 0;
    for _ in 0..4 {
        let moves = alg_matcher(cube);
        if let Some(moves) = moves {
            for _ in 0..u_moves {
                cube.do_move(Move::U3);
            }
            let rotated_moves: Vec<Move> = moves
                .into_iter()
                .map(|mut move_| {
                    for _ in 0..u_moves {
                        move_ = move_.rotate_y();
                    }
                    move_
                })
                .collect();
            for &move_ in &rotated_moves {
                cube.do_move(move_);
            }
            return rotated_moves;
        }
        u_moves += 1;
        cube.do_move(Move::U);
    }
    unreachable!();
}

pub(super) fn solve_auf(cube: &mut Cube<3>) -> Vec<Move> {
    let auf = match cube.faces[crate::Sticker::FU as usize] {
        Color::GREEN => vec![],
        Color::ORANGE => vec![Move::U],
        Color::BLUE => vec![Move::U2],
        Color::RED => vec![Move::U3],
        _ => unreachable!(),
    };
    for &move_ in &auf {
        cube.do_move(move_);
    }
    auf
}

pub(super) fn oll_matcher(cube: &Cube<3>) -> Option<Vec<Move>> {
    use crate::Sticker::*;
    match ((cube.faces[BUL as usize] == Color::WHITE) as u16) << 8
        | ((cube.faces[LBU as usize] == Color::WHITE) as u16) << 7
        | ((cube.faces[LU as usize] == Color::WHITE) as u16) << 6
        | ((cube.faces[LUF as usize] == Color::WHITE) as u16) << 5
        | ((cube.faces[FLU as usize] == Color::WHITE) as u16) << 4
        | ((cube.faces[FU as usize] == Color::WHITE) as u16) << 3
        | ((cube.faces[FUR as usize] == Color::WHITE) as u16) << 2
        | ((cube.faces[RFU as usize] == Color::WHITE) as u16) << 1
        | ((cube.faces[RU as usize] == Color::WHITE) as u16) << 0
    {
        0b000000000 => Some(vec![]),
        0b011101011 => Some(moves!("R U2 R2 F R F' U2 R' F R F'")),
        0b011011101 => Some(moves!("R U' R2 D' L F L' D R2 U R'")),
        0b101001101 => Some(moves!("L' R2 B R' B L U2 L' B L R'")),
        0b001011011 => Some(moves!("L R2 F' R F' L' U2 L F' L' R")),
        0b101100000 => Some(moves!("L' B2 R B R' B L")),
        0b011011010 => Some(moves!("L F2 R' F' R F' L'")),
        0b100001101 => Some(moves!("L F R' F R F2 L'")),
        0b000010011 => Some(moves!("L' B' R B' R' B2 L")),
        0b011001010 => Some(moves!("F' U' F L F' L' U L F L'")), // bad
        0b100101001 => Some(moves!("F U F' R' F R U' R' F' R")), // bad
        0b101000100 => Some(moves!("L' R2 B R' B R B2 R' B L R'")), // bad
        0b001011010 => Some(moves!("L R2 F' R F' R' F2 R F' L' R")), // bad
        0b100001100 => Some(moves!("L F' L' U' L F L' F' U F")),
        0b010011000 => Some(moves!("R' F R U R' F' R F U' F'")),
        0b100101000 => Some(moves!("L' B' L R' U' R U L' B L")),
        0b010011010 => Some(moves!("L F L' R U R' U' L F' L'")),
        0b001101001 => Some(moves!("F' L F L' U2 L' U B' U B L")), // bad
        0b101001001 => Some(moves!("F R U R' U F' U2 F' L F L'")), // bad
        0b001101011 => Some(moves!("R' U2 F R U R' U' F2 U2 F R")), // bad
        0b001001001 => Some(moves!("L' R' F' U2 L2 U2 L2 U2 L2 F L R")), // bad
        0b010100010 => Some(moves!("R U R' U R U' R' U R U2 R'")),
        0b010100100 => Some(moves!("R U2 R2 U' R2 U' R2 U2 R")),
        0b000010100 => Some(moves!("R2 D R' U2 R D' R' U2 R'")),
        0b010000000 => Some(moves!("R U R D R' U' R D' R2")),
        0b010000100 => Some(moves!("F' L F R' F' L' F R")),
        0b000010010 => Some(moves!("R' U' R U' R' U2 R")),
        0b100000100 => Some(moves!("R U R' U R U2 R'")),
        0b000001001 => Some(moves!("R2 F2 L F L' F2 R F' R")), // bad
        0b011001000 => Some(moves!("F R' U' R2 U' R2 U2 R U' F'")), // bad
        0b010001001 => Some(moves!("F' L U L2 U L2 U2 L' U F")), // bad
        0b101011000 => Some(moves!("R' U' F U R U' R' F' R")),
        0b000001101 => Some(moves!("L U F' U' L' U L F L'")), // bad
        0b100011000 => Some(moves!("R U R' U' R' F R F'")),
        0b010001000 => Some(moves!("R U R' U' B' R' F R F' B")),
        0b001010000 => Some(moves!("R U2 R2 F R F' R U2 R'")),
        0b001101000 => Some(moves!("R' F' U' F2 U R U' R' F' R")),
        0b000011001 => Some(moves!("F R' F' R U R U' R'")),
        0b100001011 => Some(moves!("L F U F2 U' L' U L F L'")), // bad
        0b100001010 => Some(moves!("L F' L' U' L U F U' L'")),
        0b000101000 => Some(moves!("R' F R U R' U' F' U R")),
        0b100001001 => Some(moves!("F U R2 D R' U' R D' R2 F'")),
        0b101001000 => Some(moves!("F' U' L2 D' L U L' D L2 F")), // bad
        0b000011101 => Some(moves!("R' U' F' U F R")),
        0b000001011 => Some(moves!("F U R U' R' F'")),
        0b010101000 => Some(moves!("F R U R' U' F'")),
        0b001000011 => Some(moves!("R' U' R' F R F' U R")),
        0b100101011 => Some(moves!("R' F' U' F U F' U' F U R")),
        0b010101101 => Some(moves!("F R U R' U' R U R' U' F'")),
        0b011101100 => Some(moves!("L F' L2 B L2 F L2 B' L")),
        0b011100100 => Some(moves!("L' B L2 F' L2 B' L2 F L'")),
        0b100011010 => Some(moves!("F U R U' R' U R U' R' F'")),
        0b011100101 => Some(moves!("R' F' U' F U' R U R' U R")),
        0b011100010 => Some(moves!("L' B' R B' R' B R B' R' B2 L")),
        0b011101010 => Some(moves!("L F R' F R F' R' F R F2 L'")),
        0b011100011 => Some(moves!("R U2 R2 U' R U' R' U2 F R F'")), // bad
        0b010101010 => Some(moves!("R B L B' R2 B U L' U' B' R")),   // bad
        0b001000001 => Some(moves!("F' B U' F U F B' R' F' R")),
        _ => None,
    }
}

pub(super) fn pll_matcher(cube: &Cube<3>) -> Option<Vec<Move>> {
    use crate::Sticker::*;

    let reference_sticker = cube.faces[FLU as usize].side();
    match (
        (reference_sticker - cube.faces[FU as usize].side()) & 3,
        (reference_sticker - cube.faces[FUR as usize].side()) & 3,
        (reference_sticker - cube.faces[RU as usize].side()) & 3,
        (reference_sticker - cube.faces[RUB as usize].side()) & 3,
    ) {
        (0, 0, 1, 1) => Some(vec![]),
        (1, 1, 2, 0) => Some(moves!("L2 B2 L' F' L B2 L' F L'")),
        (2, 2, 3, 0) => Some(moves!("L F' L B2 L' F L B2 L2")),
        (3, 2, 0, 1) => Some(moves!("L U' R D2 R' U R L' U' L D2 L' U R'")),
        (0, 0, 3, 2) => Some(moves!("R' U R U' R2 F' U' F U R F R' F' R2")),
        (1, 1, 3, 0) => Some(moves!("R2 D B' U B' U' B D' R2 F' U F")),
        (2, 2, 1, 0) => Some(moves!("F' U' F R2 D B' U B U' B D' R2")),
        (2, 1, 3, 0) => Some(moves!("R2 D' F U' F U F' D R2 B U' B'")),
        (3, 1, 2, 0) => Some(moves!("R U R' F2 D' L U' L' U L' D F2")),
        (2, 0, 3, 1) => Some(moves!("R2 F2 B2 L2 D R2 F2 B2 L2")),
        (0, 0, 1, 2) => Some(moves!("R' U L' U2 R U' R' U2 R L")),
        (1, 1, 0, 0) => Some(moves!("L R U2 R' U' R U2 L' U R'")),
        (2, 2, 1, 1) => Some(moves!("R U' L U2 R' U L' R U' L U2 R' U L'")),
        (0, 2, 3, 1) => Some(moves!("R' U L' U2 R U' L R' U L' U2 R U' L")),
        (2, 1, 1, 2) => Some(moves!("L2 F' L' U' L' U L F L' U2 L U2 L'")),
        (3, 1, 1, 0) => Some(moves!("R2 F R U R U' R' F' R U2 R' U2 R")),
        (0, 1, 3, 0) => Some(moves!("R2 D B2 D' R2 F2 D' L2 D F2")),
        (1, 0, 2, 1) => Some(moves!("R2 U' F B' R2 F' B U' R2")),
        (2, 0, 0, 1) => Some(moves!("R2 U F B' R2 F' B U R2")),
        (3, 2, 2, 1) => Some(moves!("F' U F' U' R' F' R2 U' R' U R' F R F")),
        (0, 2, 1, 1) => Some(moves!("R2 U' R2 U' R2 U F U F' R2 F U' F'")),
        (1, 0, 0, 1) => Some(moves!("R B' R' B F R' F B' R' B R F2")),
        _ => None,
    }
}
