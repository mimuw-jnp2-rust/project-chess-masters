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
            let color = if (i + j) % 2 != 0 {
                WHITE_BUTTON
            } else {
                BLACK_BUTTON
            };

            let field_color = if color == WHITE_BUTTON {
                FieldColor::White
            } else {
                FieldColor::Black
            };

            let piece_color = if i == 0 || i == 1 {
                PieceColor::White
            } else {
                PieceColor::Black
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
                        piece: Some(PieceType::new(
                            "Pawn",
                            i as i32 + 1,
                            j as i32 + 1,
                            PieceColor::Black,
                        )),
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
                            x: j as i32 + 1,
                            y: i as i32 + 1,
                        },
                        color: field_color,
                        piece: None,
                    });
            }

            if i == 1 || i == 6 {
                spawn_pawn(
                    &mut commands,
                    &game_textures,
                    Coordinates {
                        x: j as i32 + 1,
                        y: i as i32 + 1,
                    },
                    piece_color,
                    Vec2 { x: (x), y: (y) },
                )
            }

            if i == 0 || i == 7 {
                if j == 0 || j == 7 {
                    spawn_rook(
                        &mut commands,
                        &game_textures,
                        Coordinates {
                            x: j as i32 + 1,
                            y: i as i32 + 1,
                        },
                        piece_color,
                        Vec2 { x: (x), y: (y) },
                    )
                } else if j == 1 || j == 6 {
                    spawn_knight(
                        &mut commands,
                        &game_textures,
                        Coordinates {
                            x: j as i32 + 1,
                            y: i as i32 + 1,
                        },
                        piece_color,
                        Vec2 { x: (x), y: (y) },
                    )
                } else if j == 2 || j == 5 {
                    spawn_bishop(
                        &mut commands,
                        &game_textures,
                        Coordinates {
                            x: j as i32 + 1,
                            y: i as i32 + 1,
                        },
                        piece_color,
                        Vec2 { x: (x), y: (y) },
                    )
                } else if j == 3 {
                    spawn_queen(
                        &mut commands,
                        &game_textures,
                        Coordinates {
                            x: j as i32 + 1,
                            y: i as i32 + 1,
                        },
                        piece_color,
                        Vec2 { x: (x), y: (y) },
                    )
                } else if j == 4 {
                    spawn_king(
                        &mut commands,
                        &game_textures,
                        Coordinates {
                            x: j as i32 + 1,
                            y: i as i32 + 1,
                        },
                        piece_color,
                        Vec2 { x: (x), y: (y) },
                    )
                }
            }
            x += FIELD_SIZE;
        }
        x = start_x;
        y += FIELD_SIZE;
    }
}

fn spawn_pawn(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let texture = if color == PieceColor::White {
        game_textures.white_pawn.clone()
    } else {
        game_textures.black_pawn.clone()
    };
    commands
        .spawn(SpriteBundle {
            texture: texture,
            transform: Transform {
                translation: Vec3::new(on_window_coordinates.x, on_window_coordinates.y, 10.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(components::Piece {
            piece_type: PieceType::new("Pawn", coordinates.x, coordinates.y, color),
        });
}

fn spawn_rook(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let texture = if color == PieceColor::White {
        game_textures.white_rook.clone()
    } else {
        game_textures.black_rook.clone()
    };
    commands
        .spawn(SpriteBundle {
            texture: texture,
            transform: Transform {
                translation: Vec3::new(on_window_coordinates.x, on_window_coordinates.y, 10.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(components::Piece {
            piece_type: PieceType::new("Rook", coordinates.x, coordinates.y, color),
        });
}

fn spawn_knight(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let texture = if color == PieceColor::White {
        game_textures.white_knight.clone()
    } else {
        game_textures.black_knight.clone()
    };
    commands
        .spawn(SpriteBundle {
            texture: texture,
            transform: Transform {
                translation: Vec3::new(on_window_coordinates.x, on_window_coordinates.y, 10.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(components::Piece {
            piece_type: PieceType::new("Knight", coordinates.x, coordinates.y, color),
        });
}

fn spawn_bishop(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let texture = if color == PieceColor::White {
        game_textures.white_bishop.clone()
    } else {
        game_textures.black_bishop.clone()
    };
    commands
        .spawn(SpriteBundle {
            texture: texture,
            transform: Transform {
                translation: Vec3::new(on_window_coordinates.x, on_window_coordinates.y, 10.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(components::Piece {
            piece_type: PieceType::new("Bishop", coordinates.x, coordinates.y, color),
        });
}

fn spawn_queen(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let texture = if color == PieceColor::White {
        game_textures.white_queen.clone()
    } else {
        game_textures.black_queen.clone()
    };
    commands
        .spawn(SpriteBundle {
            texture: texture,
            transform: Transform {
                translation: Vec3::new(on_window_coordinates.x, on_window_coordinates.y, 10.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(components::Piece {
            piece_type: PieceType::new("Queen", coordinates.x, coordinates.y, color),
        });
}

fn spawn_king(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let texture = if color == PieceColor::White {
        game_textures.white_king.clone()
    } else {
        game_textures.black_king.clone()
    };
    commands
        .spawn(SpriteBundle {
            texture: texture,
            transform: Transform {
                translation: Vec3::new(on_window_coordinates.x, on_window_coordinates.y, 10.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(components::Piece {
            piece_type: PieceType::new("King", coordinates.x, coordinates.y, color),
        });
}
