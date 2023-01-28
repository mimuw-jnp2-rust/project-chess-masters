use crate::*;
use bevy_kira_audio::{AudioChannel, AudioPlugin};

pub struct ChessAudioPlugin;

pub struct AudioState {
    background_handle: Handle<AudioSource>,
}

impl Plugin for ChessAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            //.add_startup_system_to_stage(StartupStage::PreStartup, load_audio)
            .add_system_set(
                SystemSet::on_enter(GlobalState::MainMenu).with_system(start_bgm_music),
            );
    }
}

fn start_bgm_music(audio: Res<Audio>, assets: Res<AssetServer>) {
    println!("playing song lala");
    audio.play(assets.load("background_music.wav"));
}

fn load_audio(mut commands: Commands, audio: Res<Audio>, assets: Res<AssetServer>) {
    let background_handle: Handle<AudioSource> = assets.load("background_music.wav");
    let volume = 0.5;
}
