// enum representing chess pieces with position
use crate::chess_pieces::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn hello_world() {
    println!("hello world!");
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_pieces)
            .add_system(hello_world)
            .add_system(greet_pieces);
    }
}

fn add_pieces(mut commands: Commands) {
    commands.spawn(Piece::new("Pawn", 1, 1));
    commands.spawn(Piece::new("King", 2, 1));
    commands.spawn(Piece::new("Bishop", 3, 1));
}

fn greet_pieces(query: Query<&Piece>) {
    for piece in query.iter() {
        println!("hello {}!", piece.get_type());
    }
}

fn main() {
    let king = new_piece("King", 0, 0);
    print_piece(&king);
    let queen = new_piece("Queen", 2, 0);
    print_piece(&queen);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
