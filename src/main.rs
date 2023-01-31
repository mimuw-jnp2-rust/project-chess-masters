use bevy::prelude::*;
use bevy::window::WindowMode::BorderlessFullscreen;
use chess_masters::audio::ChessAudioPlugin;
use chess_masters::board::{Board, BoardPlugin};
use chess_masters::bot::BotPlugin;
use chess_masters::game_over::GameOverPlugin;
use chess_masters::game_paused::GamePausedPlugin;
use chess_masters::main_menu::MainMenuPlugin;
use chess_masters::ui::{GameTextures, UserInterfacePlugin};
use chess_masters::user_input::UserInputPlugin;
use chess_masters::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(GameTextures::new(&asset_server));

    commands.insert_resource(GameState {
        white: true,
        board: Board::empty(),
        selected_entity: None,
        winner: None,
        bot_turn: false,
        vs_bot: true,
        castling: false,
    });
}

fn create_window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        title: "Chess!".to_string(),
        mode: BorderlessFullscreen,
        ..default()
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(SADDLE_BROWN))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: create_window_descriptor(),
            ..default()
        }))
        .add_state(GlobalState::MainMenu)
        .add_state(WhoseTurn::Player)
        .add_plugin(GameOverPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(UserInputPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(ChessAudioPlugin)
        .add_plugin(GamePausedPlugin)
        .add_plugin(UserInterfacePlugin)
        .add_plugin(BotPlugin)
        .add_startup_system(setup)
        .run();
}
