use crate::{color::Color, cube::Cube, r#move::Move};

pub fn kociemba(start: Cube<3>) -> Option<Vec<Move>> {
    None
}

impl Cube<3> {
    fn is_g1(&self) -> bool {
        // TODO: all sticker constants in some module
        const FL: usize = 21;
        const FR: usize = 23;
        const BR: usize = 48;
        const BL: usize = 50;

        (0..9)
            .chain(27..36)
            .all(|i| self.faces[i] == Color::WHITE || self.faces[i] == Color::YELLOW)
            && (self.faces[FL] == Color::GREEN || self.faces[FL] == Color::BLUE)
            && (self.faces[FR] == Color::GREEN || self.faces[FR] == Color::BLUE)
            && (self.faces[BR] == Color::GREEN || self.faces[BR] == Color::BLUE)
            && (self.faces[BL] == Color::GREEN || self.faces[BL] == Color::BLUE)
    }
}

#[cfg(test)]
mod tests {
    use super::Cube;
    use crate::r#move::Move;

    #[test]
    fn test_is_g1() {
        let mut cube = Cube::<3>::new();
        assert!(cube.is_g1());

        // stay in G1
        cube.do_move(Move::R2);
        cube.do_move(Move::U);
        cube.do_move(Move::R2);
        assert!(cube.is_g1());

        // PLL T
        cube.do_move(Move::R);
        assert!(!cube.is_g1());
        cube.do_move(Move::U);
        assert!(!cube.is_g1());
        cube.do_move(Move::R3);
        assert!(!cube.is_g1());
        cube.do_move(Move::U3);
        assert!(!cube.is_g1());
        cube.do_move(Move::R3);
        assert!(!cube.is_g1());
        cube.do_move(Move::F);
        assert!(!cube.is_g1());
        cube.do_move(Move::R2);
        assert!(!cube.is_g1());
        cube.do_move(Move::U3);
        assert!(!cube.is_g1());
        cube.do_move(Move::R3);
        assert!(!cube.is_g1());
        cube.do_move(Move::U3);
        assert!(!cube.is_g1());
        cube.do_move(Move::R);
        assert!(!cube.is_g1());
        cube.do_move(Move::U);
        assert!(!cube.is_g1());
        cube.do_move(Move::R3);
        assert!(!cube.is_g1());
        cube.do_move(Move::F3);
        assert!(cube.is_g1());

        // stay in G1
        cube.do_move(Move::U);
        assert!(cube.is_g1());
        cube.do_move(Move::L2);
        assert!(cube.is_g1());
        cube.do_move(Move::D);
        assert!(cube.is_g1());
        cube.do_move(Move::F2);
        assert!(cube.is_g1());
        cube.do_move(Move::R2);
        assert!(cube.is_g1());
        cube.do_move(Move::U);
        assert!(cube.is_g1());
        cube.do_move(Move::D);
        assert!(cube.is_g1());
        cube.do_move(Move::B2);
        assert!(cube.is_g1());

        // break second layer only
        cube.do_move(Move::F3);
        cube.do_move(Move::R);
        cube.do_move(Move::L3);
        cube.do_move(Move::F);
        cube.do_move(Move::R);
        cube.do_move(Move::F3);
        cube.do_move(Move::R3);
        cube.do_move(Move::L);
        cube.do_move(Move::D);
        cube.do_move(Move::R3);
        cube.do_move(Move::D3);
        cube.do_move(Move::F);
        assert!(!cube.is_g1());
    }
}
