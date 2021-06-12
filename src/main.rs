use bevy::prelude::{App, EventReader, WindowDescriptor, DefaultPlugins, IntoSystem};

mod concepts;
use concepts::ConceptPlugin;

mod ghost;
use ghost::{GhostPlugin, GhostInteractionEvent};

mod room;
use room::RoomPlugin;

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
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GhostPlugin)
        .add_plugin(ConceptPlugin)
        .add_plugin(RoomPlugin)
        .add_system(ghost_interactions.system())
        .run();
}
