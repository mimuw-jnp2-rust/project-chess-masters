use crate::coordinates::*;
use bevy::prelude::Component;

/// The color of a field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FieldColor {
    White,
    Black,
}

#[derive(Debug, Component)]
pub struct Field {
    coordinates: Coordinates,
    color: FieldColor,
}

impl Field {}