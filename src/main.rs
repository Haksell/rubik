use std::env;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Clone, Debug)]
pub enum Color {
    WHITE,
    YELLOW,
    RED,
    GREEN,
    BLUE,
    ORANGE,
}

impl Color {
    // TODO: #[repr(u8)] on pub enum Color instead?
    fn from_int(n: u8) -> Option<Color> {
        match n {
            0 => Some(Color::WHITE),
            1 => Some(Color::YELLOW),
            2 => Some(Color::RED),
            3 => Some(Color::GREEN),
            4 => Some(Color::BLUE),
            5 => Some(Color::ORANGE),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Move {
    FRONT,
    RIGHT,
    UP,
    BACK,
    LEFT,
    DOWN,
}

impl Move {
    fn from_int(n: u8) -> Option<Move> {
        match n {
            0 => Some(Move::FRONT),
            1 => Some(Move::RIGHT),
            2 => Some(Move::UP),
            3 => Some(Move::BACK),
            4 => Some(Move::LEFT),
            5 => Some(Move::DOWN),
            _ => None,
        }
    }

    fn from_str(s: &str) -> Option<Move> {
        todo!();
    }
}

pub struct Cube {
    faces: Vec<Vec<Color>>,
}

impl Cube {
    pub fn new(n: usize) -> Cube {
        println!("New Cube !");
        let mut faces = Vec::new();
        for i in 0..6 {
            faces.push(vec![Color::from_int(i).unwrap(); n * n]);
        }
        Cube { faces }
    }

    pub fn do_move(&self, _move: Move) {
        todo!();
    }

    pub fn scramble(&self, sequence: &str) -> Result<(), Error> {
        let as_moves = sequence.split_whitespace().map(Move::from_str);

        for mov in as_moves {
            match mov {
                Some(m) => self.do_move(m),
                None => return Err(Error),
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

    let cube = Cube::new(2);
    cube.scramble(&args[1])?;
    println!("{}", cube);

    Ok(())
}
