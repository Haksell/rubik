// TODO: in a mod with Move

use move_macro::moves;

use crate::r#move::Move;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] // TODO: remove PartialOrd, Ord
pub enum Trigger {
    U,
    U2,
    U3,
    RUR3,
    RU2R3,
    RU3R3,
    R3UR,
    R3U2R,
    R3U3R,
    FUF3,
    FU2F3,
    FU3F3,
    F3UF,
    F3U2F,
    F3U3F,
    LUL3,
    LU2L3,
    LU3L3,
    L3UL,
    L3U2L,
    L3U3L,
    BUB3,
    BU2B3,
    BU3B3,
    B3UB,
    B3U2B,
    B3U3B,
    Sledge0,
    Sledge1,
    Sledge2,
    Sledge3,
    Hedge0,
    Hedge1,
    Hedge2,
    Hedge3,
}

impl Trigger {
    pub fn moves(&self) -> Vec<Move> {
        // TODO: use moves! macro everywhere
        match self {
            Trigger::U => moves!("U"),
            Trigger::U2 => moves!("U2"),
            Trigger::U3 => moves!("U'"),
            Trigger::RUR3 => moves!("R U R'"),
            Trigger::RU2R3 => moves!("R U2 R'"),
            Trigger::RU3R3 => moves!("R U' R'"),
            Trigger::R3UR => moves!("R' U R"),
            Trigger::R3U2R => moves!("R' U2 R"),
            Trigger::R3U3R => moves!("R' U' R"),
            Trigger::FUF3 => moves!("F U F'"),
            Trigger::FU2F3 => moves!("F U2 F'"),
            Trigger::FU3F3 => moves!("F U' F'"),
            Trigger::F3UF => moves!("F' U F"),
            Trigger::F3U2F => moves!("F' U2 F"),
            Trigger::F3U3F => moves!("F' U' F"),
            Trigger::LUL3 => moves!("L U L'"),
            Trigger::LU2L3 => moves!("L U2 L'"),
            Trigger::LU3L3 => moves!("L U' L'"),
            Trigger::L3UL => moves!("L' U L"),
            Trigger::L3U2L => moves!("L' U2 L"),
            Trigger::L3U3L => moves!("L' U' L"),
            Trigger::BUB3 => moves!("B U B'"),
            Trigger::BU2B3 => moves!("B U2 B'"),
            Trigger::BU3B3 => moves!("B U' B'"),
            Trigger::B3UB => moves!("B' U B"),
            Trigger::B3U2B => moves!("B' U2 B"),
            Trigger::B3U3B => moves!("B' U' B"),
            Trigger::Sledge0 => moves!("R B' R' B"),
            Trigger::Sledge1 => moves!("R' F R F'"),
            Trigger::Sledge2 => moves!("L F' L' F"),
            Trigger::Sledge3 => moves!("L' B L B'"),
            Trigger::Hedge0 => moves!("B' R B R'"),
            Trigger::Hedge1 => moves!("F R' F' R"),
            Trigger::Hedge2 => moves!("F' L F L'"),
            Trigger::Hedge3 => moves!("B L' B' L"),
        }
    }

    // TODO: return Some(usize) instead of usize::MAX special value?
    pub fn slot(&self) -> usize {
        match self {
            Trigger::R3UR
            | Trigger::R3U2R
            | Trigger::R3U3R
            | Trigger::BUB3
            | Trigger::BU2B3
            | Trigger::BU3B3
            | Trigger::Sledge0
            | Trigger::Hedge0 => 0,
            Trigger::RUR3
            | Trigger::RU2R3
            | Trigger::RU3R3
            | Trigger::F3UF
            | Trigger::F3U2F
            | Trigger::F3U3F
            | Trigger::Sledge1
            | Trigger::Hedge1 => 1,
            Trigger::L3UL
            | Trigger::L3U2L
            | Trigger::L3U3L
            | Trigger::FUF3
            | Trigger::FU2F3
            | Trigger::FU3F3
            | Trigger::Sledge2
            | Trigger::Hedge2 => 2,
            Trigger::LUL3
            | Trigger::LU2L3
            | Trigger::LU3L3
            | Trigger::B3UB
            | Trigger::B3U2B
            | Trigger::B3U3B
            | Trigger::Sledge3
            | Trigger::Hedge3 => 3,
            Trigger::U | Trigger::U2 | Trigger::U3 => usize::MAX,
        }
    }

    pub fn len(&self) -> usize {
        match &self {
            Trigger::U | Trigger::U2 | Trigger::U3 => 1,
            Trigger::RUR3
            | Trigger::RU2R3
            | Trigger::RU3R3
            | Trigger::R3UR
            | Trigger::R3U2R
            | Trigger::R3U3R
            | Trigger::FUF3
            | Trigger::FU2F3
            | Trigger::FU3F3
            | Trigger::F3UF
            | Trigger::F3U2F
            | Trigger::F3U3F
            | Trigger::LUL3
            | Trigger::LU2L3
            | Trigger::LU3L3
            | Trigger::L3UL
            | Trigger::L3U2L
            | Trigger::L3U3L
            | Trigger::BUB3
            | Trigger::BU2B3
            | Trigger::BU3B3
            | Trigger::B3UB
            | Trigger::B3U2B
            | Trigger::B3U3B => 3,
            Trigger::Sledge0
            | Trigger::Sledge1
            | Trigger::Sledge2
            | Trigger::Sledge3
            | Trigger::Hedge0
            | Trigger::Hedge1
            | Trigger::Hedge2
            | Trigger::Hedge3 => 4,
        }
    }

    pub fn opposite(&self) -> Trigger {
        todo!()
    }
}

pub const TRIGGERS_BY_SLOT: &[&[Trigger]] = &[
    &[
        Trigger::R3UR,
        Trigger::R3U2R,
        Trigger::R3U3R,
        Trigger::BUB3,
        Trigger::BU2B3,
        Trigger::BU3B3,
        Trigger::Sledge0,
        Trigger::Hedge0,
    ],
    &[
        Trigger::RUR3,
        Trigger::RU2R3,
        Trigger::RU3R3,
        Trigger::F3UF,
        Trigger::F3U2F,
        Trigger::F3U3F,
        Trigger::Sledge1,
        Trigger::Hedge1,
    ],
    &[
        Trigger::L3UL,
        Trigger::L3U2L,
        Trigger::L3U3L,
        Trigger::FUF3,
        Trigger::FU2F3,
        Trigger::FU3F3,
        Trigger::Sledge2,
        Trigger::Hedge2,
    ],
    &[
        Trigger::LUL3,
        Trigger::LU2L3,
        Trigger::LU3L3,
        Trigger::B3UB,
        Trigger::B3U2B,
        Trigger::B3U3B,
        Trigger::Sledge3,
        Trigger::Hedge3,
    ],
];
