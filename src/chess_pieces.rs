use bevy::prelude::Component;

/// The color of a piece.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceColor {
    White,
    Black,
}

impl core::fmt::Display for PieceColor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::White => "White",
                Self::Black => "Black",
            }
        )
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    King { x: i32, y: i32, color: PieceColor },
    Queen { x: i32, y: i32, color: PieceColor },
    Rook { x: i32, y: i32, color: PieceColor },
    Bishop { x: i32, y: i32, color: PieceColor },
    Knight { x: i32, y: i32, color: PieceColor },
    Pawn { x: i32, y: i32, color: PieceColor },
}

impl core::fmt::Display for Piece {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            match self.get_color() {
                PieceColor::Black => match self {
                    Self::King { .. } => "♔",
                    Self::Queen { .. } => "♕",
                    Self::Rook { .. } => "♖",
                    Self::Knight { .. } => "♘",
                    Self::Bishop { .. } => "♗",
                    Self::Pawn { .. } => "♙",
                },
                PieceColor::White => match self {
                    Self::King { .. } => "♚",
                    Self::Queen { .. } => "♛",
                    Self::Rook { .. } => "♜",
                    Self::Knight { .. } => "♞",
                    Self::Bishop { .. } => "♝",
                    Self::Pawn { .. } => "♟︎",
                },
            }
        )
    }
}

impl Piece {
    pub fn get_color(&self) -> PieceColor {
        match self {
            Self::King { x: _, y: _, color }
            | Self::Queen { x: _, y: _, color }
            | Self::Rook { x: _, y: _, color }
            | Self::Bishop { x: _, y: _, color }
            | Self::Knight { x: _, y: _, color }
            | Self::Pawn { x: _, y: _, color } => *color,
        }
    }

    pub fn new(piece: &str, x: i32, y: i32, color: PieceColor) -> Self {
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

    #[allow(dead_code)]
    pub fn print_piece(&self) {
        match &self {
            Piece::King { x, y, .. } => println!("King at ({}, {})", x, y),
            Piece::Queen { x, y, .. } => println!("Queen at ({}, {})", x, y),
            Piece::Rook { x, y, .. } => println!("Rook at ({}, {})", x, y),
            Piece::Bishop { x, y, .. } => println!("Bishop at ({}, {})", x, y),
            Piece::Knight { x, y, .. } => println!("Knight at ({}, {})", x, y),
            Piece::Pawn { x, y, .. } => println!("Pawn at ({}, {})", x, y),
        }
    }

    #[allow(dead_code)]
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
