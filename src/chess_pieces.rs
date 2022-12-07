#[derive(Debug)]
pub enum Piece {
    King { x: i32, y: i32, color: char },
    Queen { x: i32, y: i32, color: char },
    Rook { x: i32, y: i32, color: char },
    Bishop { x: i32, y: i32, color: char },
    Knight { x: i32, y: i32, color: char },
    Pawn { x: i32, y: i32, color: char },
}

impl Piece {
    pub fn new(piece: &str, x: i32, y: i32, color: char) -> Self {
        match piece {
            "King" => Piece::King { x, y, color },
            "Queen" => Piece::Queen { x, y, color },
            "Rook" => Piece::Rook { x, y, color },
            "Bishop" => Piece::Bishop { x, y, color },
            "Knight" => Piece::Knight { x, y, color },
            "Pawn" => Piece::Pawn { x, y, color },
            _ => panic!("Invalid piece"),
        }
    }

    pub fn print(&self) {
        match &self {
            Piece::King { x, y, color } => println!("King at ({}, {})", x, y),
            Piece::Queen { x, y, color } => println!("Queen at ({}, {})", x, y),
            Piece::Rook { x, y, color } => println!("Rook at ({}, {})", x, y),
            Piece::Bishop { x, y, color } => println!("Bishop at ({}, {})", x, y),
            Piece::Knight { x, y, color } => println!("Knight at ({}, {})", x, y),
            Piece::Pawn { x, y, color } => println!("Pawn at ({}, {})", x, y),
        }
    }

    pub fn get_type(&self) -> &'static str {
        match self {
            Self::King(_, _, _) => "King",
            Piece::Queen {
                x: _,
                y: _,
                color: _,
            } => "Queen",
            Piece::Rook { x, y, color } => "Rook",
            Piece::Bishop { x, y, color } => "Bishop",
            Piece::Knight { x, y, color } => "Knight",
            Piece::Pawn { x, y, color } => "Pawn",
        }
    }
}
