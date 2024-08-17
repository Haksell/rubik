// TODO: reflechir

use crate::{moves, r#move::Move};

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
            Trigger::U => vec![Move::U],
            Trigger::U2 => vec![Move::U2],
            Trigger::U3 => vec![Move::U3],
            Trigger::RUR3 => vec![Move::R, Move::U, Move::R3],
            Trigger::RU2R3 => vec![Move::R, Move::U2, Move::R3],
            Trigger::RU3R3 => vec![Move::R, Move::U3, Move::R3],
            Trigger::R3UR => vec![Move::R3, Move::U, Move::R],
            Trigger::R3U2R => vec![Move::R3, Move::U2, Move::R],
            Trigger::R3U3R => vec![Move::R3, Move::U3, Move::R],
            Trigger::FUF3 => vec![Move::F, Move::U, Move::F3],
            Trigger::FU2F3 => vec![Move::F, Move::U2, Move::F3],
            Trigger::FU3F3 => vec![Move::F, Move::U3, Move::F3],
            Trigger::F3UF => vec![Move::F3, Move::U, Move::F],
            Trigger::F3U2F => vec![Move::F3, Move::U2, Move::F],
            Trigger::F3U3F => vec![Move::F3, Move::U3, Move::F],
            Trigger::LUL3 => vec![Move::L, Move::U, Move::L3],
            Trigger::LU2L3 => vec![Move::L, Move::U2, Move::L3],
            Trigger::LU3L3 => vec![Move::L, Move::U3, Move::L3],
            Trigger::L3UL => vec![Move::L3, Move::U, Move::L],
            Trigger::L3U2L => vec![Move::L3, Move::U2, Move::L],
            Trigger::L3U3L => vec![Move::L3, Move::U3, Move::L],
            Trigger::BUB3 => vec![Move::B, Move::U, Move::B3],
            Trigger::BU2B3 => vec![Move::B, Move::U2, Move::B3],
            Trigger::BU3B3 => vec![Move::B, Move::U3, Move::B3],
            Trigger::B3UB => vec![Move::B3, Move::U, Move::B],
            Trigger::B3U2B => vec![Move::B3, Move::U2, Move::B],
            Trigger::B3U3B => vec![Move::B3, Move::U3, Move::B],
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
}

// TODO: [&[Trigger]; 4]

pub const TRIGGERS_SLOT_0: &[Trigger] = &[
    Trigger::R3UR,
    Trigger::R3U2R,
    Trigger::R3U3R,
    Trigger::BUB3,
    Trigger::BU2B3,
    Trigger::BU3B3,
    Trigger::Sledge0,
    Trigger::Hedge0,
];

pub const TRIGGERS_SLOT_1: &[Trigger] = &[
    Trigger::RUR3,
    Trigger::RU2R3,
    Trigger::RU3R3,
    Trigger::F3UF,
    Trigger::F3U2F,
    Trigger::F3U3F,
    Trigger::Sledge1,
    Trigger::Hedge1,
];

pub const TRIGGERS_SLOT_2: &[Trigger] = &[
    Trigger::L3UL,
    Trigger::L3U2L,
    Trigger::L3U3L,
    Trigger::FUF3,
    Trigger::FU2F3,
    Trigger::FU3F3,
    Trigger::Sledge2,
    Trigger::Hedge2,
];

pub const TRIGGERS_SLOT_3: &[Trigger] = &[
    Trigger::LUL3,
    Trigger::LU2L3,
    Trigger::LU3L3,
    Trigger::B3UB,
    Trigger::B3U2B,
    Trigger::B3U3B,
    Trigger::Sledge3,
    Trigger::Hedge3,
];
