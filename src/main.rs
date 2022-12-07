// enum representing chess pieces with position
use bevy::prelude::*;
//use bevy::{sprite::MaterialMesh2dBundle};
use chess_pieces::*;

mod chess_pieces;

const BOARD_SIZE: u32 = 8;

pub struct HelloPlugin;

fn add_pieces(mut commands: Commands) {
    commands.spawn(Piece::new("Pawn", 1, 1, 'b'));
    commands.spawn(Piece::new("King", 2, 1, 'w'));
    commands.spawn(Piece::new("Bishop", 3, 1, 'b'));
}

fn spawn_square(commands: &mut Commands, size: f32, x: f32, y: f32, color: (f32, f32, f32)) {
    let (r, g, b) = color;
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(x, y, 1.0),
        sprite: Sprite {
            color: Color::rgb(r, g, b),
            custom_size: Some(Vec2::new(size, size)),
            ..default()
        },
        ..default()
    });
}

fn spawn_board(mut commands: Commands, width: u32, height: u32, field_size: f32) {
    let start_x = (-1.0) * ((field_size * BOARD_SIZE as f32) / 2.0 - (field_size / 2.0));
    let mut x = start_x;
    let mut y = start_x;
    let mut current_color: (f32, f32, f32);
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if (i + j) % 2 == 0 {
                current_color = (255.0, 255.0, 255.0);
            } else {
                current_color = (0.0, 0.0, 0.0);
            }
            spawn_square(&mut commands, field_size, x, y, current_color);
            x += field_size;
        }
        x = start_x;
        y += field_size;
    }
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
    /*commands.spawn(SpriteBundle {
        texture: asset_server.load("board.png"),
        ..default()
    });*/
    spawn_board(commands, 800, 800, 50.0);
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
