use bevy::prelude::*;

#[derive(Component)]
struct ChessPiece {
    piece_type: String,
}

fn add_pieces(mut commands: Commands) {
    commands.spawn(ChessPiece {
        piece_type: String::from("pawn"),
    });
    commands.spawn(ChessPiece {
        piece_type: String::from("king"),
    });
    commands.spawn(ChessPiece {
        piece_type: String::from("bishop"),
    });
}

fn greet_pieces(query: Query<&ChessPiece>) {
    for piece in query.iter() {
        println!("hello {}!", piece.piece_type);
    }
}

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::new()
        .add_startup_system(add_pieces)
        .add_system(hello_world)
        .add_system(greet_pieces)
        .run();
}
