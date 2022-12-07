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
            Piece::King { x, y, .. } => println!("King at ({}, {})", x, y),
            Piece::Queen { x, y, .. } => println!("Queen at ({}, {})", x, y),
            Piece::Rook { x, y, .. } => println!("Rook at ({}, {})", x, y),
            Piece::Bishop { x, y, .. } => println!("Bishop at ({}, {})", x, y),
            Piece::Knight { x, y, .. } => println!("Knight at ({}, {})", x, y),
            Piece::Pawn { x, y, .. } => println!("Pawn at ({}, {})", x, y),
        }
    }

    pub fn get_type(&self) -> &'static str {
        match self {
            Piece::King { .. } => "King",
            Piece::Queen { .. } => "Queen",
            Piece::Rook { .. } => "Rook",
            Piece::Bishop { .. } => "Bishop",
            Piece::Knight { .. } => "Knight",
            Piece::Pawn { .. } => "Pawn",
        }
    }
}
