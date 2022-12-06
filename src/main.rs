// enum representing chess pieces with position
#[derive(Debug)]
enum Piece {
    King { x: i32, y: i32 },
    Queen { x: i32, y: i32 },
    Rook { x: i32, y: i32 },
    Bishop { x: i32, y: i32 },
    Knight { x: i32, y: i32 },
    Pawn { x: i32, y: i32 },
}

// piece constructor
fn new_piece(piece: &str, x: i32, y: i32) -> Piece {
    match piece {
        "King" => Piece::King { x, y },
        "Queen" => Piece::Queen { x, y },
        "Rook" => Piece::Rook { x, y },
        "Bishop" => Piece::Bishop { x, y },
        "Knight" => Piece::Knight { x, y },
        "Pawn" => Piece::Pawn { x, y },
        _ => panic!("Invalid piece"),
    }
}

// print piece
fn print_piece(piece: &Piece) {
    match piece {
        Piece::King { x, y } => println!("King at ({}, {})", x, y),
        Piece::Queen { x, y } => println!("Queen at ({}, {})", x, y),
        Piece::Rook { x, y } => println!("Rook at ({}, {})", x, y),
        Piece::Bishop { x, y } => println!("Bishop at ({}, {})", x, y),
        Piece::Knight { x, y } => println!("Knight at ({}, {})", x, y),
        Piece::Pawn { x, y } => println!("Pawn at ({}, {})", x, y),
    }
}

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
