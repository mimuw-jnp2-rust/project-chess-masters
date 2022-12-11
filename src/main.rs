use bevy::winit::WinitSettings;
use bevy::{ecs::query, prelude::*};
use chess_masters::*;
use chess_pieces::*;
use coordinates::*;
use field::*;

mod chess_pieces;
mod coordinates;
mod field;

/*fn add_pieces(mut commands: Commands) {
    commands.spawn(Piece::new("Pawn", 1, 1, PieceColor::Black));
    commands.spawn(Piece::new("King", 2, 1, PieceColor::White));
    commands.spawn(Piece::new("Bishop", 3, 1, PieceColor::Black));
}*/

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let margin_left = (WINDOW_WIDTH - (BOARD_SIZE as f32) * FIELD_SIZE) / 2.0;
    let margin_bottom = (WINDOW_HEIGHT - (BOARD_SIZE as f32) * FIELD_SIZE) / 2.0;

    for i in 0..8 {
        for j in 0..8 {
            let color = if (i + j) % 2 == 0 {
                WHITE_BUTTON
            } else {
                BLACK_BUTTON
            };

            commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(FIELD_SIZE), Val::Px(FIELD_SIZE)),
                            margin: UiRect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                left: Val::Px(margin_left + FIELD_SIZE * i as f32),
                                bottom: Val::Px(margin_bottom + FIELD_SIZE * j as f32),
                                ..default()
                            },
                            ..default()
                        },
                        background_color: color.into(),
                        ..default()
                    },
                    Coordinates { x: i + 1, y: j + 1 },
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                            ..default()
                        },
                        image: asset_server
                            .load("128px/b_bishop_png_shadow_128px.png")
                            .into(),
                        ..default()
                    });
                });

            // on each button we will spawn image of ronaldo.png with size 30px x 30px
            /*commands.spawn((
                ImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                        // center image
                        margin: UiRect::all(Val::Auto),
                        // move image up
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(margin_left + 30.0 + 50.0 * i as f32),
                            bottom: Val::Px(margin_bottom + 30.0 + 50.0 * j as f32),
                            ..default()
                        },
                        ..default()
                    },
                    image: asset_server
                        .load("128px/b_bishop_png_shadow_128px.png")
                        .into(),
                    ..default()
                },
                //Coordinates { x: i + 1, y: j + 1 },
            ));*/
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Chess!".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                ..default()
            },
            ..default()
        }))
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .add_system(change_color_touching_buttons)
        .add_system(print_touching_buttons)
        .run();
}

pub fn change_color_touching_buttons(
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
                for (entity, coordinates) in button_query.iter() {
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
}
