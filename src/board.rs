use crate::chess_pieces::*;
use crate::field::*;
use crate::*;

pub struct Board {
    pub fields: Vec<Vec<Field>>,
}

fn starting_piece_from_coordinates(coordinates: Coordinates) -> Option<Piece> {
    let piece_color = if coordinates.y < 3 {
        PieceColor::White
    } else if coordinates.y > 6 {
        PieceColor::Black
    } else {
        return None;
    };

    let piece_type = if coordinates.y == 2 || coordinates.y == 7 {
        PieceType::Pawn
    } else if coordinates.x == 1 || coordinates.x == 8 {
        PieceType::Rook
    } else if coordinates.x == 2 || coordinates.x == 7 {
        PieceType::Knight
    } else if coordinates.x == 3 || coordinates.x == 6 {
        PieceType::Bishop
    } else if coordinates.x == 4 {
        PieceType::Queen
    } else {
        PieceType::King
    };
    Some(Piece {
        piece_type: piece_type,
        piece_color: piece_color,
        coordinates: coordinates,
        border: false,
    })
}

impl Board {
    pub fn new() -> Board {
        let mut fields: Vec<Vec<Field>> = Vec::new();
        for i in 0..BOARD_SIZE {
            let mut row: Vec<Field> = Vec::new();
            for j in 0..BOARD_SIZE {
                let coordinates = Coordinates {
                    x: j as i32 + 1,
                    y: i as i32 + 1,
                };
                let color = if (i + j) % 2 == 0 {
                    FieldColor::Black
                } else {
                    FieldColor::White
                };
                let piece = starting_piece_from_coordinates(coordinates);
                row.push(Field::new(coordinates, color, piece));
            }
            fields.push(row);
        }
        Board { fields: fields }
    }

    pub fn print_board(&self) {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                match &self.fields[i][j].piece {
                    Some(piece) => print!("{}", piece),
                    None => print!(" "),
                }
            }
            println!();
        }
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, board_spawn_system);
    }
}

pub fn get_image(piece: &Piece, game_textures: &Res<GameTextures>) -> Handle<Image> {
    let maybe_image;
    if piece.piece_color == PieceColor::White {
        maybe_image = game_textures.white_images_map.get(&piece.piece_type);
    } else {
        maybe_image = game_textures.black_images_map.get(&piece.piece_type);
    };
    match maybe_image {
        Some(image_pair) => {
            if piece.border {
                return image_pair.1.clone();
            } else {
                return image_pair.0.clone();
            }
        }
        None => return game_textures.error_image.clone(),
    }
}

pub fn spawn_piece(
    commands: &mut Commands,
    piece: Piece,
    image: Handle<Image>,
    on_window_coordinates: Vec2,
) {
    commands
        .spawn(SpriteBundle {
            texture: image,
            transform: Transform {
                translation: Vec3::new(on_window_coordinates.x, on_window_coordinates.y, 10.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(piece);
}

pub fn board_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut game_state: ResMut<GameState>,
) {
    let start_x = (-1.0) * ((FIELD_SIZE * BOARD_SIZE as f32) / 2.0 - (FIELD_SIZE / 2.0));
    let mut x = start_x;
    let mut y = (-1.0) * ((FIELD_SIZE * BOARD_SIZE as f32) / 2.0 - (FIELD_SIZE / 2.0));
    game_state.board = Board::new();
    let fields = &game_state.board.fields;

    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let sprite_color = if fields[i][j].color == FieldColor::White {
                WHITE_FIELD
            } else {
                BLACK_FIELD
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
                            color: sprite_color,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Field {
                        coordinates: Coordinates {
                            x: j as i32 + 1,
                            y: i as i32 + 1,
                        },
                        color: fields[i][j].color,
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
                            color: sprite_color,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Field {
                        coordinates: Coordinates {
                            x: j as i32 + 1,
                            y: i as i32 + 1,
                        },
                        color: fields[i][j].color,
                        piece: None,
                    });
            }

            let piece_color = if i < 2 {
                PieceColor::White
            } else {
                PieceColor::Black
            };

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
    let piece = Piece::new(
        PieceType::Pawn,
        color,
        Coordinates {
            x: coordinates.x,
            y: coordinates.y,
        },
    );
    let texture = get_image(&piece, game_textures);
    spawn_piece(commands, piece, texture, on_window_coordinates);
}

fn spawn_rook(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let piece = Piece::new(
        PieceType::Rook,
        color,
        Coordinates {
            x: coordinates.x,
            y: coordinates.y,
        },
    );
    let texture = get_image(&piece, game_textures);
    spawn_piece(commands, piece, texture, on_window_coordinates);
}

fn spawn_knight(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let piece = Piece::new(
        PieceType::Knight,
        color,
        Coordinates {
            x: coordinates.x,
            y: coordinates.y,
        },
    );
    let texture = get_image(&piece, game_textures);
    spawn_piece(commands, piece, texture, on_window_coordinates);
}

fn spawn_bishop(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let piece = Piece::new(
        PieceType::Bishop,
        color,
        Coordinates {
            x: coordinates.x,
            y: coordinates.y,
        },
    );
    let texture = get_image(&piece, game_textures);
    spawn_piece(commands, piece, texture, on_window_coordinates);
}

fn spawn_queen(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let piece = Piece::new(
        PieceType::Queen,
        color,
        Coordinates {
            x: coordinates.x,
            y: coordinates.y,
        },
    );
    let texture = get_image(&piece, game_textures);
    spawn_piece(commands, piece, texture, on_window_coordinates);
}

fn spawn_king(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    coordinates: Coordinates,
    color: PieceColor,
    on_window_coordinates: Vec2,
) {
    let piece = Piece::new(
        PieceType::King,
        color,
        Coordinates {
            x: coordinates.x,
            y: coordinates.y,
        },
    );
    let texture = get_image(&piece, game_textures);
    spawn_piece(commands, piece, texture, on_window_coordinates);
}
