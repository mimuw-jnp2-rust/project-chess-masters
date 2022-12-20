use crate::coordinates::*;
use bevy::prelude::{Component, Entity};

/// The color of a piece.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct Piece {
    pub entity: Option<Entity>,
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
    pub coordinates: Coordinates,
    pub border: bool,
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

impl core::fmt::Display for Piece {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            match self.piece_color {
                PieceColor::Black => match self.piece_type {
                    PieceType::King => "♔",
                    PieceType::Queen => "♕",
                    PieceType::Rook => "♖",
                    PieceType::Knight => "♘",
                    PieceType::Bishop => "♗",
                    PieceType::Pawn => "♙",
                },
                PieceColor::White => match self.piece_type {
                    PieceType::King => "♚",
                    PieceType::Queen => "♛",
                    PieceType::Rook => "♜",
                    PieceType::Knight => "♞",
                    PieceType::Bishop => "♝",
                    PieceType::Pawn => "♟︎",
                },
            }
        )
    }
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: PieceColor, coordinates: Coordinates) -> Self {
        Self {
            entity: None,
            piece_type,
            piece_color,
            coordinates,
            border: false,
        }
    }

    #[allow(dead_code)]
    pub fn print_piece(&self) {
        match &self.piece_type {
            PieceType::King => println!("King at ({})", self.coordinates),
            PieceType::Queen => println!("Queen at ({})", self.coordinates),
            PieceType::Rook => println!("Rook at ({})", self.coordinates),
            PieceType::Bishop => println!("Bishop at ({})", self.coordinates),
            PieceType::Knight => println!("Knight at ({})", self.coordinates),
            PieceType::Pawn => println!("Pawn at ({})", self.coordinates),
        }
    }

    #[allow(dead_code)]
    pub fn get_type(&self) -> &'static str {
        match self.piece_type {
            PieceType::King => "King",
            PieceType::Queen => "Queen",
            PieceType::Rook => "Rook",
            PieceType::Bishop => "Bishop",
            PieceType::Knight => "Knight",
            PieceType::Pawn => "Pawn",
        }
    }

    pub fn get_entity(&self) -> Option<Entity> {
        self.entity
    }

    pub fn set_entity(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }
}
