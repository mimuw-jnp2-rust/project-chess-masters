use bevy_kira_audio::AudioControl;

use crate::game_over::despawn_board;
use crate::main_menu::spawn_menu_button;
use crate::*;
use crate::{field::Field, ui::*};

#[derive(Component)]
struct BackButton;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct PauseMenuRoot;

#[allow(clippy::too_many_arguments)]
fn handle_exit_button(
    mut commands: Commands,
    mut interactions: Query<(&Interaction, &mut BackgroundColor), With<ExitButton>>,
    pause_root: Query<Entity, With<PauseMenuRoot>>,
    mut global_state: ResMut<State<GlobalState>>,
    piece_query: Query<Entity, With<Piece>>,
    field_query: Query<Entity, With<Field>>,
    color_text_query: Query<Entity, With<ColorText>>,
    fps_text_query: Query<Entity, With<FpsText>>,
    mut whose_turn: ResMut<State<WhoseTurn>>,
) {
    for (interaction, mut color) in &mut interactions {
        match *interaction {
            Interaction::Clicked => {
                if whose_turn.current() == &WhoseTurn::Bot {
                    whose_turn.set(WhoseTurn::Player).unwrap();
                }
                despawn_board(
                    &mut commands,
                    &piece_query,
                    &field_query,
                    &color_text_query,
                    &fps_text_query,
                );

                let root_entity = pause_root.single();
                commands.entity(root_entity).despawn_recursive();
                global_state.set(GlobalState::MainMenu).unwrap();
            }
            Interaction::Hovered => {
                *color = BURGUNDY_LIGHT.into();
            }
            Interaction::None => {
                *color = BURGUNDY_DARK.into();
            }
        }
    }
}

fn handle_back_to_game_button(
    mut commands: Commands,
    mut interactions: Query<(&Interaction, &mut BackgroundColor), With<BackButton>>,
    pause_root: Query<Entity, With<PauseMenuRoot>>,
    mut global_state: ResMut<State<GlobalState>>,
    audio: Res<bevy_kira_audio::prelude::Audio>,
) {
    for (interaction, mut color) in &mut interactions {
        match *interaction {
            Interaction::Clicked => {
                let root_entity = pause_root.single();
                commands.entity(root_entity).despawn_recursive();
                audio.pause();
                global_state.pop().unwrap();
            }
            Interaction::Hovered => {
                *color = BURGUNDY_LIGHT.into();
            }
            Interaction::None => {
                *color = BURGUNDY_DARK.into();
            }
        }
    }
}

fn spawn_game_paused(mut commands: Commands, asset_server: Res<AssetServer>) {
    let back_to_game_button = spawn_menu_button(&mut commands, &asset_server, KEEP_PLAYING);
    commands.entity(back_to_game_button).insert(BackButton);
    let exit_button = spawn_menu_button(&mut commands, &asset_server, EXIT_TO_MENU_TEXT);
    commands.entity(exit_button).insert(ExitButton);

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
        .insert(PauseMenuRoot)
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    "Game Paused",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 85.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        })
        .add_child(back_to_game_button)
        .add_child(exit_button);
}

pub struct GamePausedPlugin;

impl Plugin for GamePausedPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GlobalState::Paused).with_system(spawn_game_paused));
        app.add_system_set(
            SystemSet::on_update(GlobalState::Paused)
                .with_system(handle_back_to_game_button)
                .with_system(handle_exit_button),
        );
    }
}
