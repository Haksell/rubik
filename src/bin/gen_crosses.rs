use std::{collections::VecDeque, usize};

use rubik::{color::Color, cube::Cube, r#move::Move, EDGES};

const NUM_CROSSES: usize = 24 * 22 * 20 * 18;

fn main() {
    let cube = Cube::<3>::new();
    let mut moves: &[Option<Move>; NUM_CROSSES] = &[None; NUM_CROSSES];
    let mut queue = VecDeque::new();
    queue.push_back((cube, Move::D));
    let mut remaining_crosses = NUM_CROSSES;
    while remaining_crosses > 0 {
        let (cube, last_move) = queue.pop_front().unwrap();
    }
}

fn cross_index(cube: &Cube<3>) -> usize {
    let mut yellow_green: usize = usize::MAX;
    let mut yellow_red: usize = usize::MAX;
    let mut yellow_blue: usize = usize::MAX;
    let mut yellow_orange: usize = usize::MAX;
    for (i, &(s1, s2)) in EDGES.iter().enumerate() {
        if cube.faces[s1 as usize] == Color::YELLOW {
            match cube.faces[s2 as usize] {
                Color::GREEN => yellow_green = 2 * i,
                Color::RED => yellow_red = 2 * i,
                Color::BLUE => yellow_blue = 2 * i,
                Color::ORANGE => yellow_orange = 2 * i,
                _ => unreachable!(),
            }
        } else if cube.faces[s2 as usize] == Color::YELLOW {
            match cube.faces[s1 as usize] {
                Color::GREEN => yellow_green = 2 * i + 1,
                Color::RED => yellow_red = 2 * i + 1,
                Color::BLUE => yellow_blue = 2 * i + 1,
                Color::ORANGE => yellow_orange = 2 * i + 1,
                _ => unreachable!(),
            }
        }
    }
    if yellow_red > yellow_green {
        yellow_red -= 2;
    }
    if yellow_blue > yellow_green {
        yellow_blue -= 2;
    }
    if yellow_blue > yellow_red {
        yellow_blue -= 2;
    }
    if yellow_orange > yellow_green {
        yellow_orange -= 2;
    }
    if yellow_orange > yellow_red {
        yellow_orange -= 2;
    }
    if yellow_orange > yellow_blue {
        yellow_orange -= 2;
    }
    yellow_orange + 18 * yellow_blue + 18 * 20 * yellow_red + 18 * 20 * 22 * yellow_green
}

#[cfg(test)]
mod tests {
    use crate::{cross_index, NUM_CROSSES};
    use rubik::{cube::Cube, r#move::Move};

    #[test]
    fn test_cross_index_solved() {
        let mut cube = Cube::<3>::new();
        assert_eq!(cross_index(&cube), 0);
        cube.do_move(Move::R);
        cube.do_move(Move::U);
        cube.do_move(Move::R3);
        cube.do_move(Move::U3);
        assert_eq!(cross_index(&cube), 0);
    }

    #[test]
    fn test_cross_index_random() {
        let mut cube = Cube::<3>::new();
        for _ in 0..100 {
            cube.do_move(Move::random());
            assert!(cross_index(&cube) < NUM_CROSSES);
        }
    }
}
