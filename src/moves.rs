use crate::board::*;
use crate::chess_pieces::*;
use crate::coordinates::*;

fn ok_king_knight_move(coords: &Coordinates, board: &Board, color: PieceColor) -> bool {
    if let Some(field) = board.get_field(*coords) {
        if let Some(some_piece) = &field.piece {
            some_piece.piece_color != color
        } else {
            true
        }
    } else {
        false
    }
}

fn get_pawn_moves(piece: &Piece, board: &Board) -> Vec<Coordinates> {
    let directions;
    let dest;
    let mut result: Vec<Coordinates> = Vec::new();
    if piece.piece_color == PieceColor::White {
        dest = Coordinates {
            x: piece.coordinates.x,
            y: piece.coordinates.y + 1,
        };
        directions = vec![(-1, -1), (1, -1)];
    } else {
        dest = Coordinates {
            x: piece.coordinates.x,
            y: piece.coordinates.y - 1,
        };
        directions = vec![(-1, 1), (1, 1)];
    }
    if let Some(field) = board.get_field(dest) {
        if field.piece.is_none() {
            result.push(dest);
        }
    }

    let dirs_iter = directions.iter();
    for pair in dirs_iter {
        let dest = piece.coordinates
            + Coordinates {
                x: pair.0,
                y: pair.1,
            };
        if let Some(field) = board.get_field(dest) {
            if let Some(some_piece) = &field.piece {
                if some_piece.piece_color != piece.piece_color {
                    result.push(dest);
                }
            }
        }
    }
    result
}

fn get_knight_moves(piece: &Piece, board: &Board) -> Vec<Coordinates> {
    let mut result = Vec::new();
    let mul = vec![(-1, 1), (1, -1), (1, 1), (-1, -1)];

    let mul_iter = mul.iter();
    for pair in mul_iter {
        result.push(Coordinates {
            x: piece.coordinates.x + pair.0,
            y: piece.coordinates.y + 2 * pair.1,
        });
        result.push(Coordinates {
            x: piece.coordinates.x + 2 * pair.0,
            y: piece.coordinates.y + pair.1,
        });
    }
    result
        .into_iter()
        .filter(|c| ok_king_knight_move(c, board, piece.piece_color))
        .collect()
}

fn get_king_moves(piece: &Piece, board: &Board) -> Vec<Coordinates> {
    let mut result = Vec::new();
    let directions = vec![
        (-1, 1),
        (1, -1),
        (1, 1),
        (-1, -1),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ];

    let dirs_iter = directions.iter();
    for pair in dirs_iter {
        result.push(Coordinates {
            x: piece.coordinates.x + pair.0,
            y: piece.coordinates.y + pair.1,
        });
    }
    result
        .into_iter()
        .filter(|c| ok_king_knight_move(c, board, piece.piece_color))
        .collect()
}

fn get_queen_moves(piece: &Piece, board: &Board) -> Vec<Coordinates> {
    let mut result = get_rook_moves(piece, board);
    result.append(&mut get_bishop_moves(piece, board));
    result
}

fn get_rook_bishop_moves(
    piece: &Piece,
    board: &Board,
    directions: Vec<(i32, i32)>,
) -> Vec<Coordinates> {
    let mut result = Vec::new();
    let dirs_iter = directions.iter();
    for pair in dirs_iter {
        let mut coords = Coordinates {
            x: piece.coordinates.x + pair.0,
            y: piece.coordinates.y + pair.1,
        };
        while coords.in_board_bounds() {
            if let Some(field) = board.get_field(coords) {
                if let Some(some_piece) = &field.piece {
                    if some_piece.piece_color != piece.piece_color {
                        result.push(coords);
                    }
                    break;
                }
            }
            result.push(coords);
            coords = Coordinates {
                x: coords.x + pair.0,
                y: coords.y + pair.1,
            };
        }
    }
    result
}

fn get_rook_moves(piece: &Piece, board: &Board) -> Vec<Coordinates> {
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    get_rook_bishop_moves(piece, board, directions)
}

fn get_bishop_moves(piece: &Piece, board: &Board) -> Vec<Coordinates> {
    let directions = vec![(1, 1), (-1, 1), (1, -1), (-1, -1)];
    get_rook_bishop_moves(piece, board, directions)
}

pub fn get_possible_moves(piece: &Piece, board: &Board) -> Vec<Coordinates> {
    match piece.piece_type {
        PieceType::King { .. } => get_king_moves(piece, board),
        PieceType::Queen { .. } => get_queen_moves(piece, board),
        PieceType::Rook { .. } => get_rook_moves(piece, board),
        PieceType::Bishop { .. } => get_bishop_moves(piece, board),
        PieceType::Knight { .. } => get_knight_moves(piece, board),
        PieceType::Pawn { .. } => get_pawn_moves(piece, board),
    }
}
