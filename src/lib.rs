use bevy::prelude::*;
use board::*;
use chess_pieces::*;
use coordinates::Coordinates;
use std::collections::HashMap;
use ui::GameTextures;

pub mod audio;
pub mod board;
pub mod bot;
pub mod chess_pieces;
pub mod coordinates;
pub mod field;
pub mod game_over;
pub mod main_menu;
pub mod moves;
pub mod ui;
pub mod user_input;

//pub const WINDOW_WIDTH: f32 = 800.0;
//pub const WINDOW_HEIGHT: f32 = 800.0;
pub const FIELD_SIZE: f32 = 100.0;
pub const BOARD_SIZE: usize = 8;
pub const PLAY_AGAIN_BUTTON_WIDTH: f32 = 150.0;
pub const PLAY_AGAIN_BUTTON_HEIGHT: f32 = 50.0;

pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const WHITE_FIELD: Color = Color::rgb(0.9, 0.9, 0.9);
pub const BLACK_FIELD: Color = Color::rgb(0.1, 0.1, 0.1);
pub const DARK_GRAY: Color = Color::rgb(80.0 / 255.0, 80.0 / 255.0, 80.0 / 255.0);
pub const LIGHT_GRAY: Color = Color::rgb(150.0 / 255.0, 150.0 / 255.0, 150.0 / 255.0);
pub const RED: Color = Color::rgb(0.9, 0.1, 0.1);
pub const SADDLE_BROWN: Color = Color::rgb(59.0 / 255.0, 26.0 / 255.0, 14.0 / 255.0);
pub const YELLOW: Color = Color::rgb(218.0 / 255.0, 160.0 / 255.0, 0.0 / 255.0);
pub const GREEN: Color = Color::rgb(47.0 / 255.0, 168.0 / 255.0, 43.0 / 255.0);

pub const FRIEND_TEXT: &str = "Play with your friend";
pub const BOT_TEXT: &str = "Play with bot";

pub const WHITE_PAWN_SPRITE: &str = "128px/w_pawn_png_shadow_128px.png";
pub const BORDERED_WHITE_PAWN_SPRITE: &str = "128px/w_pawn_bordered.png";
pub const BLACK_PAWN_SPRITE: &str = "128px/b_pawn_png_shadow_128px.png";
pub const BORDERED_BLACK_PAWN_SPRITE: &str = "128px/b_pawn_bordered.png";
pub const WHITE_KNIGHT_SPRITE: &str = "128px/w_knight_png_shadow_128px.png";
pub const BORDERED_WHITE_KNIGHT_SPRITE: &str = "128px/w_knight_bordered.png";
pub const BLACK_KNIGHT_SPRITE: &str = "128px/b_knight_png_shadow_128px.png";
pub const BORDERED_BLACK_KNIGHT_SPRITE: &str = "128px/b_knight_bordered.png";
pub const WHITE_BISHOP_SPRITE: &str = "128px/w_bishop_png_shadow_128px.png";
pub const BORDERED_WHITE_BISHOP_SPRITE: &str = "128px/w_bishop_bordered.png";
pub const BLACK_BISHOP_SPRITE: &str = "128px/b_bishop_png_shadow_128px.png";
pub const BORDERED_BLACK_BISHOP_SPRITE: &str = "128px/b_bishop_bordered.png";
pub const WHITE_ROOK_SPRITE: &str = "128px/w_rook_png_shadow_128px.png";
pub const BORDERED_WHITE_ROOK_SPRITE: &str = "128px/w_rook_bordered.png";
pub const BLACK_ROOK_SPRITE: &str = "128px/b_rook_png_shadow_128px.png";
pub const BORDERED_BLACK_ROOK_SPRITE: &str = "128px/b_rook_bordered.png";
pub const WHITE_QUEEN_SPRITE: &str = "128px/w_queen_png_shadow_128px.png";
pub const BORDERED_WHITE_QUEEN_SPRITE: &str = "128px/w_queen_bordered.png";
pub const BLACK_QUEEN_SPRITE: &str = "128px/b_queen_png_shadow_128px.png";
pub const BORDERED_BLACK_QUEEN_SPRITE: &str = "128px/b_queen_bordered.png";
pub const WHITE_KING_SPRITE: &str = "128px/w_king_png_shadow_128px.png";
pub const BORDERED_WHITE_KING_SPRITE: &str = "128px/w_king_bordered.png";
pub const BLACK_KING_SPRITE: &str = "128px/b_king_png_shadow_128px.png";
pub const BORDERED_BLACK_KING_SPRITE: &str = "128px/b_king_bordered.png";
pub const RONALDO: &str = "ronaldo.png";
pub const BACKGROUNG: &str = "background.png";

#[derive(Resource)]
pub struct GameState {
    pub board: Board,
    pub white: bool,
    pub selected_entity: Option<Entity>,
    pub winner: Option<PieceColor>,
    pub bot_turn: bool,
    pub vs_bot: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GlobalState {
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]

pub enum WhoseTurn {
    Bot,
    Player,
}

pub fn get_image(piece: &Piece, game_textures: &Res<GameTextures>) -> Handle<Image> {
    let maybe_image = if piece.piece_color == PieceColor::White {
        game_textures.white_images_map.get(&piece.piece_type)
    } else {
        game_textures.black_images_map.get(&piece.piece_type)
    };
    match maybe_image {
        Some(image_pair) => {
            if piece.border {
                image_pair.1.clone()
            } else {
                image_pair.0.clone()
            }
        }
        None => game_textures.error_image.clone(),
    }
}
