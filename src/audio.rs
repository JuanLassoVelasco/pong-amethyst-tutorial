use std::{iter::Cycle, vec::IntoIter};
use amethyst::{
    assets::Loader,
    audio::{OggFormat, SourceHandle, AudioSink},
    ecs::{World, WorldExt},
    assets::AssetStorage,
    audio::{output::Output, Source},
};

const BOUNCE_SOUND: &str = "audio/bounce.ogg";
const SCORE_SOUND: &str = "audio/score.ogg";

const MUSIC_TRACKS: &[&str] = &[
    "audio/Computer_Music_All-Stars_-_Albatross_v2.ogg",
    "audio/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
];

pub struct Sounds {
    pub score_fx: SourceHandle,
    pub bounce_fx: SourceHandle,
}

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub fn initialize_audio(world: &mut World) {
    let (sound_fx, music) = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25); // music is loud, reduce sound

        let music = MUSIC_TRACKS
            .iter()
            .map(|file| load_audio_track(&loader, world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();

        let music = Music { music };

        let sound = Sounds {
            bounce_fx: load_audio_track(&loader, world, BOUNCE_SOUND),
            score_fx: load_audio_track(&loader, world, SCORE_SOUND),
        };

        (sound, music)
    };

    world.insert(sound_fx);
    world.insert(music);
}

pub fn play_bounce_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bounce_fx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_score_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.score_fx) {
            output.play_once(sound, 1.0);
        }
    }
}
