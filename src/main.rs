use bevy::{ecs::query, prelude::*};
//use bevy::{sprite::MaterialMesh2dBundle};
use board::*;
use chess_pieces::*;

mod board;
mod chess_pieces;

pub struct HelloPlugin;

fn add_pieces(mut commands: Commands) {
    commands.spawn(Piece::new("Pawn", 1, 1, PieceColor::Black));
    commands.spawn(Piece::new("King", 2, 1, PieceColor::White));
    commands.spawn(Piece::new("Bishop", 3, 1, PieceColor::Black));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_pieces(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Piece>) {
    if timer.0.tick(time.delta()).just_finished() {
        for piece in query.iter() {
            println!("hello {}!", &piece);
        }
    }
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(4.0, TimerMode::Repeating)))
            .add_startup_system(add_pieces)
            .add_system(greet_pieces);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    spawn_board(commands, 50.0);
}

// for each square sprite change its color to red or blue
fn change_color(time: Res<Time>, mut timer: ResMut<GreetTimer>, mut query: Query<&mut Sprite>) {
    for mut sprite in query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            sprite.color = Color::rgb(1.0, 0.0, 0.0);
        }
    }
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
        .add_system(change_color)
        .run();
}
