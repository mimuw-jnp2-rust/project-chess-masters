use bevy::input::{mouse::*, ButtonState};
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use chess_masters::board::get_image;
use chess_masters::chess_pieces::*;
use chess_masters::coordinates::mouse_pos_to_coordinates;
use chess_masters::field::Field;
use chess_masters::moves::*;
use std::collections::HashMap;
use std::hash::Hash;
// use chess_masters::field::Field;
use chess_masters::*;
// use chess_pieces::*;
// use coordinates::*;
// use field::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>, /* , mut windows: ResMut<Windows>*/
) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(GameTextures {
        white_images_map: HashMap::from([
            (
                PieceType::Pawn,
                (
                    asset_server.load(WHITE_PAWN_SPRITE),
                    asset_server.load(BORDERED_WHITE_PAWN_SPRITE),
                ),
            ),
            (
                PieceType::Bishop,
                (
                    asset_server.load(WHITE_BISHOP_SPRITE),
                    asset_server.load(BORDERED_WHITE_BISHOP_SPRITE),
                ),
            ),
            (
                PieceType::Rook,
                (
                    asset_server.load(WHITE_ROOK_SPRITE),
                    asset_server.load(BORDERED_WHITE_ROOK_SPRITE),
                ),
            ),
            (
                PieceType::King,
                (
                    asset_server.load(WHITE_KING_SPRITE),
                    asset_server.load(BORDERED_WHITE_KING_SPRITE),
                ),
            ),
            (
                PieceType::Queen,
                (
                    asset_server.load(WHITE_QUEEN_SPRITE),
                    asset_server.load(BORDERED_WHITE_QUEEN_SPRITE),
                ),
            ),
            (
                PieceType::Knight,
                (
                    asset_server.load(WHITE_KNIGHT_SPRITE),
                    asset_server.load(BORDERED_WHITE_KNIGHT_SPRITE),
                ),
            ),
        ]),
        black_images_map: HashMap::from([
            (
                PieceType::Pawn,
                (
                    asset_server.load(BLACK_PAWN_SPRITE),
                    asset_server.load(BORDERED_BLACK_PAWN_SPRITE),
                ),
            ),
            (
                PieceType::Bishop,
                (
                    asset_server.load(BLACK_BISHOP_SPRITE),
                    asset_server.load(BORDERED_BLACK_BISHOP_SPRITE),
                ),
            ),
            (
                PieceType::Rook,
                (
                    asset_server.load(BLACK_ROOK_SPRITE),
                    asset_server.load(BORDERED_BLACK_ROOK_SPRITE),
                ),
            ),
            (
                PieceType::King,
                (
                    asset_server.load(BLACK_KING_SPRITE),
                    asset_server.load(BORDERED_BLACK_KING_SPRITE),
                ),
            ),
            (
                PieceType::Queen,
                (
                    asset_server.load(BLACK_QUEEN_SPRITE),
                    asset_server.load(BORDERED_BLACK_QUEEN_SPRITE),
                ),
            ),
            (
                PieceType::Knight,
                (
                    asset_server.load(BLACK_KNIGHT_SPRITE),
                    asset_server.load(BORDERED_BLACK_KNIGHT_SPRITE),
                ),
            ),
        ]),
        error_image: asset_server.load(RONALDO),
        /*white_pawn: asset_server.load(WHITE_PAWN_SPRITE),
        black_pawn: asset_server.load(BLACK_PAWN_SPRITE),
        white_knight: asset_server.load(WHITE_KNIGHT_SPRITE),
        black_knight: asset_server.load(BLACK_KNIGHT_SPRITE),
        white_bishop: asset_server.load(WHITE_BISHOP_SPRITE),
        black_bishop: asset_server.load(BLACK_BISHOP_SPRITE),
        bordered_black_bishop: asset_server.load(BORDERED_BLACK_BISHOP_SPRITE),
        white_rook: asset_server.load(WHITE_ROOK_SPRITE),
        black_rook: asset_server.load(BLACK_ROOK_SPRITE),
        white_queen: asset_server.load(WHITE_QUEEN_SPRITE),
        black_queen: asset_server.load(BLACK_QUEEN_SPRITE),
        white_king: asset_server.load(WHITE_KING_SPRITE),
        black_king: asset_server.load(BLACK_KING_SPRITE),*/
    });

    commands.insert_resource(Turn { white: true });

    /*let window = windows.get_primary_mut().unwrap();
    let (width, height) = window.physical_size();*/
}

fn piece_movement_system(mut query: Query<(&mut Transform, &Piece), With<Sprite>>) {
    for (mut transform, piece) in query.iter_mut() {
        let (x, y) = (piece.coordinates.x, piece.coordinates.y);
        println!("x: {}, y: {}", x, y);
        if x == 2 && y == 1 {
            let translation = &mut transform.translation;
            translation.y += 1.0;
        }
    }
}

pub fn input_handling(
    windows: Res<Windows>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut piece_query: Query<(&mut Handle<Image>, &mut Piece), With<Sprite>>,
    mut field_query: Query<(&mut Sprite, &Field), With<Sprite>>,
    game_textures: Res<GameTextures>,
) {
    let window = windows.get_primary().unwrap();
    // print window size

    for event in button_evr.iter() {
        if let ButtonState::Pressed = event.state {
            if event.button != MouseButton::Left {
                continue;
            }
            for (mut sprite_field, field) in field_query.iter_mut() {
                match field.color {
                    field::FieldColor::White => (*sprite_field).color = WHITE_BUTTON,
                    field::FieldColor::Black => (*sprite_field).color = BLACK_BUTTON,
                };
            }

            let position = window.cursor_position();
            if let Some(pos) = position {
                let clicked_coords = mouse_pos_to_coordinates(pos.x, pos.y);
                println!(
                    "Mouse button pressed: {:?} at {}",
                    event.button, &clicked_coords
                );
                for (mut image, mut piece) in piece_query.iter_mut() {
                    let coords = piece.coordinates;
                    // Koncepcyjnie myślę, że chcielibyśmy po kliknięciu gdziekolwiek na planszy
                    // wyłączyć wszystkie podświetlenia, poza tymi co opisują ostatni ruch (jeśli był)
                    // tzn. pola skąd i dokąd ruszyła figura powinny się świecić.
                    if coords == clicked_coords {
                        /*if *image == game_textures.bordered_black_bishop {
                            *image = game_textures.black_bishop.clone();
                        } else {
                            *image = game_textures.bordered_black_bishop.clone();
                        }*/
                        if piece.border {
                            (*piece).border = false;
                            *image = get_image(&piece, &game_textures);
                        } else {
                            (*piece).border = true;
                            *image = get_image(&piece, &game_textures);
                        }
                    }

                    let possible_moves = get_possible_moves(piece.piece_type, &clicked_coords);
                    if piece.coordinates == clicked_coords {
                        for (mut sprite_field, field) in field_query.iter_mut() {
                            if possible_moves.contains(&field.coordinates) {
                                println!("possible coords = {}", &field.coordinates);
                                match field.color {
                                    field::FieldColor::White => (*sprite_field).color = LIGHT_GRAY,
                                    field::FieldColor::Black => (*sprite_field).color = DARK_GRAY,
                                }
                            }
                        }
                    }
                }
            }
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
        .add_plugin(board::BoardPlugin)
        .insert_resource(ClearColor(SADDLE_BROWN))
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        //.add_system(piece_movement_system)
        .add_system(input_handling)
        .run();
}
