use bevy::prelude::{
    AppBuilder, IntoSystem, Plugin, Query, Res, TextureAtlasSprite, Time, Timer, Commands,
    Entity, EventReader,
};
use crate::dialogue::{AnimationStartEvent, AnimationEndEvent};
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone)]
pub enum AnimationDefinition {
    Simple,
    Progression(fn(usize) -> usize),
    WithState(fn(usize) -> (usize, usize)),
}


// Predefined animations
pub const BLINK_ANIMATION: AnimationDefinition = AnimationDefinition::WithState(|state| match state {
        // Closed
        0 => {
            let mut rng = rand::thread_rng();
            (2, rng.gen_range(16..32))
        },
        // Closing
        1 => (1, 0),
        // Open
        i => (0, i-1)
    });

pub const TALK_ANIMATION: AnimationDefinition =
    AnimationDefinition::WithState(|state| match state {
        // Opening
        0 => {
            let mut rng = rand::thread_rng();
            (1, rng.gen_range(3..10))
        },
        // Closed
        1 => (0, 0),
        // Closing
        2 => {
            (1, 1)
        },
        // Open
        i => (2, i-3)
    });

pub struct Transitions {
    default: (AnimationDefinition, Vec<u32>),
    animations: HashMap<String, (AnimationDefinition, Vec<u32>)>,
    current: Option<String>,
}

pub struct Animation {
    definition: AnimationDefinition,
    frames: Vec<u32>,
    state: usize,
}

pub fn animation_bundle(default: (AnimationDefinition, Vec<u32>),
                        animations: HashMap<String, (AnimationDefinition, Vec<u32>)>) -> (Transitions, Animation, Timer) {
    (
        Transitions {
            default: default.clone(),
            animations,
            current: None,
        },
        Animation {
            definition: default.0,
            frames: default.1,
            state: 0,
        },
        Timer::from_seconds(0.2, true),
    )
}

fn handle_animation_transitions(
    mut transition_query: Query<(Entity, &mut Transitions)>,
    mut ev_start: EventReader<AnimationStartEvent>,
    mut ev_end: EventReader<AnimationEndEvent>,
    mut commands: Commands,
) {
    for AnimationStartEvent ( label ) in ev_start.iter() {
        for (entity, mut transitions) in transition_query.iter_mut() {
            if transitions.current == Some(label.clone()) {
                continue;
            }
            if let Some((definition, frames)) = transitions.animations.get(label) {
                commands
                    .entity(entity)
                    .insert(Animation {
                        definition: definition.clone(),
                        frames: frames.clone(),
                        state: 0,
                    });

                transitions.current = Some(label.clone());
            }
        }
    }
    for AnimationEndEvent ( label ) in ev_end.iter() {
        for (entity, mut transitions) in transition_query.iter_mut() {
            if transitions.current == Some(label.clone()) {
                commands
                    .entity(entity)
                    .insert(Animation {
                        definition: transitions.default.0.clone(),
                        frames: transitions.default.1.clone(),
                        state: 0,
                    });

                transitions.current = None;
            }
        }
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut Animation)>,
) {
    for (mut timer, mut sprite, mut animation) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let (frame, state) = match &animation.definition {
                AnimationDefinition::Simple => {
                    let f = (animation.state + 1) % (animation.frames.len());
                    (f, f)
                }
                AnimationDefinition::Progression(get_frame) => {
                    let f = get_frame(animation.state);
                    (f, f)
                }
                AnimationDefinition::WithState(get_frame_and_state) => {
                    get_frame_and_state(animation.state)
                }
            };

            animation.state = state;
            sprite.index = animation.frames[frame];
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(animate_sprite_system.system())
            .add_system(handle_animation_transitions.system());
    }
}
