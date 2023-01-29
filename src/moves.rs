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

fn check_castling(
    color: PieceColor,
    board: &Board,
    rook_coords: &Coordinates,
    need_safe: &Coordinates,
    need_empty: &Vec<(i32, i32)>,
) -> bool {
    if let Some(some_piece) = board.get_piece(*rook_coords) {
        if some_piece.piece_type == (PieceType::Rook { moved: false }) {
            for pair in need_empty {
                let coords = Coordinates {
                    x: pair.0,
                    y: pair.1,
                };
                if let Some(_) = board.get_piece(coords) {
                    return false; // piece between rook and king
                }
            }

            if board.king_in_danger(color) {
                return false;
            }
            if board.field_in_danger(color, *need_safe) {
                return false;
            }
        }
    }
    return true;
}

// castling representation: the field where rook is standing
// king and rook hasn't moved
// there are no pieces in between
// king is not in danger
// king is not in danger on the passing field
fn check_for_castlings(board: &Board, color: PieceColor, result: &mut Vec<Coordinates>) {
    if color == PieceColor::Black {
        let rook_coords1 = Coordinates { x: 1, y: 8 };
        let rook_coords2 = Coordinates { x: 8, y: 8 };

        let need_safe1 = Coordinates { x: 4, y: 8 };
        let need_empty1 = vec![(2, 8), (3, 8), (4, 8)];

        let need_safe2 = Coordinates { x: 6, y: 8 };
        let need_empty2 = vec![(6, 8), (7, 8)];

        if check_castling(color, board, &rook_coords1, &need_safe1, &need_empty1) {
            result.push(rook_coords1);
        }
        if check_castling(color, board, &rook_coords2, &need_safe2, &need_empty2) {
            result.push(rook_coords2);
        }
    } else {
        let rook_coords1 = Coordinates { x: 1, y: 1 };
        let rook_coords2 = Coordinates { x: 8, y: 1 };

        let need_safe1 = Coordinates { x: 4, y: 1 };
        let need_empty1 = vec![(2, 1), (3, 1), (4, 1)];

        let need_safe2 = Coordinates { x: 6, y: 1 };
        let need_empty2 = vec![(6, 1), (7, 1)];

        if check_castling(color, board, &rook_coords1, &need_safe1, &need_empty1) {
            result.push(rook_coords1);
        }
        if check_castling(color, board, &rook_coords2, &need_safe2, &need_empty2) {
            result.push(rook_coords2);
        }
    }
}

fn add_if_empty(dest: Coordinates, board: &Board, result: &mut Vec<Coordinates>) -> bool {
    if let Some(field) = board.get_field(dest) {
        if field.piece.is_none() {
            result.push(dest);
            return true;
        }
        return false;
    }
    return false;
}

fn add_forward_moves(piece: &Piece, board: &Board, result: &mut Vec<Coordinates>, dir: i32) {
    let dest = Coordinates {
        x: piece.coordinates.x,
        y: piece.coordinates.y + dir,
    };
    if add_if_empty(dest, board, result) {
        if let PieceType::Pawn { moved } = piece.piece_type {
            if !moved {
                let dest2 = dest + Coordinates { x: 0, y: dir };
                add_if_empty(dest2, board, result);
            }
        }
    }
}

fn get_pawn_moves(piece: &Piece, board: &Board) -> Vec<Coordinates> {
    let mut result: Vec<Coordinates> = Vec::new();
    let directions = if piece.piece_color == PieceColor::White {
        add_forward_moves(piece, board, &mut result, 1);
        vec![(-1, 1), (1, 1)]
    } else {
        add_forward_moves(piece, board, &mut result, -1);
        vec![(-1, -1), (1, -1)]
    };

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

fn get_king_moves(piece: &Piece, board: &Board, check_castling: bool) -> Vec<Coordinates> {
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
    result = result
        .into_iter()
        .filter(|c| ok_king_knight_move(c, board, piece.piece_color))
        .collect();

    if check_castling {
        if piece.piece_type == (PieceType::King { moved: false }) {
            check_for_castlings(board, piece.piece_color, &mut result);
        }
    }
    return result;
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

pub fn get_possible_moves(piece: &Piece, board: &Board, filter_check: bool) -> Vec<Coordinates> {
    let result;
    match piece.piece_type {
        PieceType::King { .. } => result = get_king_moves(piece, board, filter_check),
        PieceType::Queen { .. } => result = get_queen_moves(piece, board),
        PieceType::Rook { .. } => result = get_rook_moves(piece, board),
        PieceType::Bishop { .. } => result = get_bishop_moves(piece, board),
        PieceType::Knight { .. } => result = get_knight_moves(piece, board),
        PieceType::Pawn { .. } => result = get_pawn_moves(piece, board),
    }
    if filter_check {
        result
            .into_iter()
            .filter(|c| !board.is_check_after_move(&piece.coordinates, c, piece.piece_color))
            .collect()
    } else {
        result
    }
}
