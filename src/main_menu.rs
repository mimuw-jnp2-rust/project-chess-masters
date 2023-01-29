use crate::*;
use bevy_kira_audio::prelude::*;

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct BotButton;

#[derive(Component)]
struct MenuBackground;

fn start_button_clicked(
    mut commands: Commands,
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, Changed<Interaction>),
    >,
    menu_root: Query<Entity, With<MainMenuRoot>>,
    menu_background: Query<Entity, With<MenuBackground>>,
    mut global_state: ResMut<State<GlobalState>>,
    audio: Res<bevy_kira_audio::prelude::Audio>,
    //mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color) in &mut interactions {
        //let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = bevy::prelude::BackgroundColor(Color::BLACK);
                let root_entity = menu_root.single();
                let background_entity = menu_background.single();

                commands.entity(root_entity).despawn_recursive();
                commands.entity(background_entity).despawn();

                global_state.set(GlobalState::InGame).unwrap();
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

fn spawn_button(commands: &mut Commands, asset_server: &AssetServer, text: &str) -> Entity {
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

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // , windows: Res<Windows>
    //let window = windows.get_primary().unwrap();
    //let (height, width) = (window.height(), window.width());
    //print!("Window height= {} width = {}", height, width);
    //println!("Spawning main menu");

    let start_game_button = spawn_button(&mut commands, &asset_server, FRIEND_TEXT);
    commands.entity(start_game_button).insert(StartButton);
    let bot_button = spawn_button(&mut commands, &asset_server, BOT_TEXT);
    commands.entity(start_game_button).insert(BotButton);

    let background_image: Handle<Image> = asset_server.load("background.png");
    commands
        .spawn(SpriteBundle {
            texture: background_image,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                scale: Vec3::new(0.75, 0.75, 1.0),
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
        .add_child(bot_button);
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GlobalState::MainMenu).with_system(spawn_main_menu))
            .add_system_set(
                SystemSet::on_update(GlobalState::MainMenu).with_system(start_button_clicked),
            );
    }
}
