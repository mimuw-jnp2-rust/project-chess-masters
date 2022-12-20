use bevy::input::{mouse::*, ButtonState};
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use chess_masters::board::*;
use chess_masters::chess_pieces::*;
use chess_masters::coordinates::mouse_pos_to_coordinates;
use chess_masters::field::Field;
use chess_masters::moves::*;
use chess_masters::*;
use std::collections::HashMap;

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
    });

    commands.insert_resource(GameState {
        white: true,
        board: Board::new(),
        selected_piece: None,
    });

    /*let window = windows.get_primary_mut().unwrap();
    let (width, height) = window.physical_size();*/
}

#[allow(dead_code)]
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

pub fn clear_board() {}

pub fn border_piece_on_click(
    windows: Res<Windows>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut piece_query: Query<(Entity, &mut Handle<Image>, &mut Transform, &mut Piece), With<Sprite>>,
    mut field_query: Query<&mut Field, With<Sprite>>,
    game_textures: Res<GameTextures>,
    mut game_state: ResMut<GameState>,
) {
    let window = windows.get_primary().unwrap();
    // get current window size
    let height = window.height();
    let width = window.width();

    for event in button_evr.iter() {
        if let ButtonState::Pressed = event.state {
            if event.button != MouseButton::Left {
                continue;
            }

            let position = window.cursor_position();
            if let Some(pos) = position {
                let clicked_coords = mouse_pos_to_coordinates(pos.x, pos.y, width, height);
                if game_state.selected_piece.is_some() {
                    println!("it is some!");
                    let selected_piece_id = game_state.selected_piece.unwrap();
                    let query_item = piece_query.get_mut(selected_piece_id);
                    let (_, mut image, mut transform, mut selected_piece) = query_item.unwrap();

                    if selected_piece.coordinates == clicked_coords {
                        game_state.selected_piece = None;
                        (*selected_piece).border = false;
                        *image = get_image(&selected_piece, &game_textures);
                    } else {
                        let possible_moves = get_possible_moves(&selected_piece, &game_state.board);
                        game_state.selected_piece = None;
                        if possible_moves.contains(&clicked_coords) {
                            println!(
                                "it is possible to make move from {:?} to {:?}",
                                selected_piece.coordinates, clicked_coords
                            );
                            let _ = &game_state
                                .board
                                .move_piece(selected_piece.coordinates, clicked_coords);

                            //game_state.board.print_board();

                            let old_field_id = game_state
                                .board
                                .get_field_entity(selected_piece.coordinates);

                            let new_field_id = game_state.board.get_field_entity(clicked_coords);
                            let old_field_query_item = field_query.get_mut(old_field_id.unwrap());
                            let mut old_field = old_field_query_item.unwrap();
                            old_field.piece = None;
                            let new_field_query_item = field_query.get_mut(new_field_id.unwrap());

                            let mut new_field = new_field_query_item.unwrap();
                            new_field.piece = Some(selected_piece.clone());

                            game_state.white = !game_state.white;
                            let translation = &mut transform.translation;
                            translation.x += (clicked_coords.x as f32
                                - selected_piece.coordinates.x as f32)
                                * FIELD_SIZE;
                            translation.y += (clicked_coords.y as f32
                                - selected_piece.coordinates.y as f32)
                                * FIELD_SIZE;

                            selected_piece.coordinates = clicked_coords;
                        } else {
                            println!(
                                "it is not possible to make move from {:?} to {:?}",
                                selected_piece.coordinates, clicked_coords
                            );
                        }
                        game_state.selected_piece = None;
                        for (_, mut image, _, mut piece) in piece_query.iter_mut() {
                            (*piece).border = false;
                            *image = get_image(&piece, &game_textures);
                        }
                    }
                } else {
                    for (entity, mut image, _, mut piece) in piece_query.iter_mut() {
                        if piece.coordinates == clicked_coords {
                            (*piece).border = true;
                            *image = get_image(&piece, &game_textures);
                            game_state.selected_piece = Some(entity);
                        } else {
                            (*piece).border = false;
                            *image = get_image(&piece, &game_textures);
                        }
                    }
                }
            }
        }
    }
}

// podzielić na 2 funkcje: dla figur i dla pól osobne
pub fn highlight_moves_on_click(
    windows: Res<Windows>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut field_query: Query<(&mut Sprite, &Field), With<Sprite>>,
    game_state: Res<GameState>,
) {
    let window = windows.get_primary().unwrap();
    // get current window size
    let height = window.height();
    //print!("height i guess is {}", height);
    let width = window.width();

    for event in button_evr.iter() {
        if let ButtonState::Pressed = event.state {
            if event.button != MouseButton::Left {
                continue;
            }

            // reset pól
            for (mut sprite_field, field) in field_query.iter_mut() {
                match field.color {
                    field::FieldColor::White => (*sprite_field).color = WHITE_FIELD,
                    field::FieldColor::Black => (*sprite_field).color = BLACK_FIELD,
                };
            }

            if let Some(pos) = window.cursor_position() {
                let clicked_coords = mouse_pos_to_coordinates(pos.x, pos.y, width, height);
                println!("clicked_coords = {}", clicked_coords);

                if let Some(clicked_field) = &game_state.board.get_field(clicked_coords) {
                    println!("clicked_field = {:?}", clicked_field);
                    if let Some(piece) = &clicked_field.piece {
                        println!("piece = {:?}", piece);
                        game_state.board.print_board();
                        let possible_moves = get_possible_moves(&piece, &game_state.board);
                        println!("possible_moves = {:?}", possible_moves);
                        for (mut sprite_field, field) in field_query.iter_mut() {
                            if possible_moves.contains(&field.coordinates) {
                                //println!("possible coords = {}", &field.coordinates);
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

#[allow(dead_code)]
fn move_piece_on_click(
    windows: Res<Windows>,
    mut button_evr: EventReader<MouseButtonInput>,
    //mut field_query: Query<(&mut Sprite, &Field), With<Sprite>>,
    mut piece_query: Query<(&mut Transform, &mut Piece), With<Sprite>>,
    // game_state: Res<GameState>,
    // koordynaty podswietlonej figury
) {
    let window = windows.get_primary().unwrap();
    // get current window size
    let height = window.height();
    let width = window.width();

    for event in button_evr.iter() {
        if let ButtonState::Pressed = event.state {
            if event.button != MouseButton::Left {
                continue;
            }

            let position = window.cursor_position();
            if let Some(mut pos) = position {
                pos.x -= width as f32 / 2.0;
                pos.y -= height as f32 / 2.0;
                //let clicked_coords = mouse_pos_to_coordinates(pos.x, pos.y, width, height);
                // if kliknięte podświetlone pole to rusz na nie figurę
                for (mut transform, piece) in piece_query.iter_mut() {
                    if piece.border {
                        /*let possible_moves =
                        get_possible_moves(piece.piece_type, &piece.coordinates);*/
                        let translation = &mut transform.translation;
                        translation.x = pos.x;
                        translation.y = pos.y;
                        /*if possible_moves.contains(&clicked_coords) {
                            // print!("possible coords = {}", &clicked_coords);
                        } else {
                            println!("Nie można ruszyć figury na to pole");
                        }*/
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
        .add_system(highlight_moves_on_click)
        .add_system(border_piece_on_click)
        //.add_system(move_piece_on_click)
        .run();
}
