use crate::chess_pieces::*;
use crate::field::*;
use crate::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, board_spawn_system);
    }
}

pub fn get_image(piece: &Piece, game_textures: &Res<GameTextures>) -> Handle<Image> {
    if piece.piece_color == PieceColor::White {
        match game_textures.white_images_map.get(&piece.piece_type) {
            Some(image_pair) => {
                if piece.border {
                    return image_pair.1.clone();
                } else {
                    return image_pair.0.clone();
                }
            }
            None => return game_textures.error_image.clone(),
        }
    } else {
        match game_textures.black_images_map.get(&piece.piece_type) {
            Some(image_pair) => {
                if piece.border {
                    return image_pair.1.clone();
                } else {
                    return image_pair.0.clone();
                }
            }
            None => return game_textures.error_image.clone(),
        }
    };
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
                            x: j as i32 + 1,
                            y: i as i32 + 1,
                        },
                        color: field_color,
                        piece: Some(Piece::new(
                            PieceType::Pawn,
                            PieceColor::Black,
                            Coordinates {
                                x: i as i32 + 1,
                                y: j as i32 + 1,
                            },
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
        match game_textures.white_images_map.get(&PieceType::Pawn) {
            Some(image_pair) => image_pair.0.clone(),
            None => game_textures.error_image.clone(),
        }
    } else {
        match game_textures.black_images_map.get(&PieceType::Pawn) {
            Some(image_pair) => image_pair.0.clone(),
            None => game_textures.error_image.clone(),
        }
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
        .insert(Piece::new(
            PieceType::Pawn,
            color,
            Coordinates {
                x: coordinates.x,
                y: coordinates.y,
            },
        ));
}

fn spawn_rook(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let texture = if color == PieceColor::White {
        match game_textures.white_images_map.get(&PieceType::Rook) {
            Some(image_pair) => image_pair.0.clone(),
            None => game_textures.error_image.clone(),
        }
    } else {
        match game_textures.black_images_map.get(&PieceType::Rook) {
            Some(image_pair) => image_pair.0.clone(),
            None => game_textures.error_image.clone(),
        }
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
        .insert(Piece::new(
            PieceType::Rook,
            color,
            Coordinates {
                x: coordinates.x,
                y: coordinates.y,
            },
        ));
}

fn spawn_knight(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let texture = if color == PieceColor::White {
        match game_textures.white_images_map.get(&PieceType::Knight) {
            Some(image_pair) => image_pair.0.clone(),
            None => game_textures.error_image.clone(),
        }
    } else {
        match game_textures.black_images_map.get(&PieceType::Knight) {
            Some(image_pair) => image_pair.0.clone(),
            None => game_textures.error_image.clone(),
        }
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
        .insert(Piece::new(
            PieceType::Knight,
            color,
            Coordinates {
                x: coordinates.x,
                y: coordinates.y,
            },
        ));
}

fn spawn_bishop(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let texture = if color == PieceColor::White {
        match game_textures.white_images_map.get(&PieceType::Bishop) {
            Some(image_pair) => image_pair.0.clone(),
            None => game_textures.error_image.clone(),
        }
    } else {
        match game_textures.black_images_map.get(&PieceType::Bishop) {
            Some(image_pair) => image_pair.0.clone(),
            None => game_textures.error_image.clone(),
        }
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
        .insert(Piece::new(
            PieceType::Bishop,
            color,
            Coordinates {
                x: coordinates.x,
                y: coordinates.y,
            },
        ));
}

fn spawn_queen(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let texture = if color == PieceColor::White {
        match game_textures.white_images_map.get(&PieceType::Queen) {
            Some(image_pair) => image_pair.0.clone(),
            None => game_textures.error_image.clone(),
        }
    } else {
        match game_textures.black_images_map.get(&PieceType::Queen) {
            Some(image_pair) => image_pair.0.clone(),
            None => game_textures.error_image.clone(),
        }
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
        .insert(Piece::new(
            PieceType::Queen,
            color,
            Coordinates {
                x: coordinates.x,
                y: coordinates.y,
            },
        ));
}

fn spawn_king(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let texture = if color == PieceColor::White {
        match game_textures.white_images_map.get(&PieceType::King) {
            Some(image_pair) => image_pair.0.clone(),
            None => game_textures.error_image.clone(),
        }
    } else {
        match game_textures.black_images_map.get(&PieceType::King) {
            Some(image_pair) => image_pair.0.clone(),
            None => game_textures.error_image.clone(),
        }
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
        .insert(Piece::new(
            PieceType::King,
            color,
            Coordinates {
                x: coordinates.x,
                y: coordinates.y,
            },
        ));
}
