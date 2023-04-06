use crate::*;
use bevy::app::AppExit;
use bevy_kira_audio::prelude::*;

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct BotButton;

#[derive(Component)]
struct QuitButton;

#[derive(Component)]
struct MenuBackground;

fn despawn_menu(
    commands: &mut Commands,
    menu_root: &Query<Entity, With<MainMenuRoot>>,
    menu_background: &Query<Entity, With<MenuBackground>>,
) {
    let root_entity = menu_root.single();
    let background_entity = menu_background.single();

    commands.entity(root_entity).despawn_recursive();
    commands.entity(background_entity).despawn();
}

fn handle_start_button(
    mut commands: Commands,
    mut interactions: Query<(&Interaction, &mut BackgroundColor), With<StartButton>>,
    menu_root: Query<Entity, With<MainMenuRoot>>,
    menu_background: Query<Entity, With<MenuBackground>>,
    mut global_state: ResMut<State<GlobalState>>,
    audio: Res<bevy_kira_audio::prelude::Audio>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color) in &mut interactions {
        match *interaction {
            Interaction::Clicked => {
                game_state.vs_bot = false;
                game_state.white = true;
                game_state.bot_turn = false;
                game_state.winner = None;
                despawn_menu(&mut commands, &menu_root, &menu_background);
                global_state
                    .set(GlobalState::InGame)
                    .expect("Error in setting state");
                audio.pause().fade_out(AudioTween::default());
            }
            Interaction::Hovered => {
                *color = LIGHT_GRAY.into();
            }
            Interaction::None => {
                *color = DARK_GRAY.into();
            }
        }
    }
}

fn handle_bot_button(
    mut commands: Commands,
    mut interactions: Query<(&Interaction, &mut BackgroundColor), With<BotButton>>,
    menu_root: Query<Entity, With<MainMenuRoot>>,
    menu_background: Query<Entity, With<MenuBackground>>,
    mut global_state: ResMut<State<GlobalState>>,
    audio: Res<bevy_kira_audio::prelude::Audio>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color) in &mut interactions {
        match *interaction {
            Interaction::Clicked => {
                game_state.vs_bot = true;
                game_state.white = true;
                game_state.bot_turn = false;
                game_state.winner = None;
                despawn_menu(&mut commands, &menu_root, &menu_background);
                global_state
                    .set(GlobalState::InGame)
                    .expect("Error in setting state");
                audio.pause().fade_out(AudioTween::default());
            }
            Interaction::Hovered => {
                *color = LIGHT_GRAY.into();
            }
            Interaction::None => {
                *color = DARK_GRAY.into();
            }
        }
    }
}

fn handle_quit_button(
    mut commands: Commands,
    mut interactions: Query<(&Interaction, &mut BackgroundColor), With<QuitButton>>,
    menu_root: Query<Entity, With<MainMenuRoot>>,
    menu_background: Query<Entity, With<MenuBackground>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color) in &mut interactions {
        match *interaction {
            Interaction::Clicked => {
                despawn_menu(&mut commands, &menu_root, &menu_background);
                exit.send(AppExit);
            }
            Interaction::Hovered => {
                *color = LIGHT_GRAY.into();
            }
            Interaction::None => {
                *color = DARK_GRAY.into();
            }
        }
    }
}

pub fn spawn_menu_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    text: &str,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(40.0), Val::Percent(8.0)),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Percent(2.0)),
                ..default()
            },
            background_color: DARK_GRAY.into(),
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
                    text,
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

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>, window: Res<Windows>) {
    let start_game_button = spawn_menu_button(&mut commands, &asset_server, FRIEND_TEXT);
    commands.entity(start_game_button).insert(StartButton);
    let bot_button = spawn_menu_button(&mut commands, &asset_server, BOT_TEXT);
    commands.entity(bot_button).insert(BotButton);
    let quit_button = spawn_menu_button(&mut commands, &asset_server, QUIT_TEXT);
    commands.entity(quit_button).insert(QuitButton);

    let window = window.get_primary().unwrap();
    let mut scale_x = window.width() / 2560.0;
    let mut scale_y = window.height() / 1440.0;
    if window.width() < 1000.0 || window.height() < 800.0 {
        scale_x = 0.75;
        scale_y = 0.75;
    }

    let background_image: Handle<Image> = asset_server.load("background.png");
    commands
        .spawn(SpriteBundle {
            texture: background_image,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                scale: Vec3::new(scale_x, scale_y, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(MenuBackground);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(MainMenuRoot)
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    "Welcome to Chess",
                    TextStyle {
                        font: asset_server.load("fonts/Aboreto-Regular.ttf"),
                        font_size: 85.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        })
        .add_child(start_game_button)
        .add_child(bot_button)
        .add_child(quit_button);
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GlobalState::MainMenu).with_system(spawn_main_menu))
            .add_system_set(
                SystemSet::on_update(GlobalState::MainMenu)
                    .with_system(handle_start_button)
                    .with_system(handle_quit_button)
                    .with_system(handle_bot_button),
            );
    }
}
