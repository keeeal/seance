use bevy::prelude::{AppBuilder, IntoSystem, Plugin, Query, Res, TextureAtlasSprite, Time, Timer};
use rand::Rng;

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

pub struct Animation {
    definition: AnimationDefinition,
    frames: Vec<u32>,
    state: usize,
}

pub fn animation_bundle(definition: AnimationDefinition, frames: Vec<u32>) -> (Animation, Timer) {
    (
        Animation {
            definition,
            frames, 
            state: 0,
        },
        Timer::from_seconds(0.2, true),
    )
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
        app.add_system(animate_sprite_system.system());
    }
}
