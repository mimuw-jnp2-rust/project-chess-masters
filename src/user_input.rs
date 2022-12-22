use crate::coordinates::{mouse_pos_to_coordinates, Coordinates};
use crate::field::Field;
use crate::moves::*;
use crate::ui::GameTextures;
use crate::*;
use bevy::input::{mouse::*, ButtonState};

fn move_piece_sprite(mut transform: Mut<Transform>, from: Coordinates, to: Coordinates) {
    transform.translation.x += (to.x as f32 - from.x as f32) * FIELD_SIZE;
    transform.translation.y += (to.y as f32 - from.y as f32) * FIELD_SIZE;
}

fn handle_piece_move(
    commands: &mut Commands,
    game_state: &mut ResMut<GameState>,
    piece_query: &mut Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    field_query: &mut Query<(&mut Sprite, &mut Field)>,
    selected_entity: Entity,
    clicked_coords: Coordinates,
) {
    let query_item = piece_query.get_mut(selected_entity);
    let (_, transform, mut piece) = query_item.unwrap();

    let possible_moves = get_possible_moves(&piece, &game_state.board);
    if possible_moves.contains(&clicked_coords) {
        let old_field_id = game_state.board.get_field_entity(piece.coordinates);
        let old_field_query_item = field_query.get_mut(old_field_id.unwrap());
        let mut old_field = old_field_query_item.unwrap().1;
        old_field.piece = None;

        let new_field_id = game_state.board.get_field_entity(clicked_coords);
        let new_field_query_item = field_query.get_mut(new_field_id.unwrap());
        let mut new_field = new_field_query_item.unwrap().1;
        // despawn piece if there is one
        if let Some(piece) = &new_field.piece {
            commands.entity(piece.entity.unwrap()).despawn();
        }
        new_field.piece = Some(piece.clone());

        move_piece_sprite(transform, piece.coordinates, clicked_coords);

        game_state.white = !game_state.white;
        let _ = &game_state
            .board
            .move_piece(piece.coordinates, clicked_coords);
        piece.coordinates = clicked_coords;

        // game_state.board.print_board();
    }
}

fn handle_piece_choice(
    game_state: &mut ResMut<GameState>,
    game_textures: &Res<GameTextures>,
    query: &mut Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    entity: Entity,
    select: bool,
) {
    let query_item = query.get_mut(entity).unwrap();
    let query_item = (query_item.0, query_item.2);
    let (mut image, mut piece) = query_item;
    piece.border = select;
    *image = get_image(&piece, game_textures);
    if select {
        game_state.selected_entity = Some(entity);
    } else {
        game_state.selected_entity = None;
    }
}

fn unselect_piece(
    game_state: &mut ResMut<GameState>,
    game_textures: &Res<GameTextures>,
    query: &mut Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    entity: Entity,
) {
    handle_piece_choice(game_state, game_textures, query, entity, false);
}

fn select_piece(
    game_state: &mut ResMut<GameState>,
    game_textures: &Res<GameTextures>,
    query: &mut Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    entity: Entity,
) {
    handle_piece_choice(game_state, game_textures, query, entity, true);
}

fn handle_field_click(
    commands: &mut Commands,
    game_state: &mut ResMut<GameState>,
    game_textures: &Res<GameTextures>,
    clicked_coords: Coordinates,
    piece_query: &mut Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    field_query: &mut Query<(&mut Sprite, &mut Field)>,
) {
    if let Some(selected_id) = game_state.selected_entity {
        clear_board(game_state, game_textures, piece_query, field_query);
        let clicked_field = game_state.board.get_field(clicked_coords).unwrap();
        if clicked_field.piece.is_some()
            && (clicked_field.piece.clone().unwrap().piece_color == PieceColor::White)
                == game_state.white
        {
            if clicked_field.piece.clone().unwrap().entity.unwrap() == selected_id {
                clear_board(game_state, game_textures, piece_query, field_query);
                return;
            }
            // println!("Clicked on own piece");
            let piece = clicked_field.piece.clone().unwrap();
            let clicked_id = piece.entity.unwrap();
            select_piece(game_state, game_textures, piece_query, clicked_id);
        } else {
            handle_piece_move(
                commands,
                game_state,
                piece_query,
                field_query,
                selected_id,
                clicked_coords,
            );
        }
    } else {
        clear_board(game_state, game_textures, piece_query, field_query);
        let clicked_field = game_state.board.get_field(clicked_coords).unwrap();
        if let Some(piece) = clicked_field.piece.clone() {
            if (piece.piece_color == PieceColor::White) == game_state.white {
                let clicked_id = piece.entity.unwrap();
                select_piece(game_state, game_textures, piece_query, clicked_id);
            }
        }
    }
}

fn clear_board(
    game_state: &mut ResMut<GameState>,
    game_textures: &Res<GameTextures>,
    piece_query: &mut Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    field_query: &mut Query<(&mut Sprite, &mut Field)>,
) {
    for (mut sprite_field, field) in field_query.iter_mut() {
        match field.color {
            field::FieldColor::White => sprite_field.color = WHITE_FIELD,
            field::FieldColor::Black => sprite_field.color = BLACK_FIELD,
        };
    }

    if let Some(selected_id) = game_state.selected_entity {
        unselect_piece(game_state, game_textures, piece_query, selected_id);
    }
}

fn handle_user_input(
    mut commands: Commands,
    windows: Res<Windows>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut piece_query: Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    mut field_query: Query<(&mut Sprite, &mut Field)>,
    game_textures: Res<GameTextures>,
    mut game_state: ResMut<GameState>,
) {
    let window = windows.get_primary().unwrap();
    let (height, width) = (window.height(), window.width());

    for event in button_evr.iter() {
        if let ButtonState::Pressed = event.state {
            if event.button != MouseButton::Left {
                continue;
            }

            if let Some(pos) = window.cursor_position() {
                let clicked_coords = mouse_pos_to_coordinates(pos.x, pos.y, width, height);
                //println!("clicked_coords = {}", clicked_coords);

                if game_state.board.get_field(clicked_coords).is_some() {
                    handle_field_click(
                        &mut commands,
                        &mut game_state,
                        &game_textures,
                        clicked_coords,
                        &mut piece_query,
                        &mut field_query,
                    );
                } else {
                    println!("Opps, clicked outside the board");
                    clear_board(
                        &mut game_state,
                        &game_textures,
                        &mut piece_query,
                        &mut field_query,
                    );
                }
            }
        }
    }
}

fn reset_fields_to_default(field_query: &mut Query<(&mut Sprite, &Field)>) {
    for (mut sprite_field, field) in field_query.iter_mut() {
        match field.color {
            field::FieldColor::White => sprite_field.color = WHITE_FIELD,
            field::FieldColor::Black => sprite_field.color = BLACK_FIELD,
        }
    }
}

fn highlight_fields(
    piece: &Piece,
    field_query: &mut Query<(&mut Sprite, &Field)>,
    game_state: &Res<GameState>,
) {
    let possible_moves = get_possible_moves(piece, &game_state.board);
    for (mut sprite_field, field) in field_query.iter_mut() {
        if possible_moves.contains(&field.coordinates) {
            match field.color {
                field::FieldColor::White => sprite_field.color = LIGHT_GRAY,
                field::FieldColor::Black => sprite_field.color = DARK_GRAY,
            }
        }
    }
}

fn highlight_moves_on_click(
    windows: Res<Windows>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut field_query: Query<(&mut Sprite, &Field)>,
    game_state: Res<GameState>,
) {
    let window = windows.get_primary().unwrap();
    let (height, width) = (window.height(), window.width());

    for event in button_evr.iter() {
        if let ButtonState::Pressed = event.state {
            if event.button != MouseButton::Left {
                continue;
            }
            reset_fields_to_default(&mut field_query);

            if game_state.selected_entity.is_none() {
                return;
            }

            if let Some(pos) = window.cursor_position() {
                let clicked_coords = mouse_pos_to_coordinates(pos.x, pos.y, width, height);
                if let Some(clicked_field) = game_state.board.get_field(clicked_coords) {
                    if let Some(piece) = &clicked_field.piece {
                        if (piece.piece_color == PieceColor::White) == game_state.white {
                            highlight_fields(piece, &mut field_query, &game_state);
                        }
                    }
                }
            }
        }
    }
}

pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_user_input);
        app.add_system(highlight_moves_on_click.after(handle_user_input));
    }
}
