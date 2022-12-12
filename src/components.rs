use crate::chess_pieces::*;
use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Piece {
    pub piece_type: PieceType,
}
