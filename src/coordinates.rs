// coordinates.rs
use bevy::prelude::*;
use std::fmt::{self, Display, Formatter};
use std::ops::Add;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: i8, // or u16? i is for moves simulation
    pub y: i8,
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
        write!(f, "({}, {})", ((65 + self.x) as u8) as char, 1 + self.y)
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
