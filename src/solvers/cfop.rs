use crate::{color::Color, cube::Cube, r#move::Move};

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
}

#[cfg(test)]
mod tests {
    use super::Cube;
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
}