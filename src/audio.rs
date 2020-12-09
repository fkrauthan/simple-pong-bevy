use bevy::asset::{AssetServer, Handle};
use bevy::audio::{Audio, AudioSource};
use bevy::ecs::{Commands, Res};
use std::iter::Cycle;
use std::vec::IntoIter;

const BOUNCE_SOUND: &str = "audio/bounce.ogg";
const SCORE_SOUND: &str = "audio/score.ogg";

const MUSIC_TRACKS: &[&str] = &[
    "audio/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
    "audio/Computer_Music_All-Stars_-_Albatross_v2.ogg",
];

pub struct Sounds {
    pub score_sfx: Handle<AudioSource>,
    pub bounce_sfx: Handle<AudioSource>,
}

pub struct Music {
    pub music: Cycle<IntoIter<Handle<AudioSource>>>,
}

fn load_audio_track(asset_server: &Res<AssetServer>, file: &str) -> Handle<AudioSource> {
    asset_server.load(file)
}

// TODO play game music

pub fn initialise_audio(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let (sound_effects, music) = {
        let music = MUSIC_TRACKS
            .iter()
            .map(|file| load_audio_track(&asset_server, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };

        let sound = Sounds {
            bounce_sfx: load_audio_track(&asset_server, BOUNCE_SOUND),
            score_sfx: load_audio_track(&asset_server, SCORE_SOUND),
        };

        (sound, music)
    };

    commands
        .insert_resource(sound_effects)
        .insert_resource(music);
}

pub fn play_bounce_sound(audio: &Res<Audio>, sounds: &Res<Sounds>) {
    audio.play((*sounds).bounce_sfx.clone());
}

pub fn play_score_sound(audio: &Res<Audio>, sounds: &Res<Sounds>) {
    audio.play((*sounds).score_sfx.clone());
}
