use crate::{field::Field, ui::*, *};
use bevy_kira_audio::AudioControl;

#[derive(Component)]
struct PlayAgainButton;

#[derive(Component)]
struct GameOverText;

#[derive(Component)]

struct GameOverRoot;

pub fn despawn_board(
    commands: &mut Commands,
    piece_query: &Query<Entity, With<Piece>>,
    field_query: &Query<Entity, With<Field>>,
    color_text_qury: &Query<Entity, With<ColorText>>,
    fps_text_qury: &Query<Entity, With<FpsText>>,
) {
    let color_text_e = color_text_qury.single();
    commands.entity(color_text_e).despawn_recursive();

    let fps_text_e = fps_text_qury.single();
    commands.entity(fps_text_e).despawn_recursive();

    for entity in field_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in piece_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[allow(clippy::too_many_arguments)]
fn play_again_button_clicked(
    mut commands: Commands,
    mut interactions: Query<(&Interaction, &mut BackgroundColor), With<PlayAgainButton>>,
    mut global_state: ResMut<State<GlobalState>>,
    piece_query: Query<Entity, With<Piece>>,
    field_query: Query<Entity, With<Field>>,
    color_text_query: Query<Entity, With<ColorText>>,
    fps_text_query: Query<Entity, With<FpsText>>,
    game_over_root: Query<Entity, With<GameOverRoot>>,
) {
    for (interaction, mut color) in &mut interactions {
        match *interaction {
            Interaction::Clicked => {
                let game_over_e = game_over_root.single();
                commands.entity(game_over_e).despawn_recursive();

                despawn_board(
                    &mut commands,
                    &piece_query,
                    &field_query,
                    &color_text_query,
                    &fps_text_query,
                );

                global_state
                    .set(GlobalState::MainMenu)
                    .expect("Error in setting state");
            }
            Interaction::Hovered => {
                *color = BURGUNDY_LIGHT.into();
            }
            Interaction::None => {
                *color = TRANSPARENT_BURGUNDY.into();
            }
        }
    }
}

pub fn spawn_text(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    game_state: ResMut<GameState>,
) -> Entity {
    let mut winner = " DRAW";
    if let Some(color) = game_state.winner {
        match color {
            PieceColor::White => winner = " WHITE WINS!",
            PieceColor::Black => winner = " BLACK WINS!",
        }
    }

    let text = format!("{}{}", "GAME OVER:", winner);

    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                margin: UiRect::all(Val::Percent(3.0)),
                ..default()
            },
            text: Text::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            ..default()
        })
        .id()
}

pub fn spawn_button(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(20.0), Val::Percent(8.0)),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Percent(2.0)),
                ..default()
            },
            background_color: TRANSPARENT_BURGUNDY.into(),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    "PLAY AGAIN",
                    TextStyle {
                        font: asset_server.load("fonts/Aboreto-Regular.ttf"),
                        font_size: 25.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        })
        .id()
}

fn spawn_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: ResMut<GameState>,
    audio: Res<bevy_kira_audio::prelude::Audio>,
) {
    let play_again_button = spawn_button(&mut commands, &asset_server);
    commands.entity(play_again_button).insert(PlayAgainButton);
    let game_over_text = spawn_text(&mut commands, &asset_server, game_state);
    commands.entity(game_over_text).insert(GameOverText);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: TRANSPARENT_GRAY.into(),
            ..default()
        })
        .insert(GameOverRoot)
        .add_child(game_over_text)
        .add_child(play_again_button);

    audio.resume();
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GlobalState::GameOver).with_system(spawn_game_over))
            .add_system_set(
                SystemSet::on_update(GlobalState::GameOver).with_system(play_again_button_clicked),
            );
    }
}
