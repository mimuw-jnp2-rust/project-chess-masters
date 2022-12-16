use crate::chess_pieces::PieceType;
use crate::coordinates::*;
use crate::BOARD_SIZE;

pub fn get_pawn_moves(coordinates: &Coordinates) -> Vec<Coordinates> {
    let result = vec![*coordinates + Coordinates { x: 0, y: 1 }];
    result.into_iter().filter(|c| c.in_board_bounds()).collect()
}

// skoczek :3
pub fn get_knight_moves(from: &Coordinates) -> Vec<Coordinates> {
    let mut result = Vec::new();
    let mul = vec![(-1, 1), (1, -1), (1, 1), (-1, -1)];

    let mul_iter = mul.iter();
    for pair in mul_iter {
        result.push(Coordinates {
            x: from.x + 1 * pair.0,
            y: from.y + 2 * pair.1,
        });
        result.push(Coordinates {
            x: from.x + 2 * pair.0,
            y: from.y + 1 * pair.1,
        });
    }
    result.into_iter().filter(|c| c.in_board_bounds()).collect()
}

pub fn get_king_moves(from: &Coordinates) -> Vec<Coordinates> {
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
            x: from.x + pair.0,
            y: from.y + pair.1,
        });
    }
    result.into_iter().filter(|c| c.in_board_bounds()).collect()
}

pub fn get_queen_moves(from: &Coordinates) -> Vec<Coordinates> {
    let mut result = get_rook_moves(from);
    result.append(&mut get_bishop_moves(from));
    result
}

// wieza
pub fn get_rook_moves(from: &Coordinates) -> Vec<Coordinates> {
    let mut result = Vec::new();
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let dirs_iter = directions.iter();
    for pair in dirs_iter {
        for i in 1..BOARD_SIZE {
            result.push(Coordinates {
                x: from.x + pair.0 * i as i32,
                y: from.y + pair.1 * i as i32,
            })
        }
    }
    result.into_iter().filter(|c| c.in_board_bounds()).collect()
}

pub fn get_bishop_moves(from: &Coordinates) -> Vec<Coordinates> {
    let mut result = Vec::new();
    let directions = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];

    let dirs_iter = directions.iter();
    for pair in dirs_iter {
        for i in 1..BOARD_SIZE {
            result.push(Coordinates {
                x: from.x + pair.0 * i as i32,
                y: from.y + pair.1 * i as i32,
            })
        }
    }
    result.into_iter().filter(|c| c.in_board_bounds()).collect()
}

pub fn get_possible_moves(piece: PieceType, coordinates: &Coordinates) -> Vec<Coordinates> {
    match piece {
        PieceType::King { .. } => return get_king_moves(coordinates),
        PieceType::Queen { .. } => return get_queen_moves(coordinates),
        PieceType::Rook { .. } => return get_rook_moves(coordinates),
        PieceType::Bishop { .. } => return get_bishop_moves(coordinates),
        PieceType::Knight { .. } => return get_knight_moves(coordinates),
        PieceType::Pawn { .. } => return get_pawn_moves(coordinates),
    }
}
