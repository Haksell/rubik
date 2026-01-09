// TODO: in a mod with Move

use {crate::r#move::Move, move_macro::moves};

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
            Self::U => moves!["U"],
            Self::U2 => moves!["U2"],
            Self::U3 => moves!["U'"],
            Self::RUR3 => moves!["R U R'"],
            Self::RU2R3 => moves!["R U2 R'"],
            Self::RU3R3 => moves!["R U' R'"],
            Self::R3UR => moves!["R' U R"],
            Self::R3U2R => moves!["R' U2 R"],
            Self::R3U3R => moves!["R' U' R"],
            Self::FUF3 => moves!["F U F'"],
            Self::FU2F3 => moves!["F U2 F'"],
            Self::FU3F3 => moves!["F U' F'"],
            Self::F3UF => moves!["F' U F"],
            Self::F3U2F => moves!["F' U2 F"],
            Self::F3U3F => moves!["F' U' F"],
            Self::LUL3 => moves!["L U L'"],
            Self::LU2L3 => moves!["L U2 L'"],
            Self::LU3L3 => moves!["L U' L'"],
            Self::L3UL => moves!["L' U L"],
            Self::L3U2L => moves!["L' U2 L"],
            Self::L3U3L => moves!["L' U' L"],
            Self::BUB3 => moves!["B U B'"],
            Self::BU2B3 => moves!["B U2 B'"],
            Self::BU3B3 => moves!["B U' B'"],
            Self::B3UB => moves!["B' U B"],
            Self::B3U2B => moves!["B' U2 B"],
            Self::B3U3B => moves!["B' U' B"],
            Self::Sledge0 => moves!["R B' R' B"],
            Self::Sledge1 => moves!["R' F R F'"],
            Self::Sledge2 => moves!["L F' L' F"],
            Self::Sledge3 => moves!["L' B L B'"],
            Self::Hedge0 => moves!["B' R B R'"],
            Self::Hedge1 => moves!["F R' F' R"],
            Self::Hedge2 => moves!["F' L F L'"],
            Self::Hedge3 => moves!["B L' B' L"],
        }
    }

    // TODO: return Some(usize) instead of usize::MAX special value?
    pub const fn slot(&self) -> usize {
        match self {
            Self::R3UR
            | Self::R3U2R
            | Self::R3U3R
            | Self::BUB3
            | Self::BU2B3
            | Self::BU3B3
            | Self::Sledge0
            | Self::Hedge0 => 0,
            Self::RUR3
            | Self::RU2R3
            | Self::RU3R3
            | Self::F3UF
            | Self::F3U2F
            | Self::F3U3F
            | Self::Sledge1
            | Self::Hedge1 => 1,
            Self::L3UL
            | Self::L3U2L
            | Self::L3U3L
            | Self::FUF3
            | Self::FU2F3
            | Self::FU3F3
            | Self::Sledge2
            | Self::Hedge2 => 2,
            Self::LUL3
            | Self::LU2L3
            | Self::LU3L3
            | Self::B3UB
            | Self::B3U2B
            | Self::B3U3B
            | Self::Sledge3
            | Self::Hedge3 => 3,
            Self::U | Self::U2 | Self::U3 => usize::MAX,
        }
    }

    pub const fn len(&self) -> usize {
        match &self {
            Self::U | Self::U2 | Self::U3 => 1,
            Self::RUR3
            | Self::RU2R3
            | Self::RU3R3
            | Self::R3UR
            | Self::R3U2R
            | Self::R3U3R
            | Self::FUF3
            | Self::FU2F3
            | Self::FU3F3
            | Self::F3UF
            | Self::F3U2F
            | Self::F3U3F
            | Self::LUL3
            | Self::LU2L3
            | Self::LU3L3
            | Self::L3UL
            | Self::L3U2L
            | Self::L3U3L
            | Self::BUB3
            | Self::BU2B3
            | Self::BU3B3
            | Self::B3UB
            | Self::B3U2B
            | Self::B3U3B => 3,
            Self::Sledge0
            | Self::Sledge1
            | Self::Sledge2
            | Self::Sledge3
            | Self::Hedge0
            | Self::Hedge1
            | Self::Hedge2
            | Self::Hedge3 => 4,
        }
    }
}

pub const TRIGGERS_BY_SLOT: [&[Trigger]; 4] = [
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
