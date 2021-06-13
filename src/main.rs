use bevy::prelude::{App, EventReader, WindowDescriptor, DefaultPlugins, IntoSystem};

mod audio;
use audio::AudioPlugin;

mod concepts;
use concepts::ConceptPlugin;

mod dialogue;
use dialogue::DialoguePlugin;

mod ghost;
use ghost::{GhostPlugin, GhostInteractionEvent};

mod room;
use room::RoomPlugin;

mod animation;
use animation::AnimationPlugin;

mod question_display;
use question_display::QuestionDisplayPlugin;

fn ghost_interactions(mut event_reader: EventReader<GhostInteractionEvent>) {
    for GhostInteractionEvent { ghost, target } in event_reader.iter() {
        eprintln!("Entity {:?} interacted with {:?}", ghost, target);
    }
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: String::from("Seance"),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(GhostPlugin)
        .add_plugin(ConceptPlugin)
        .add_plugin(DialoguePlugin)
        .add_plugin(RoomPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(QuestionDisplayPlugin)
        .add_system(ghost_interactions.system())
        .run();
}
