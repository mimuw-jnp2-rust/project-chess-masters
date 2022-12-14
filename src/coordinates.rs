// coordinates.rs
use crate::BOARD_SIZE;
use crate::FIELD_SIZE;
// use crate::WINDOW_HEIGHT;
// use crate::WINDOW_WIDTH;
use bevy::prelude::*;
use std::fmt::{self, Display, Formatter};
use std::ops::Add;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: i32, // or u16? i is for moves simulation
    pub y: i32,
}

impl Coordinates {
    pub fn in_board_bounds(&self) -> bool {
        self.x > 0 && self.y > 0 && self.x <= BOARD_SIZE as i32 && self.y <= BOARD_SIZE as i32
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        //write!(f, "({}, {})", ((65 + self.x) as u8) as char, 1 + self.y)
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn mouse_pos_to_coordinates(x: f32, y: f32, width: f32, height: f32) -> Coordinates {
    let left_down_x = ((width as f32) / 2.0) - ((BOARD_SIZE as f32 * FIELD_SIZE) / 2.0);
    let left_down_y = ((height as f32) / 2.0) - ((BOARD_SIZE as f32 * FIELD_SIZE) / 2.0);
    Coordinates {
        x: ((x - left_down_x) / FIELD_SIZE as f32).floor() as i32 + 1,
        y: ((y - left_down_y) / FIELD_SIZE as f32).floor() as i32 + 1,
    }
}

// system prints coordinates of the button that was clicked
#[allow(dead_code)]
pub fn print_coordinates(
    // mut commands: Commands,
    button_query: Query<(Entity, &Coordinates), With<Button>>,
    interaction_query: Query<&Interaction, Changed<Interaction>>,
) {
    for (entity, coordinates) in button_query.iter() {
        if let Ok(interaction) = interaction_query.get(entity) {
            if *interaction == Interaction::Clicked {
                //println!("Clicked button at ({}, {})", coordinates.x, coordinates.y);
                if (coordinates.x + coordinates.y) % 2 == 0 {
                    println!(
                        "Clicked white button at ({}, {})",
                        coordinates.x, coordinates.y
                    );
                } else {
                    println!(
                        "Clicked black button at ({}, {})",
                        coordinates.x, coordinates.y
                    );
                }
            }
        }
    }
}
