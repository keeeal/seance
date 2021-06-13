use bevy::prelude::{
    Plugin, Res, AppBuilder, IntoSystem, Handle, EventReader, Local,
};
use bevy_kira_audio::{Audio, AudioChannel, AudioSource};
use std::ops::Deref;


pub enum Channel {
    Music,
    Dialogue,
}

pub struct PlayAudioEvent {
    pub channel: Channel,
    pub handle: Handle<AudioSource>,
}

pub struct StopAudioEvent {
    pub channel: Channel,
}

struct MusicChannel(AudioChannel);

impl Default for MusicChannel {
    fn default() -> MusicChannel {
        return MusicChannel(AudioChannel::new("music".to_string()))
    }
}

impl Deref for MusicChannel {
    type Target = AudioChannel;

    #[inline]
    fn deref(&self) -> &AudioChannel {
        return &self.0
    }
}
struct DialogueChannel(AudioChannel);

impl Default for DialogueChannel {
    fn default() -> DialogueChannel {
        return DialogueChannel(AudioChannel::new("dialogue".to_string()))
    }
}

impl Deref for DialogueChannel {
    type Target = AudioChannel;

    #[inline]
    fn deref(&self) -> &AudioChannel {
        return &self.0
    }
}

fn events(
    mut ev_play: EventReader<PlayAudioEvent>,
    mut ev_stop: EventReader<StopAudioEvent>,
    audio: Res<Audio>,
    music_channel: Local<MusicChannel>,
    dialogue_channel: Local<DialogueChannel>,
) {
    for PlayAudioEvent { channel, handle } in ev_play.iter() {
        match channel {
            Channel::Music => audio.play_in_channel(handle.clone(), &music_channel),
            Channel::Dialogue => audio.play_in_channel(handle.clone(), &dialogue_channel),
        }
    }

    for StopAudioEvent { channel } in ev_stop.iter() {
        match channel {
            Channel::Music => audio.stop_channel(&music_channel),
            Channel::Dialogue => audio.stop_channel(&dialogue_channel),
        }
    }
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(bevy_kira_audio::AudioPlugin)
            .add_system(events.system())
            .add_event::<PlayAudioEvent>()
            .add_event::<StopAudioEvent>();
    }
}
