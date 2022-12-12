use crate::chess_pieces::*;
use crate::field::*;
use crate::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, board_spawn_system);
    }
}

pub fn board_spawn_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    let start_x = (-1.0) * ((FIELD_SIZE * BOARD_SIZE as f32) / 2.0 - (FIELD_SIZE / 2.0));
    let mut x = start_x;
    println!("x: {}", x);
    let mut y = (-1.0) * ((FIELD_SIZE * BOARD_SIZE as f32) / 2.0 - (FIELD_SIZE / 2.0));

    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let color = if (i + j) % 2 == 0 {
                WHITE_BUTTON
            } else {
                BLACK_BUTTON
            };

            let field_color = if color == WHITE_BUTTON {
                FieldColor::White
            } else {
                FieldColor::Black
            };

            if i == 1 || i == 6 {
                commands
                    .spawn(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(x as f32, y as f32, 0.0),
                            ..default()
                        },
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(FIELD_SIZE, FIELD_SIZE)),
                            color: color,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Field {
                        coordinates: Coordinates {
                            x: i as i32 + 1,
                            y: j as i32 + 1,
                        },
                        color: field_color,
                        piece: Some(PieceType::new("Pawn", i + 1, j + 1, PieceColor::Black)),
                    });
            } else {
                commands
                    .spawn(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(x as f32, y as f32, 0.0),
                            ..default()
                        },
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(FIELD_SIZE, FIELD_SIZE)),
                            color: color,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Field {
                        coordinates: Coordinates {
                            x: i as i32 + 1,
                            y: j as i32 + 1,
                        },
                        color: field_color,
                        piece: None,
                    });
            }

            if i == 1 || i == 6 {
                commands
                    .spawn(SpriteBundle {
                        texture: game_textures.white_pawn.clone(),
                        transform: Transform {
                            translation: Vec3::new(x as f32, y as f32, 10.0),
                            scale: Vec3::new(0.3, 0.3, 1.0),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(components::Piece {
                        piece_type: PieceType::new("Pawn", i + 1, j + 1, PieceColor::White),
                    });
            }
            x += FIELD_SIZE;
        }
        x = start_x;
        y += FIELD_SIZE;
    }
}
