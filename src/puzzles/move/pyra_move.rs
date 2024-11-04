use super::Move;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum PyraMove {
    R,
    U,
    B,
    L,
    R2,
    U2,
    B2,
    L2,
    TR,
    TU,
    TB,
    TL,
    TR2,
    TU2,
    TB2,
    TL2,
}

impl Move for PyraMove {}
