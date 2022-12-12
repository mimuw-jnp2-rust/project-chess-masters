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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceType {
    King { x: u32, y: u32, color: PieceColor },
    Queen { x: u32, y: u32, color: PieceColor },
    Rook { x: u32, y: u32, color: PieceColor },
    Bishop { x: u32, y: u32, color: PieceColor },
    Knight { x: u32, y: u32, color: PieceColor },
    Pawn { x: u32, y: u32, color: PieceColor },
}

impl core::fmt::Display for PieceType {
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

impl PieceType {
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

    pub fn get_coordinates(&self) -> (u32, u32) {
        match self {
            Self::King { x, y, .. }
            | Self::Queen { x, y, .. }
            | Self::Rook { x, y, .. }
            | Self::Bishop { x, y, .. }
            | Self::Knight { x, y, .. }
            | Self::Pawn { x, y, .. } => (*x, *y),
        }
    }

    pub fn new(piece: &str, x: u32, y: u32, color: PieceColor) -> Self {
        match piece {
            "King" => PieceType::King { x, y, color },
            "Queen" => PieceType::Queen { x, y, color },
            "Rook" => PieceType::Rook { x, y, color },
            "Bishop" => PieceType::Bishop { x, y, color },
            "Knight" => PieceType::Knight { x, y, color },
            "Pawn" => PieceType::Pawn { x, y, color },
            _ => panic!("Invalid piece"),
        }
    }

    #[allow(dead_code)]
    pub fn print_piece(&self) {
        match &self {
            PieceType::King { x, y, .. } => println!("King at ({}, {})", x, y),
            PieceType::Queen { x, y, .. } => println!("Queen at ({}, {})", x, y),
            PieceType::Rook { x, y, .. } => println!("Rook at ({}, {})", x, y),
            PieceType::Bishop { x, y, .. } => println!("Bishop at ({}, {})", x, y),
            PieceType::Knight { x, y, .. } => println!("Knight at ({}, {})", x, y),
            PieceType::Pawn { x, y, .. } => println!("Pawn at ({}, {})", x, y),
        }
    }

    #[allow(dead_code)]
    pub fn get_type(&self) -> &'static str {
        match self {
            PieceType::King { .. } => "King",
            PieceType::Queen { .. } => "Queen",
            PieceType::Rook { .. } => "Rook",
            PieceType::Bishop { .. } => "Bishop",
            PieceType::Knight { .. } => "Knight",
            PieceType::Pawn { .. } => "Pawn",
        }
    }
}
