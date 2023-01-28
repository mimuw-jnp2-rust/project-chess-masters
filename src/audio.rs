use crate::*;
use bevy_kira_audio::prelude::*;
pub struct ChessAudioPlugin;

impl Plugin for ChessAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system(start_bgm_music)
            .add_system_set(
                SystemSet::on_enter(GlobalState::MainMenu).with_system(resume_bgm_music),
            );
    }
}

fn start_bgm_music(audio: Res<bevy_kira_audio::prelude::Audio>, assets: Res<AssetServer>) {
    println!("playing song lala");
    audio.play(assets.load("background_music.wav")).looped();
}

fn resume_bgm_music(audio: Res<bevy_kira_audio::prelude::Audio>) {
    audio.resume();
}
