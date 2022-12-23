use crate::chess_pieces::*;
use crate::coordinates::*;
use bevy::prelude::{Component, Entity};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FieldColor {
    White,
    Black,
}

#[derive(Debug, Component, Clone, PartialEq, Eq)]
pub struct Field {
    pub entity: Entity,
    pub coordinates: Coordinates,
    pub color: FieldColor,
    pub piece: Option<Piece>,
}

impl Field {
    pub fn new(
        entity: Entity,
        coordinates: Coordinates,
        color: FieldColor,
        maybe_piece: Option<Piece>,
    ) -> Self {
        Self {
            entity,
            coordinates,
            color,
            piece: maybe_piece,
        }
    }
}
