use bevy::prelude::*;
use bevy::winit::WinitSettings;
use chess_masters::board::*;
use chess_masters::chess_pieces::*;
use chess_masters::user_input::UserInputPlugin;
use chess_masters::*;
use std::collections::HashMap;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        board: Board::empty(),
        selected_entity: None,
    });
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
        .add_plugin(UserInputPlugin)
        .insert_resource(ClearColor(SADDLE_BROWN))
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .run();
}
