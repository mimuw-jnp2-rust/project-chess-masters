use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

use crate::*;

#[derive(Resource, Default)]
pub struct GameTextures {
    pub white_images_map: HashMap<PieceType, (Handle<Image>, Handle<Image>)>,
    pub black_images_map: HashMap<PieceType, (Handle<Image>, Handle<Image>)>,
    pub error_image: Handle<Image>,
}

impl GameTextures {
    pub fn new(asset_server: &Res<AssetServer>) -> GameTextures {
        GameTextures {
            white_images_map: HashMap::from([
                (
                    PieceType::Pawn { moved: false },
                    (
                        asset_server.load(WHITE_PAWN_SPRITE),
                        asset_server.load(BORDERED_WHITE_PAWN_SPRITE),
                    ),
                ),
                (
                    PieceType::Pawn { moved: true },
                    (
                        asset_server.load(WHITE_PAWN_SPRITE),
                        asset_server.load(BORDERED_WHITE_PAWN_SPRITE),
                    ),
                ),
                (
                    PieceType::Bishop,
                    (
                        asset_server.load(WHITE_BISHOP_SPRITE),
                        asset_server.load(BORDERED_WHITE_BISHOP_SPRITE),
                    ),
                ),
                (
                    PieceType::Rook,
                    (
                        asset_server.load(WHITE_ROOK_SPRITE),
                        asset_server.load(BORDERED_WHITE_ROOK_SPRITE),
                    ),
                ),
                (
                    PieceType::King,
                    (
                        asset_server.load(WHITE_KING_SPRITE),
                        asset_server.load(BORDERED_WHITE_KING_SPRITE),
                    ),
                ),
                (
                    PieceType::Queen,
                    (
                        asset_server.load(WHITE_QUEEN_SPRITE),
                        asset_server.load(BORDERED_WHITE_QUEEN_SPRITE),
                    ),
                ),
                (
                    PieceType::Knight,
                    (
                        asset_server.load(WHITE_KNIGHT_SPRITE),
                        asset_server.load(BORDERED_WHITE_KNIGHT_SPRITE),
                    ),
                ),
            ]),
            black_images_map: HashMap::from([
                (
                    PieceType::Pawn { moved: false },
                    (
                        asset_server.load(BLACK_PAWN_SPRITE),
                        asset_server.load(BORDERED_BLACK_PAWN_SPRITE),
                    ),
                ),
                (
                    PieceType::Pawn { moved: true },
                    (
                        asset_server.load(BLACK_PAWN_SPRITE),
                        asset_server.load(BORDERED_BLACK_PAWN_SPRITE),
                    ),
                ),
                (
                    PieceType::Bishop,
                    (
                        asset_server.load(BLACK_BISHOP_SPRITE),
                        asset_server.load(BORDERED_BLACK_BISHOP_SPRITE),
                    ),
                ),
                (
                    PieceType::Rook,
                    (
                        asset_server.load(BLACK_ROOK_SPRITE),
                        asset_server.load(BORDERED_BLACK_ROOK_SPRITE),
                    ),
                ),
                (
                    PieceType::King,
                    (
                        asset_server.load(BLACK_KING_SPRITE),
                        asset_server.load(BORDERED_BLACK_KING_SPRITE),
                    ),
                ),
                (
                    PieceType::Queen,
                    (
                        asset_server.load(BLACK_QUEEN_SPRITE),
                        asset_server.load(BORDERED_BLACK_QUEEN_SPRITE),
                    ),
                ),
                (
                    PieceType::Knight,
                    (
                        asset_server.load(BLACK_KNIGHT_SPRITE),
                        asset_server.load(BORDERED_BLACK_KNIGHT_SPRITE),
                    ),
                ),
            ]),
            error_image: asset_server.load(RONALDO),
        }
    }
}

#[derive(Default, Component, Debug)]
struct FpsText;

#[derive(Default, Component, Debug)]
pub struct ColorText;

fn init_next_move_text(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn((
        TextBundle::from_section(
            "Next move: ",
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(50.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font,
                font_size: 60.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(50.0),
                ..default()
            },
            ..default()
        }),
        FpsText,
    ));
}

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in &mut query {
        let seconds = time.elapsed_seconds();

        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}

fn change_text_system(mut query: Query<&mut Text, With<ColorText>>, game_state: Res<GameState>) {
    if game_state.white {
        let mut item = query.single_mut();
        item.sections[0].value = "Next move: White".to_string();
    } else {
        let mut item = query.single_mut();
        item.sections[0].value = "Next move: Black".to_string();
    }
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(init_next_move_text)
            .add_system(text_color_system)
            .add_system(text_update_system)
            .add_system(change_text_system);
    }
}
