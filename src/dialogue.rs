use bevy::prelude::{
    Plugin, AppBuilder
};
use std::time::Duration;

pub struct Line {
    pub text: String,
}

pub struct Spoken(Vec<Duration>);


pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut AppBuilder) {
    }
}
