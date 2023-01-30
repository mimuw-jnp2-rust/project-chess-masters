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
    state: &mut ResMut<State<GlobalState>>,
    whose_turn: &mut ResMut<State<WhoseTurn>>,
) {
    game_state.white = !game_state.white; // end of move

    // check for winner or draw
    let mut color = PieceColor::White;
    let mut maybe_winner = PieceColor::Black;
    if !game_state.white {
        color = PieceColor::Black;
        maybe_winner = PieceColor::White;
    }

    if game_state.vs_bot {
        if game_state.bot_turn {
            whose_turn
                .set(WhoseTurn::Player)
                .expect("Unexpected error while setting state");
        } else {
            whose_turn
                .set(WhoseTurn::Bot)
                .expect("Unexpected error while setting state");
        }
        game_state.bot_turn = !game_state.bot_turn;
    }

    if game_state.board.no_possible_moves(color) {
        if game_state.board.king_in_danger(color) {
            println!("Game over!"); // change state :/
            game_state.winner = Some(maybe_winner);
        } else {
            println!("Draw!");
        }
        state
            .set(GlobalState::GameOver)
            .expect("Unexpected error while setting state");
    }
}

fn promote_pawn(
    image: &mut Handle<Image>,
    piece: &mut Piece,
    game_textures: &Res<GameTextures>,
    white: bool,
) {
    piece.piece_type = PieceType::Queen;
    if white {
        *image = game_textures
            .white_images_map
            .get(&PieceType::Queen)
            .expect("Error in getting image")
            .0
            .clone()
    } else {
        *image = game_textures
            .black_images_map
            .get(&PieceType::Queen)
            .expect("Error in getting image")
            .0
            .clone()
    }
}

fn handle_pawn_promotion(
    image: &mut Handle<Image>,
    piece: &mut Piece,
    game_textures: &Res<GameTextures>,
    clicked_coords: Coordinates,
) {
    if (piece.piece_type == PieceType::Pawn { moved: true }) {
        if piece.piece_color == PieceColor::White && clicked_coords.y == 8 {
            promote_pawn(image, piece, game_textures, true);
        } else if piece.piece_color == PieceColor::Black && clicked_coords.y == 1 {
            promote_pawn(image, piece, game_textures, false);
        }
    }
}

fn check_if_piece_already_moved(piece: &mut Piece) {
    if (piece.piece_type == PieceType::Pawn { moved: false }) {
        piece.piece_type = PieceType::Pawn { moved: true };
    }

    if (piece.piece_type == PieceType::King { moved: false }) {
        piece.piece_type = PieceType::King { moved: true };
    }

    if (piece.piece_type == PieceType::Rook { moved: false }) {
        piece.piece_type = PieceType::Rook { moved: true };
    }
}

fn move_piece_on_board(
    game_state: &mut ResMut<GameState>,
    piece: &mut Piece,
    clicked_coords: Coordinates,
) {
    let _ = &game_state
        .board
        .move_piece(piece.coordinates, clicked_coords);
    piece.coordinates = clicked_coords;
}

pub fn handle_piece_move(
    commands: &mut Commands,
    game_state: &mut ResMut<GameState>,
    piece_query: &mut Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    selected_entity: Entity,
    clicked_coords: Coordinates,
    state: &mut ResMut<State<GlobalState>>,
    game_textures: &Res<GameTextures>,
    whose_turn: &mut ResMut<State<WhoseTurn>>,
) {
    let query_item = piece_query.get_mut(selected_entity);
    let (mut image, transform, mut piece) = query_item.expect("Error in getting piece");

    let new_field = game_state
        .board
        .get_field(clicked_coords)
        .expect("Error in getting field");
    if let Some(new_piece) = &new_field.piece {
        if let Some(entity) = new_piece.entity {
            commands.entity(entity).despawn();
        }
    }

    check_if_piece_already_moved(&mut piece);

    handle_pawn_promotion(&mut image, &mut piece, game_textures, clicked_coords);

    move_piece_sprite(transform, piece.coordinates, clicked_coords);

    move_piece_on_board(game_state, &mut piece, clicked_coords);

    if !game_state.castling {
        handle_end_of_move(game_state, state, whose_turn);
    }
}

fn handle_piece_choice(
    game_state: &mut ResMut<GameState>,
    game_textures: &Res<GameTextures>,
    query: &mut Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    entity: Entity,
    select: bool,
) {
    // panics?
    let query_item = query.get_mut(entity).expect("Error in getting query item");
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

fn handle_castling(
    commands: &mut Commands,
    game_state: &mut ResMut<GameState>,
    piece_query: &mut Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    king_entity: Entity,
    rook_entity: Entity,
    state: &mut ResMut<State<GlobalState>>,
    game_textures: &Res<GameTextures>,
    whose_turn: &mut ResMut<State<WhoseTurn>>,
) {
    game_state.castling = true;
    let rook_piece = piece_query
        .get_mut(rook_entity)
        .expect("Error in getting piece")
        .2;
    let rook_coords = rook_piece.coordinates;

    let king_piece = piece_query
        .get_mut(king_entity)
        .expect("Error in getting piece")
        .2;
    let king_coords = king_piece.coordinates;

    let new_king_coords: Coordinates;
    let new_rook_coords: Coordinates;

    if (king_coords.x - rook_coords.x).abs() == 4 {
        new_king_coords = Coordinates {
            x: king_coords.x - 2,
            y: king_coords.y,
        };
        new_rook_coords = Coordinates {
            x: rook_coords.x + 3,
            y: rook_coords.y,
        };
    } else {
        new_king_coords = Coordinates {
            x: king_coords.x + 2,
            y: king_coords.y,
        };
        new_rook_coords = Coordinates {
            x: rook_coords.x - 2,
            y: rook_coords.y,
        };
    }
    handle_piece_move(
        commands,
        game_state,
        piece_query,
        king_entity,
        new_king_coords,
        state,
        game_textures,
        whose_turn,
    );
    game_state.castling = false;
    handle_piece_move(
        commands,
        game_state,
        piece_query,
        rook_entity,
        new_rook_coords,
        state,
        game_textures,
        whose_turn,
    );
    //game_state.white = !game_state.white; - czemu to tu
}

fn handle_field_click(
    commands: &mut Commands,
    game_state: &mut ResMut<GameState>,
    game_textures: &Res<GameTextures>,
    clicked_coords: Coordinates,
    piece_query: &mut Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    field_query: &mut Query<(&mut Sprite, &mut Field)>,
    state: &mut ResMut<State<GlobalState>>,
    whose_turn: &mut ResMut<State<WhoseTurn>>,
) {
    if let Some(selected_id) = game_state.selected_entity {
        clear_board(game_state, game_textures, piece_query, field_query);

        let clicked_field = game_state
            .board
            .get_field(clicked_coords)
            .expect("Error in getting field");

        // if clicked field has piece and it's the same color as the player who's turn it is
        if clicked_field.piece.is_some()
            && (clicked_field
                .piece
                .clone()
                .expect("Error in cloning piece")
                .piece_color
                == PieceColor::White)
                == game_state.white
        {
            let clicked_piece = clicked_field
                .piece
                .as_ref()
                .expect("Error in getting field ref");
            let clicked_id = clicked_piece.entity.expect("Error in getting piece");
            let selected_piece = piece_query
                .get_mut(selected_id)
                .expect("Error in getting piece")
                .2;

            if clicked_id == selected_id {
                clear_board(game_state, game_textures, piece_query, field_query);
                return;
            }

            // check if castling is possible
            if clicked_piece.piece_type == (PieceType::Rook { moved: false })
                && selected_piece.piece_type == (PieceType::King { moved: false })
            {
                let possible_moves = get_possible_moves(&selected_piece, &game_state.board, true);
                if possible_moves.contains(&clicked_coords) {
                    handle_castling(
                        commands,
                        game_state,
                        piece_query,
                        selected_id,
                        clicked_id,
                        state,
                        game_textures,
                        whose_turn,
                    );
                } else {
                    select_piece(game_state, game_textures, piece_query, clicked_id);
                }
            } else {
                select_piece(game_state, game_textures, piece_query, clicked_id);
            }
        } else {
            let piece = piece_query
                .get_mut(selected_id)
                .expect("Error in getting piece")
                .2;

            let possible_moves = get_possible_moves(&piece, &game_state.board, true);
            if possible_moves.contains(&clicked_coords) {
                handle_piece_move(
                    commands,
                    game_state,
                    piece_query,
                    selected_id,
                    clicked_coords,
                    state,
                    game_textures,
                    whose_turn,
                );
            }
        }
    } else {
        clear_board(game_state, game_textures, piece_query, field_query);
        let clicked_field = game_state
            .board
            .get_field(clicked_coords)
            .expect("Error in getting field");

        if let Some(piece) = clicked_field.piece.as_ref() {
            if (piece.piece_color == PieceColor::White) == game_state.white {
                let clicked_id = piece.entity.expect("Error in getting piece entity");
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
    mut whose_turn: ResMut<State<WhoseTurn>>,
) {
    let window = windows.get_primary().expect("Error in getting windows");
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
                        &mut whose_turn,
                    );
                } else {
                    // clicked outside of the board
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
    let window = windows.get_primary().expect("Error in getting windows");
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
        state
            .push(GlobalState::Paused)
            .expect("Error in setting state");
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
