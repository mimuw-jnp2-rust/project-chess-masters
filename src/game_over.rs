use crate::{field::Field, ui::*, *};

#[derive(Component)]
struct PlayAgainButton;

#[derive(Component)]
struct GameOverText;

/*fn despawn_all_with<C: Component>(query: Query<Entity, With<C>>, mut commands: Commands) {
    commands.entity(query.iter()).despawn();
}*/

fn play_again_button_clicked(
    mut commands: Commands,
    interactions: Query<&Interaction, (With<PlayAgainButton>, Changed<Interaction>)>,
    mut global_state: ResMut<State<GlobalState>>,
    piece_query: Query<Entity, With<Piece>>,
    field_query: Query<Entity, With<Field>>,
    color_text_qury: Query<Entity, With<ColorText>>,
    fps_text_qury: Query<Entity, With<FpsText>>,
    game_over_text_qury: Query<Entity, With<GameOverText>>,
    play_again_button: Query<Entity, With<PlayAgainButton>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            // despawnowansko
            let color_text_e = color_text_qury.single();
            commands.entity(color_text_e).despawn_recursive();

            let fps_text_e = fps_text_qury.single();
            commands.entity(fps_text_e).despawn_recursive();

            let game_over_text_e = game_over_text_qury.single();
            commands.entity(game_over_text_e).despawn_recursive();

            let play_again_button_e = play_again_button.single();
            commands.entity(play_again_button_e).despawn_recursive();

            for entity in field_query.iter() {
                commands.entity(entity).despawn_recursive();
            }

            for entity in piece_query.iter() {
                commands.entity(entity).despawn_recursive();
            }

            global_state.set(GlobalState::MainMenu).unwrap();
        }
    }
}

pub fn spawn_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: ResMut<GameState>,
) {
    spawn_button(&mut commands, &asset_server);
    spawn_text(&mut commands, &asset_server, game_state);
}

pub fn spawn_text(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    game_state: ResMut<GameState>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 35.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;
    let mut winner = " DRAW";
    if let Some(color) = game_state.winner {
        match color {
            PieceColor::White => winner = " WHITE WINS!",
            PieceColor::Black => winner = " BLACK WINS!",
        }
    }

    let text = format!("{}{}", "GAME OVER:", winner);
    // Demonstrate changing translation
    commands
        .spawn((Text2dBundle {
            text: Text::from_section(text, text_style.clone()).with_alignment(text_alignment),
            transform: Transform::from_translation(Vec3::new(0., 320., 1.)),
            ..default()
        },))
        .insert(GameOverText);
}

pub fn spawn_button(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(
                    Val::Px(PLAY_AGAIN_BUTTON_WIDTH),
                    Val::Px(PLAY_AGAIN_BUTTON_HEIGHT),
                ),
                align_self: AlignSelf::Center,
                margin: UiRect {
                    bottom: Val::Px(50.0),
                    right: Val::Auto,
                    left: Val::Auto,
                    top: Val::Auto,
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: DARK_GRAY.into(),
            ..default()
        })
        .insert(PlayAgainButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PLAY AGAIN",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
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
