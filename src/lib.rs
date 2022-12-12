use bevy::prelude::*;
use coordinates::Coordinates;
// use coordinates::*;
pub mod board;
pub mod chess_pieces;
pub mod components;
pub mod coordinates;
pub mod field;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 800.0;
pub const FIELD_SIZE: f32 = 70.0;
pub const BOARD_SIZE: u32 = 8;

pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const WHITE_BUTTON: Color = Color::rgb(0.9, 0.9, 0.9);
pub const BLACK_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RED_BUTTON: Color = Color::rgb(0.9, 0.1, 0.1);
pub const SADDLE_BROWN: Color = Color::rgb(59.0 / 255.0, 26.0 / 255.0, 14.0 / 255.0);

pub const WHITE_PAWN_SPRITE: &str = "128px/w_pawn_png_shadow_128px.png";
pub const BLACK_PAWN_SPRITE: &str = "128px/b_pawn_png_shadow_128px.png";
pub const WHITE_KNIGHT_SPRITE: &str = "128px/w_knight_png_shadow_128px.png";
pub const BLACK_KNIGHT_SPRITE: &str = "128px/b_knight_png_shadow_128px.png";
pub const WHITE_BISHOP_SPRITE: &str = "128px/w_bishop_png_shadow_128px.png";
pub const BLACK_BISHOP_SPRITE: &str = "128px/b_bishop_png_shadow_128px.png";
pub const WHITE_ROOK_SPRITE: &str = "128px/w_rook_png_shadow_128px.png";
pub const BLACK_ROOK_SPRITE: &str = "128px/b_rook_png_shadow_128px.png";
pub const WHITE_QUEEN_SPRITE: &str = "128px/w_queen_png_shadow_128px.png";
pub const BLACK_QUEEN_SPRITE: &str = "128px/b_queen_png_shadow_128px.png";
pub const WHITE_KING_SPRITE: &str = "128px/w_king_png_shadow_128px.png";
pub const BLACK_KING_SPRITE: &str = "128px/b_king_png_shadow_128px.png";

#[derive(Resource, Default)]
pub struct GameTextures {
    pub white_pawn: Handle<Image>,
    pub black_pawn: Handle<Image>,
    pub white_knight: Handle<Image>,
    pub black_knight: Handle<Image>,
    pub white_bishop: Handle<Image>,
    pub black_bishop: Handle<Image>,
    pub white_rook: Handle<Image>,
    pub black_rook: Handle<Image>,
    pub white_queen: Handle<Image>,
    pub black_queen: Handle<Image>,
    pub white_king: Handle<Image>,
    pub black_king: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct Turn {
    pub white: bool,
}

/*pub fn change_color_touching_buttons(
    //mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &Coordinates),
        (Changed<Interaction>, With<Button>),
    >,
    mut color_query: Query<(&mut BackgroundColor, &Coordinates), With<Button>>,
) {
    for (interaction, coordinates) in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            let curr_coordinates = coordinates.clone();

            // first reset all colors to white or black
            for (mut color, coordinates) in &mut color_query {
                if (coordinates.x + coordinates.y) % 2 == 0 {
                    *color = WHITE_BUTTON.into();
                } else {
                    *color = BLACK_BUTTON.into();
                }
            }

            for (mut color, coordinates) in &mut color_query {
                if (coordinates.x == curr_coordinates.x + 1
                    || coordinates.x == curr_coordinates.x - 1)
                    && (coordinates.y == curr_coordinates.y)
                    || (coordinates.y == curr_coordinates.y + 1
                        || coordinates.y == curr_coordinates.y - 1)
                        && (coordinates.x == curr_coordinates.x)
                {
                    *color = RED_BUTTON.into();
                }
            }
        }
    }
}

pub fn print_touching_buttons(
    //mut commands: Commands,
    button_query: Query<(Entity, &Coordinates), With<Button>>,
    interaction_query: Query<&Interaction, Changed<Interaction>>,
) {
    for (entity, coordinates) in button_query.iter() {
        if let Ok(interaction) = interaction_query.get(entity) {
            if *interaction == Interaction::Clicked {
                println!("Clicked button at ({}, {})", coordinates.x, coordinates.y);
                let curr_coordinates = coordinates.clone();
                for (_, coordinates) in button_query.iter() {
                    if (coordinates.x == curr_coordinates.x + 1
                        || coordinates.x == curr_coordinates.x - 1)
                        && (coordinates.y == curr_coordinates.y)
                        || (coordinates.y == curr_coordinates.y + 1
                            || coordinates.y == curr_coordinates.y - 1)
                            && (coordinates.x == curr_coordinates.x)
                    {
                        println!(
                            "Button at ({}, {}) touches clicked button",
                            coordinates.x, coordinates.y
                        );
                    }
                }
            }
        }
    }
}*/
