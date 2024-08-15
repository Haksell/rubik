// TODO: reflechir

use crate::r#move::Move;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Trigger {
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
    U,
    U2,
    U3,
}

impl Trigger {
    pub fn moves(&self) -> Vec<Move> {
        match self {
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
            Trigger::U => vec![Move::U],
            Trigger::U2 => vec![Move::U2],
            Trigger::U3 => vec![Move::U3],
        }
    }

    pub fn slot(&self) -> usize {
        match self {
            Trigger::R3UR
            | Trigger::R3U2R
            | Trigger::R3U3R
            | Trigger::BUB3
            | Trigger::BU2B3
            | Trigger::BU3B3 => 0,
            Trigger::RUR3
            | Trigger::RU2R3
            | Trigger::RU3R3
            | Trigger::F3UF
            | Trigger::F3U2F
            | Trigger::F3U3F => 1,
            Trigger::L3UL
            | Trigger::L3U2L
            | Trigger::L3U3L
            | Trigger::FUF3
            | Trigger::FU2F3
            | Trigger::FU3F3 => 2,
            Trigger::LUL3
            | Trigger::LU2L3
            | Trigger::LU3L3
            | Trigger::B3UB
            | Trigger::B3U2B
            | Trigger::B3U3B => 3,
            _ => usize::MAX,
        }
    }
}

// TODO: [[Trigger; 6]; 4]

pub const TRIGGERS_SLOT_0: [Trigger; 6] = [
    Trigger::R3UR,
    Trigger::R3U2R,
    Trigger::R3U3R,
    Trigger::BUB3,
    Trigger::BU2B3,
    Trigger::BU3B3,
];

pub const TRIGGERS_SLOT_1: [Trigger; 6] = [
    Trigger::RUR3,
    Trigger::RU2R3,
    Trigger::RU3R3,
    Trigger::F3UF,
    Trigger::F3U2F,
    Trigger::F3U3F,
];

pub const TRIGGERS_SLOT_2: [Trigger; 6] = [
    Trigger::L3UL,
    Trigger::L3U2L,
    Trigger::L3U3L,
    Trigger::FUF3,
    Trigger::FU2F3,
    Trigger::FU3F3,
];

pub const TRIGGERS_SLOT_3: [Trigger; 6] = [
    Trigger::LUL3,
    Trigger::LU2L3,
    Trigger::LU3L3,
    Trigger::B3UB,
    Trigger::B3U2B,
    Trigger::B3U3B,
];
