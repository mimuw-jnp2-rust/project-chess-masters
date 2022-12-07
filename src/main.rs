// enum representing chess pieces with position
use crate::chess_pieces::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(10.0, TimerMode::Repeating)))
            .add_startup_system(add_pieces)
            .add_system(greet_pieces);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("board.png"),
        ..default()
    });
}

fn main() {
    let king = new_piece("King", 0, 0);
    print_piece(&king);
    let queen = new_piece("Queen", 2, 0);
    print_piece(&queen);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Chess!".to_string(),
                width: 800.,
                height: 800.,
                ..default()
            },
            ..default()
        }))
        .add_plugin(HelloPlugin)
        .add_startup_system(setup)
        .run();
}
