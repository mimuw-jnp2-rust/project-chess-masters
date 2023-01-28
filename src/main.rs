use bevy::prelude::*;
use bevy::window::WindowMode::BorderlessFullscreen;
use chess_masters::board::*;
use chess_masters::game_over::*;
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
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Chess!".to_string(),
                //mode: BorderlessFullscreen,
                height: WINDOW_HEIGHT,
                width: WINDOW_WIDTH,
                ..default()
            },
            ..default()
        }))
        .add_state(GlobalState::MainMenu)
        .add_plugin(GameOverPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(UserInputPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(UserInterfacePlugin)
        .insert_resource(ClearColor(SADDLE_BROWN))
        //.insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .run();
}
