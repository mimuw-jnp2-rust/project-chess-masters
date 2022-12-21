use crate::board::*;
use crate::chess_pieces::*;
use crate::coordinates::{mouse_pos_to_coordinates, Coordinates};
use crate::field::Field;
use crate::moves::*;
use crate::*;
use bevy::ecs::system::SystemParam;
use bevy::input::{mouse::*, ButtonState};
use bevy::prelude::*;

type PieceQueryType<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Handle<Image>,
        &'static mut Transform,
        &'static mut Piece,
    ),
>;
type FieldQueryType<'w, 's> = Query<'w, 's, (&'w mut Sprite, &'w mut Field)>;
//mut field_query: Query<(&mut Sprite, &Field), With<Sprite>>,
//mut piece_query: Query<(&mut Handle<Image>, &mut Transform, &mut Piece), With<Sprite>>,
/*
struct input_handler<'a, static> {
    pub game_state: Res<'a, GameState>,
    pub game_textures: Res<'a, GameTextures>,
    pub button_evr: EventReader<'a, 'a, MouseButtonInput>,
    pub field_query: Query<'a, 'a, (&'a mut Sprite, &'a Field), With<Sprite>>,
    pub piece_query:
        Query<'a, 'a, (&'a mut Handle<Image>, &'a mut Transform, &'a mut Piece), With<Sprite>>,
}*/

struct QueriesPair<'a> {
    pub field_query: FieldQueryType<'w, 's>,
    pub piece_query: Query<'a, 'a, (&'a mut Handle<Image>, &'a mut Transform, &'a mut Piece)>,
}

impl<'w, 's> QueriesPair<'w, 's> {
    pub fn new(field_query: FieldQueryType<'w, 's>, piece_query: PieceQueryType<'w, 's>) -> Self {
        Self {
            field_query,
            piece_query,
        }
    }
}

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
                if game_state.selected_entity.is_some() {
                    println!("it is some!");
                    let selected_piece_id = game_state.selected_entity.unwrap();
                    let query_item = piece_query.get_mut(selected_piece_id);
                    let (_, mut image, mut transform, mut selected_piece) = query_item.unwrap();

                    if selected_piece.coordinates == clicked_coords {
                        game_state.selected_entity = None;
                        (*selected_piece).border = false;
                        *image = get_image(&selected_piece, &game_textures);
                    } else {
                        let possible_moves = get_possible_moves(&selected_piece, &game_state.board);
                        game_state.selected_entity = None;
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
                        game_state.selected_entity = None;
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
                            game_state.selected_entity = Some(entity);
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

fn handle_piece_move(
    game_state: &mut ResMut<GameState>,
    queries: &mut QueriesPair,
    selected_entity: Entity,
    clicked_coords: Coordinates,
) {
    let mut query_item = queries.piece_query.get_mut(selected_entity);
    let (mut image, mut transform, mut piece) = query_item.unwrap();

    let possible_moves = get_possible_moves(&piece, &game_state.board);
    if possible_moves.contains(&clicked_coords) {
        let old_field_id = game_state.board.get_field_entity(piece.coordinates);
        let old_field_query_item = queries.field_query.get_mut(old_field_id.unwrap());
        let mut old_field = old_field_query_item.unwrap().1;
        old_field.piece = None;

        let new_field_id = game_state.board.get_field_entity(clicked_coords);
        let new_field_query_item = queries.field_query.get_mut(new_field_id.unwrap());
        let mut new_field = new_field_query_item.unwrap().1;

        new_field.piece = Some(piece.clone());

        let translation = &mut transform.translation;
        translation.x += (clicked_coords.x as f32 - piece.coordinates.x as f32) * FIELD_SIZE;
        translation.y += (clicked_coords.y as f32 - piece.coordinates.y as f32) * FIELD_SIZE;

        piece.coordinates = clicked_coords;

        game_state.white = !game_state.white;
        let _ = &game_state
            .board
            .move_piece(piece.coordinates, clicked_coords);
    }
}

fn handle_piece_choice(
    game_state: &mut ResMut<GameState>,
    game_textures: &Res<GameTextures>,
    query: &mut PieceQueryType,
    enity: Entity,
    select: bool,
) {
    let mut query_item = query.get_mut(enity);
    let (mut image, _, mut piece) = query_item.unwrap();
    piece.border = select;
    *image = get_image(&piece, &game_textures);
    if select {
        game_state.selected_entity = Some(enity);
    } else {
        game_state.selected_entity = None;
    }
}

fn unselect_piece(
    game_state: &mut ResMut<GameState>,
    game_textures: &Res<GameTextures>,
    query: &mut PieceQueryType,
    enity: Entity,
) {
    handle_piece_choice(game_state, game_textures, query, enity, false);
}

fn select_piece(
    game_state: &mut ResMut<GameState>,
    game_textures: &Res<GameTextures>,
    query: &mut PieceQueryType,
    enity: Entity,
) {
    handle_piece_choice(game_state, game_textures, query, enity, true);
}

fn handle_field_click(
    game_state: &mut ResMut<GameState>,
    game_textures: &Res<GameTextures>,
    clicked_coords: Coordinates,
    queries: &mut QueriesPair,
) {
    let clicked_field = game_state.board.get_field(clicked_coords).unwrap();
    if let Some(mut piece) = clicked_field.piece.clone() {
        if (piece.piece_color == PieceColor::White) == game_state.white {
            let clicked_id = piece.entity.unwrap();
            if let Some(selected_id) = game_state.selected_entity {
                unselect_piece(
                    game_state,
                    game_textures,
                    &mut queries.piece_query,
                    selected_id,
                );

                if clicked_id != selected_id {
                    select_piece(
                        game_state,
                        game_textures,
                        &mut queries.piece_query,
                        clicked_id,
                    );
                }
            } else {
                select_piece(
                    game_state,
                    game_textures,
                    &mut queries.piece_query,
                    clicked_id,
                );
            }
        } else {
            if let Some(selected_id) = game_state.selected_entity {
                unselect_piece(
                    game_state,
                    game_textures,
                    &mut queries.piece_query,
                    selected_id,
                );
                handle_piece_move(game_state, queries, selected_id, clicked_coords);
            }
        }
    } else {
        // nie stoi figura... lub stoi enemy figura
        if let Some(selected_id) = game_state.selected_entity {
            unselect_piece(
                game_state,
                game_textures,
                &mut queries.piece_query,
                selected_id,
            );
            handle_piece_move(game_state, queries, selected_id, clicked_coords);
        }
    }
}

fn clear_board(
    game_state: &mut ResMut<GameState>,
    game_textures: &Res<GameTextures>,
    queries: &mut QueriesPair,
) {
    for (mut sprite_field, field) in queries.field_query.iter_mut() {
        match field.color {
            field::FieldColor::White => (*sprite_field).color = WHITE_FIELD,
            field::FieldColor::Black => (*sprite_field).color = BLACK_FIELD,
        };
    }

    game_state.selected_entity = None;

    for (mut image, _, mut piece) in queries.piece_query.iter_mut() {
        (*piece).border = false;
        *image = get_image(&piece, &game_textures);
    }
}

pub fn handle_user_input(
    windows: Res<Windows>,
    mut button_evr: EventReader<MouseButtonInput>,

    mut piece_query: Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    mut field_query: Query<(&mut Sprite, &mut Field)>,
    game_textures: Res<GameTextures>,
    mut game_state: ResMut<GameState>,
) {
    let window = windows.get_primary().unwrap();
    let height = window.height();
    let width = window.width();
    let mut queries = QueriesPair::new(field_query, piece_query);

    for event in button_evr.iter() {
        if let ButtonState::Pressed = event.state {
            if event.button != MouseButton::Left {
                continue;
            }

            if let Some(pos) = window.cursor_position() {
                let clicked_coords = mouse_pos_to_coordinates(pos.x, pos.y, width, height);
                println!("clicked_coords = {}", clicked_coords);

                if let Some(clicked_field) = game_state.board.get_field(clicked_coords) {
                    handle_field_click(
                        &mut game_state,
                        &game_textures,
                        clicked_coords,
                        &mut queries,
                    );
                } else {
                    clear_board(&mut game_state, &game_textures, &mut queries);
                }
            }
        }
    }
}
