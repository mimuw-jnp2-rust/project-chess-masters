use crate::coordinates::{mouse_pos_to_coordinates, Coordinates};
use crate::field::Field;
use crate::moves::*;
use crate::ui::GameTextures;
use crate::*;
use bevy::input::{mouse::*, ButtonState};
use bevy_kira_audio::AudioControl;

fn move_piece_sprite(mut transform: Mut<Transform>, from: Coordinates, to: Coordinates) {
    transform.translation.x += (to.x as f32 - from.x as f32) * FIELD_SIZE;
    transform.translation.y += (to.y as f32 - from.y as f32) * FIELD_SIZE;
}

fn handle_end_of_move(
    game_state: &mut ResMut<GameState>,
    piece: &mut Piece,
    state: &mut ResMut<State<GlobalState>>,
    clicked_coords: Coordinates,
) {
    game_state.white = !game_state.white; // end of move
    let _ = &game_state
        .board
        .move_piece(piece.coordinates, clicked_coords);
    piece.coordinates = clicked_coords;

    // check for winner or draw
    let mut color = PieceColor::White;
    let mut maybe_winner = PieceColor::Black;
    if !game_state.white {
        color = PieceColor::Black;
        maybe_winner = PieceColor::White;
    }

    if game_state.board.no_possible_moves(color) {
        if game_state.board.king_in_danger(color) {
            println!("Game over!"); // change state :/
            game_state.winner = Some(maybe_winner);
        } else {
            println!("Draw!");
        }
        state.set(GlobalState::GameOver).unwrap();
    }
}

fn handle_piece_move(
    commands: &mut Commands,
    game_state: &mut ResMut<GameState>,
    piece_query: &mut Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    field_query: &mut Query<(&mut Sprite, &mut Field)>,
    selected_entity: Entity,
    clicked_coords: Coordinates,
    state: &mut ResMut<State<GlobalState>>,
    game_textures: &Res<GameTextures>,
) {
    let query_item = piece_query.get_mut(selected_entity);
    let (mut image, transform, mut piece) = query_item.unwrap();

    let possible_moves = get_possible_moves(&piece, &game_state.board, true);

    if possible_moves.contains(&clicked_coords) {
        let old_field_id = game_state.board.get_field_entity(piece.coordinates);
        let old_field_query_item = field_query.get_mut(old_field_id.unwrap());
        let mut old_field = old_field_query_item.unwrap().1;
        old_field.piece = None;

        let new_field_id = game_state.board.get_field_entity(clicked_coords);
        let new_field_query_item = field_query.get_mut(new_field_id.unwrap());
        let mut new_field = new_field_query_item.unwrap().1;
        // despawn piece if there is one
        if let Some(new_piece) = &new_field.piece {
            commands.entity(new_piece.entity.unwrap()).despawn();
        }
        new_field.piece = Some(piece.clone());

        if (piece.piece_type == PieceType::Pawn { moved: false }) {
            piece.piece_type = PieceType::Pawn { moved: true };
        }
        // if piece is a pawn and it is on the last row, promote it
        if (piece.piece_type == PieceType::Pawn { moved: true }) {
            if piece.piece_color == PieceColor::White && clicked_coords.y == 8 {
                println!("Promoting white pawn!");
                piece.piece_type = PieceType::Queen;
                *image = game_textures
                    .white_images_map
                    .get(&PieceType::Queen)
                    .unwrap()
                    .0
                    .clone()
            } else if piece.piece_color == PieceColor::Black && clicked_coords.y == 1 {
                println!("Promoting black pawn!");
                piece.piece_type = PieceType::Queen;
                *image = game_textures
                    .black_images_map
                    .get(&PieceType::Queen)
                    .unwrap()
                    .0
                    .clone()
            }
        }

        move_piece_sprite(transform, piece.coordinates, clicked_coords);

        handle_end_of_move(game_state, &mut piece, state, clicked_coords);
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
    state: &mut ResMut<State<GlobalState>>,
) {
    if let Some(selected_id) = game_state.selected_entity {
        clear_board(game_state, game_textures, piece_query, field_query);

        let clicked_field = game_state.board.get_field(clicked_coords).unwrap();

        // if clicked field has piece and it's the same color as the player who's turn it is
        if clicked_field.piece.is_some()
            && (clicked_field.piece.clone().unwrap().piece_color == PieceColor::White)
                == game_state.white
        {
            let clicked_piece = clicked_field.piece.as_ref().unwrap();
            let clicked_id = clicked_piece.entity.unwrap();

            if clicked_id == selected_id {
                clear_board(game_state, game_textures, piece_query, field_query);
                return;
            }

            select_piece(game_state, game_textures, piece_query, clicked_id);
        } else {
            handle_piece_move(
                commands,
                game_state,
                piece_query,
                field_query,
                selected_id,
                clicked_coords,
                state,
                game_textures,
            );
        }
    } else {
        clear_board(game_state, game_textures, piece_query, field_query);
        let clicked_field = game_state.board.get_field(clicked_coords).unwrap();

        if let Some(piece) = clicked_field.piece.as_ref() {
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
    mut state: ResMut<State<GlobalState>>,
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

                if game_state.board.get_field(clicked_coords).is_some() {
                    handle_field_click(
                        &mut commands,
                        &mut game_state,
                        &game_textures,
                        clicked_coords,
                        &mut piece_query,
                        &mut field_query,
                        &mut state,
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
    let possible_moves = get_possible_moves(piece, &game_state.board, true);
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

fn pause_on_escape(
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<GlobalState>>,
    audio: Res<bevy_kira_audio::prelude::Audio>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        state.push(GlobalState::Paused).unwrap();
        audio.resume();
    }
}

pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GlobalState::InGame)
                .with_system(handle_user_input)
                .with_system(highlight_moves_on_click.after(handle_user_input))
                .with_system(pause_on_escape),
        );
    }
}
