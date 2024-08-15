use crate::{color::Color, cube::Cube, r#move::Move, EDGES};

pub const NUM_CROSSES: usize = 24 * 22 * 20 * 18;

#[allow(dead_code)]
pub fn cfop(start: Cube<3>) -> Option<Vec<Move>> {
    if start.is_solved() {
        Some(vec![])
    } else {
        None
    }
}

impl Cube<3> {
    fn is_cross_solved(&self) -> bool {
        use crate::Sticker::*;
        return self.faces[DF as usize] == Color::YELLOW
            && self.faces[DR as usize] == Color::YELLOW
            && self.faces[DB as usize] == Color::YELLOW
            && self.faces[DL as usize] == Color::YELLOW
            && self.faces[FD as usize] == Color::GREEN
            && self.faces[RD as usize] == Color::RED
            && self.faces[BD as usize] == Color::BLUE
            && self.faces[LD as usize] == Color::ORANGE;
    }

    pub fn cross_index(&self) -> usize {
        let mut yellow_green: usize = usize::MAX;
        let mut yellow_red: usize = usize::MAX;
        let mut yellow_blue: usize = usize::MAX;
        let mut yellow_orange: usize = usize::MAX;
        for (i, &(s1, s2)) in EDGES.iter().enumerate() {
            if self.faces[s1 as usize] == Color::YELLOW {
                match self.faces[s2 as usize] {
                    Color::GREEN => yellow_green = 2 * i,
                    Color::RED => yellow_red = 2 * i,
                    Color::BLUE => yellow_blue = 2 * i,
                    Color::ORANGE => yellow_orange = 2 * i,
                    _ => unreachable!(),
                }
            } else if self.faces[s2 as usize] == Color::YELLOW {
                match self.faces[s1 as usize] {
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
}

#[cfg(test)]
mod tests {
    use super::{Cube, NUM_CROSSES};
    use crate::r#move::Move;

    #[test]
    fn test_is_cross_solved() {
        let mut cube = Cube::<3>::new();
        assert!(cube.is_cross_solved());

        // sexy move
        cube.do_move(Move::R);
        cube.do_move(Move::U);
        cube.do_move(Move::R3);
        cube.do_move(Move::U3);
        assert!(cube.is_cross_solved());

        // 3G
        cube.do_move(Move::L3);
        cube.do_move(Move::U2);
        cube.do_move(Move::L);
        assert!(cube.is_cross_solved());

        // D4
        cube.do_move(Move::D);
        assert!(!cube.is_cross_solved());
        cube.do_move(Move::D);
        assert!(!cube.is_cross_solved());
        cube.do_move(Move::D);
        assert!(!cube.is_cross_solved());
        cube.do_move(Move::D);
        assert!(cube.is_cross_solved());

        // PLL I
        cube.do_move(Move::L2);
        cube.do_move(Move::R2);
        cube.do_move(Move::U2);
        cube.do_move(Move::L2);
        cube.do_move(Move::R2);
        cube.do_move(Move::D2);
        assert!(!cube.is_cross_solved());
    }

    #[test]
    fn test_cross_index_solved() {
        let mut cube = Cube::<3>::new();
        assert_eq!(cube.cross_index(), 0);
        cube.do_move(Move::R);
        cube.do_move(Move::U);
        cube.do_move(Move::R3);
        cube.do_move(Move::U3);
        assert_eq!(cube.cross_index(), 0);
    }

    #[test]
    fn test_cross_index_random() {
        let mut cube = Cube::<3>::new();
        for _ in 0..100 {
            cube.do_move(Move::random());
            assert!(cube.cross_index() < NUM_CROSSES);
        }
    }
}
