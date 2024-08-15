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
        const DF: usize = 28;
        const DR: usize = 32;
        const DB: usize = 34;
        const DL: usize = 30;

        const FD: usize = 25;
        const RD: usize = 16;
        const BD: usize = 52;
        const LD: usize = 43;

        return self.faces[DF] == Color::YELLOW
            && self.faces[DR] == Color::YELLOW
            && self.faces[DB] == Color::YELLOW
            && self.faces[DL] == Color::YELLOW
            && self.faces[FD] == Color::GREEN
            && self.faces[RD] == Color::RED
            && self.faces[BD] == Color::BLUE
            && self.faces[LD] == Color::ORANGE;
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
