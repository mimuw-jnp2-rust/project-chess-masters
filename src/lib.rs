use bevy::prelude::*;
// use coordinates::*;
pub mod coordinates;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 800.0;
pub const FIELD_SIZE: f32 = 70.0;
pub const BOARD_SIZE: u32 = 8;

pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const WHITE_BUTTON: Color = Color::rgb(0.9, 0.9, 0.9);
pub const BLACK_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RED_BUTTON: Color = Color::rgb(0.9, 0.1, 0.1);
