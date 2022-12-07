// enum representing chess pieces with position
use bevy::prelude::*;
use chess_pieces::*;

mod chess_pieces;

pub struct HelloPlugin;

fn add_pieces(mut commands: Commands) {
    commands.spawn(Piece::new("Pawn", 1, 1, 'b'));
    commands.spawn(Piece::new("King", 2, 1, 'w'));
    commands.spawn(Piece::new("Bishop", 3, 1, 'b'));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_pieces(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Piece>) {
    if timer.0.tick(time.delta()).just_finished() {
        for piece in query.iter() {
            println!("hello {}!", piece.get_type());
        }
    }
}

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
