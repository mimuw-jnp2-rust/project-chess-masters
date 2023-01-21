use crate::*;

pub fn spawn_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    game_state: ResMut<GameState>,
) {
    spawn_button(&mut commands, &asset_server, windows);
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
    commands.spawn((Text2dBundle {
        text: Text::from_section(text, text_style.clone()).with_alignment(text_alignment),
        transform: Transform::from_translation(Vec3::new(0., 320., 1.)),
        ..default()
    },));
}

pub fn spawn_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    windows: Res<Windows>,
) {
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
            background_color: PLAY_AGAIN_COL.into(),
            ..default()
        })
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

/*pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set(
            SystemSet::on_enter(GlobalState::GameOver).with_system(spawn_game_over),
        );
    }
}*/
