use std::convert::TryFrom;
use std::env;
use std::fmt::{Debug, Display, Error, Formatter};

#[repr(u8)]
#[derive(Clone, Debug, Copy)]
pub enum Color {
    WHITE,
    YELLOW,
    RED,
    GREEN,
    BLUE,
    ORANGE,
}

impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::WHITE),
            1 => Ok(Color::YELLOW),
            2 => Ok(Color::RED),
            3 => Ok(Color::GREEN),
            4 => Ok(Color::BLUE),
            5 => Ok(Color::ORANGE),
            _ => Err(()),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Debug, Copy)]
pub enum Move {
    F,
    R,
    U,
    B,
    L,
    D,
    F2,
    R2,
    U2,
    B2,
    L2,
    D2,
    F3,
    R3,
    U3,
    B3,
    L3,
    D3,
}

impl Move {
    pub fn from_int(index: u8) -> Option<Move> {
        match index {
            0 => Some(Move::F),
            1 => Some(Move::R),
            2 => Some(Move::U),
            3 => Some(Move::B),
            4 => Some(Move::L),
            5 => Some(Move::D),
            6 => Some(Move::F2),
            7 => Some(Move::R2),
            8 => Some(Move::U2),
            9 => Some(Move::B2),
            10 => Some(Move::L2),
            11 => Some(Move::D2),
            12 => Some(Move::F3),
            13 => Some(Move::R3),
            14 => Some(Move::U3),
            15 => Some(Move::B3),
            16 => Some(Move::L3),
            17 => Some(Move::D3),
            _ => None,
        }
    }

    pub fn as_int(&self) -> u8 {
        *self as u8
    }

    #[allow(dead_code)]
    fn opposite(&self) -> Move {
        let mut i = self.as_int();
        if i < 6 {
            i += 12;
        } else if i > 11 {
            i %= 12;
        }
        Move::from_int(i).unwrap()
    }
}

impl TryFrom<&str> for Move {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "F" => Ok(Move::F),
            "F2" => Ok(Move::F2),
            "F'" | "F’" => Ok(Move::F3),
            "R" => Ok(Move::R),
            "R2" => Ok(Move::R2),
            "R'" | "R’" => Ok(Move::R3),
            "U" => Ok(Move::U),
            "U2" => Ok(Move::U2),
            "U'" | "U’" => Ok(Move::U3),
            "B" => Ok(Move::B),
            "B2" => Ok(Move::B2),
            "B'" | "B’" => Ok(Move::B3),
            "L" => Ok(Move::L),
            "L2" => Ok(Move::L2),
            "L'" | "L’" => Ok(Move::L3),
            "D" => Ok(Move::D),
            "D2" => Ok(Move::D2),
            "D'" | "D’" => Ok(Move::D3),
            _ => Err(()),
        }
    }
}

pub struct Cube {
    faces: Vec<Color>,
    size: usize,
}

impl Cube {
    pub fn new(n: usize) -> Cube {
        println!("New Cube !");
        let mut faces = Vec::new();
        for i in 0..6 * n * n {
            let face = i / (n * n);
            // let _y = (i % (n * n)) / n;
            // let _x = (i % (n * n)) % n;
            let color = (face as u8).try_into();
            faces.push(color.unwrap());
        }
        Cube { faces, size: n }
    }

    pub fn get_face(&self, face: u8) -> Vec<Color> {
        let start = face as usize * self.size * self.size;
        let end = (face as usize + 1) * self.size * self.size;
        self.faces[start..end].to_vec()
    }

    // TODO
    pub fn do_move(&self, _move: Move) {
        println!("{:?}", _move);
    }

    pub fn scramble(&self, sequence: &str) -> Result<(), Error> {
        let as_moves = sequence.split_whitespace().map(Move::try_from);

        for mov in as_moves {
            match mov {
                Ok(m) => self.do_move(m),
                Err(_) => return Err(Error),
            }
        }

        Ok(())
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.faces
            .iter()
            .try_for_each(|faces| write!(f, "{:?}", faces))
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./rubik <mixing sequence>");
        return Err(Error); // TODO Better error handling
    }

    let cube = Cube::new(3);
    println!("{}", cube.faces.len());

    cube.scramble(&args[1])?;
    println!("{}", cube);

    Ok(())
}
