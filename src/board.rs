use crate::field::*;
use crate::moves::get_possible_moves;
use crate::*;

#[derive(Debug, Component, Clone, PartialEq, Eq)]
pub struct Board {
    pub fields: Vec<Vec<Field>>,
    pub white_king_pos: Coordinates,
    pub black_king_pos: Coordinates,
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
        PieceType::Pawn { moved: false }
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
    Some(Piece::new(piece_type, piece_color, coordinates))
}

impl Board {
    pub fn empty() -> Board {
        let fields: Vec<Vec<Field>> = Vec::new();
        Board {
            fields: fields,
            white_king_pos: Coordinates { x: 5, y: 1 },
            black_king_pos: Coordinates { x: 5, y: 8 },
        }
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

    pub fn get_field(&self, coordinates: Coordinates) -> Option<&Field> {
        if coordinates.x < 1 || coordinates.x > BOARD_SIZE as i32 {
            return None;
        }
        if coordinates.y < 1 || coordinates.y > BOARD_SIZE as i32 {
            return None;
        }
        Some(&self.fields[(coordinates.y - 1) as usize][(coordinates.x - 1) as usize])
    }

    pub fn get_field_mut(&mut self, coordinates: Coordinates) -> Option<&mut Field> {
        if coordinates.x < 1 || coordinates.x > BOARD_SIZE as i32 {
            return None;
        }
        if coordinates.y < 1 || coordinates.y > BOARD_SIZE as i32 {
            return None;
        }
        Some(&mut self.fields[(coordinates.y - 1) as usize][(coordinates.x - 1) as usize])
    }

    pub fn get_piece(&self, coordinates: Coordinates) -> Option<&Piece> {
        match self.get_field(coordinates) {
            Some(field) => match &field.piece {
                Some(piece) => Some(piece),
                None => None,
            },
            None => None,
        }
    }

    // iterates through all fields. If there is an enemy piece, check if
    // it could take the king
    pub fn king_in_danger(&self, my_color: PieceColor) -> bool {
        let king_position = &self.get_king_position(my_color);
        for row in &self.fields {
            for field in row {
                if let Some(some_piece) = &field.piece {
                    if some_piece.piece_color != my_color {
                        let possible_moves = get_possible_moves(some_piece, &self, false);
                        if possible_moves.contains(king_position) {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    pub fn no_possible_moves(&self, my_color: PieceColor) -> bool {
        let mut all_moves = Vec::new();
        for row in &self.fields {
            for field in row {
                if let Some(some_piece) = &field.piece {
                    if some_piece.piece_color == my_color {
                        let mut possible_moves = get_possible_moves(some_piece, &self, true);
                        all_moves.append(&mut possible_moves);
                    }
                }
            }
        }
        all_moves.is_empty()
    }

    fn get_king_position(&self, my_color: PieceColor) -> Coordinates {
        let mut king_position = self.white_king_pos;
        if my_color == PieceColor::Black {
            king_position = self.black_king_pos;
        }
        king_position
    }

    pub fn is_check_after_move(
        &self,
        from: &Coordinates,
        to: &Coordinates,
        my_color: PieceColor,
    ) -> bool {
        // clone fields here, don't see better option...

        let mut dummy_board: Board = self.clone();
        if !dummy_board.move_piece(from.clone(), to.clone()) {
            panic!("Something went wrong! Can't make a dummy move");
        }

        let my_king_in_danger: bool;
        match my_color {
            PieceColor::White => {
                my_king_in_danger = dummy_board.king_in_danger(PieceColor::White);
            }
            PieceColor::Black => {
                my_king_in_danger = dummy_board.king_in_danger(PieceColor::Black);
            }
        }
        my_king_in_danger
    }

    pub fn remove_piece(&mut self, coordinates: Coordinates) -> Option<Piece> {
        match self.get_field_mut(coordinates) {
            Some(field) => {
                let piece = field.piece.clone();
                field.piece = None;
                piece
            }
            None => None,
        }
    }

    pub fn move_piece(&mut self, from: Coordinates, to: Coordinates) -> bool {
        let mut white_king_moved = false;
        let mut black_king_moved = false;
        let mut ok = true;
        let piece = self.remove_piece(from);
        match piece {
            Some(piece) => {
                let field = self.get_field_mut(to);
                match field {
                    Some(field) => {
                        let mut piece = piece;
                        piece.coordinates = to;
                        if piece.piece_type == (PieceType::Pawn { moved: false }) {
                            piece.piece_type = PieceType::Pawn { moved: true };
                        }
                        if piece.piece_type == PieceType::King {
                            if piece.piece_color == PieceColor::White {
                                white_king_moved = true;
                            } else {
                                black_king_moved = true;
                            }
                        }
                        field.piece = Some(piece);
                    }
                    None => ok = false,
                }
            }
            None => ok = false,
        }
        if white_king_moved {
            self.white_king_pos = to;
        }
        if black_king_moved {
            self.black_king_pos = to;
        }
        ok
    }

    pub fn set_field_entity(&mut self, coordinates: Coordinates, entity: Entity) {
        if let Some(field) = self.get_field_mut(coordinates) {
            field.entity = entity;
        }
    }

    pub fn get_field_entity(&self, coordinates: Coordinates) -> Option<Entity> {
        self.get_field(coordinates).map(|field| field.entity)
    }

    pub fn get_piece_entity(&self, coordinates: Coordinates) -> Option<Entity> {
        match self.get_field(coordinates) {
            Some(field) => match &field.piece {
                Some(piece) => piece.entity,
                None => None,
            },
            None => None,
        }
    }

    pub fn set_piece_entity(&mut self, coordinates: Coordinates, entity: Entity) {
        if let Some(field) = self.get_field_mut(coordinates) {
            if let Some(piece) = &mut field.piece {
                piece.entity = Some(entity);
            }
        }
    }
}

fn spawn_piece(
    commands: &mut Commands,
    mut piece: &mut Piece,
    image: Handle<Image>,
    on_window_coordinates: Vec2,
) -> Entity {
    let id = commands
        .spawn(SpriteBundle {
            texture: image,
            transform: Transform {
                translation: Vec3::new(on_window_coordinates.x, on_window_coordinates.y, 10.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..default()
            },
            ..default()
        })
        .id();
    piece.entity = Some(id);
    commands.entity(id).insert(piece.clone());
    id
}

pub fn board_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut game_state: ResMut<GameState>,
) {
    println!("Spawning board");

    let start_x = (-1.0) * ((FIELD_SIZE * BOARD_SIZE as f32) / 2.0 - (FIELD_SIZE / 2.0));
    let mut x = start_x;
    let mut y = (-1.0) * ((FIELD_SIZE * BOARD_SIZE as f32) / 2.0 - (FIELD_SIZE / 2.0));

    let mut fields: Vec<Vec<Field>> = Vec::new();

    for i in 0..BOARD_SIZE {
        let mut row: Vec<Field> = Vec::new();
        for j in 0..BOARD_SIZE {
            let (field_color, sprite_color) = if (i + j) % 2 == 0 {
                (FieldColor::Black, BLACK_FIELD)
            } else {
                (FieldColor::White, WHITE_FIELD)
            };
            let coordinates = Coordinates {
                x: (j + 1) as i32,
                y: (i + 1) as i32,
            };

            let piece = starting_piece_from_coordinates(coordinates);

            let field_id = commands
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
                .id();

            let mut field = Field {
                entity: field_id,
                coordinates,
                color: field_color,
                piece: piece.clone(),
            };

            if let Some(mut piece) = piece {
                let image = get_image(&piece, &game_textures);
                spawn_piece(&mut commands, &mut piece, image, Vec2 { x, y });
                field.piece = Some(piece);
            }

            commands.entity(field_id).insert(field.clone());
            row.push(field);
            x += FIELD_SIZE;
        }
        fields.push(row);
        x = start_x;
        y += FIELD_SIZE;
    }
    game_state.board.fields = fields;
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        //app.add_startup_system_to_stage(StartupStage::PostStartup, board_spawn_system);
        app.add_system_set(
            SystemSet::on_enter(GlobalState::InGame).with_system(board_spawn_system),
        );
    }
}
