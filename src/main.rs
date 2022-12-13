use bevy::input::{mouse::*, ButtonState};
use bevy::prelude::*;
use bevy::winit::WinitSettings;
// use chess_masters::chess_pieces::*;
use chess_masters::components::Piece;
use chess_masters::coordinates::mouse_pos_to_coordinates;
// use chess_masters::field::Field;
use chess_masters::*;
// use chess_pieces::*;
// use coordinates::*;
// use field::*;

// mod board;
//mod chess_pieces;
//mod coordinates;
//mod field;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>, /* , mut windows: ResMut<Windows>*/
) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(GameTextures {
        white_pawn: asset_server.load(WHITE_PAWN_SPRITE),
        black_pawn: asset_server.load(BLACK_PAWN_SPRITE),
        white_knight: asset_server.load(WHITE_KNIGHT_SPRITE),
        black_knight: asset_server.load(BLACK_KNIGHT_SPRITE),
        white_bishop: asset_server.load(WHITE_BISHOP_SPRITE),
        black_bishop: asset_server.load(BLACK_BISHOP_SPRITE),
        white_rook: asset_server.load(WHITE_ROOK_SPRITE),
        black_rook: asset_server.load(BLACK_ROOK_SPRITE),
        white_queen: asset_server.load(WHITE_QUEEN_SPRITE),
        black_queen: asset_server.load(BLACK_QUEEN_SPRITE),
        white_king: asset_server.load(WHITE_KING_SPRITE),
        black_king: asset_server.load(BLACK_KING_SPRITE),
    });

    commands.insert_resource(Turn { white: true });

    /*let window = windows.get_primary_mut().unwrap();
    let (width, height) = window.physical_size();*/
}

fn piece_movement_system(mut query: Query<(&mut Transform, &Piece), With<Sprite>>) {
    for (mut transform, piece) in query.iter_mut() {
        let (x, y) = piece.piece_type.get_coordinates();
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
    mut query: Query<(&mut Handle<Image>, &Piece), With<Sprite>>,
    game_textures: Res<GameTextures>,
) {
    let window = windows.get_primary().unwrap();
    // print window size

    for event in button_evr.iter() {
        if let ButtonState::Pressed = event.state {
            if event.button != MouseButton::Left {
                continue;
            }
            let position = window.cursor_position();
            if let Some(pos) = position {
                let clicked_coords = mouse_pos_to_coordinates(pos.x, pos.y);
                println!(
                    "Mouse button pressed: {:?} at {}",
                    event.button, &clicked_coords
                );
                for (mut image, piece) in query.iter_mut() {
                    let (x, y) = piece.piece_type.get_coordinates();
                    if x == clicked_coords.x && y == clicked_coords.y {
                        *image = game_textures.white_king.clone();
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
