use crate::concepts::{Concept, EvokesConcept};
use crate::ghost::{ghost_bundle, Clickable};
use crate::animation::{animation_bundle, AnimationDefinition, TALK_ANIMATION};
use crate::dialogue::{Line};
use rand::Rng;
use bevy::prelude::{
    AppBuilder, AssetServer, Assets, Commands, IntoSystem, OrthographicCameraBundle, Plugin, Res,
    ResMut, SpriteSheetBundle, TextureAtlas, Transform, Vec2,
};
use bevy_interact_2d::{Interactable, InteractionSource};
use std::time::Duration;

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(InteractionSource::default());

    // load background
    let background_texture = asset_server.load("background.png");
    let background_atlas = texture_atlases.add(TextureAtlas::from_grid(
        background_texture,
        Vec2::new(3840., 2160.),
        1,
        1,
    ));

    let _background = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: background_atlas,
            transform: Transform::from_xyz(1., 0., 0.),
            ..Default::default()
        })
        .id();

    // load medium
    let medium_texture = asset_server.load("characters/medium.png");
    let medium_atlas = texture_atlases.add(TextureAtlas::from_grid(
        medium_texture,
        Vec2::new(250.0, 700.0),
        3,
        3,
    ));

    let medium_idle = AnimationDefinition::WithState(|state| match state {
        0 => {
            let mut rng = rand::thread_rng();
            (5, rng.gen_range(16..32))
        }
        1 => (4, 0),
        i => (3, i-1)
    });

    let medium_talk_frames = vec![0,1,2];

    let _medium = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: medium_atlas,
            transform: Transform::from_xyz(20., -760., 0.),
            ..Default::default()
        })
        .insert_bundle(animation_bundle(TALK_ANIMATION, medium_talk_frames))
        .id();

    // load ghost
    let ghost_texture = asset_server.load("ghost.png");
    let ghost_atlas = texture_atlases.add(TextureAtlas::from_grid(
        ghost_texture,
        Vec2::new(128., 128.),
        1,
        1,
    ));

    let _ghost = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: ghost_atlas,
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .insert_bundle(ghost_bundle())
        .id();

    let s1_pause = commands
        .spawn()
        .insert(
            Line {
                text: "".to_string(),
                priority: 5,
                duration: Duration::from_secs(3),
                ..Default::default()
            }
        )
        .id();

    let s1_introduction_a = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "The world is cold and dark as you wander the halls of a home you used ",
                    "to find great comfort in. Your desire to leave it all behind is ",
                    "palpable but still something keeps you here. The presence of the ones ",
                    "you love. You see your daughters crying and your wife sitting quietly ",
                    "on the bed you share. You reach out but no one notices. Even the ",
                    "mirror on the wall refuses to portray your presence. Is this a dream?",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(40),
                audio: Some(asset_server.load("dialogue/NAR.S1.Introduction.mp3")),
                ..Default::default()
            }
        )
        .id();
    let s1_introduction_b = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Unconnected to the passage of time you watch strange happenings scare ",
                    "your family. Are you responsible? Why can’t you leave?",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(15),
                requires_spoken: vec![s1_introduction_a],
                ..Default::default()
            }
        )
        .id();
    let s1_introduction_c = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Suddenly, a warm light draws you to your living room. Your family is ",
                    "congregated around the dining table with an old friend, a medium, ",
                    "Madam Gretchen. A seat sits empty beckoning you into the circle.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(25),
                requires_spoken: vec![s1_introduction_b],
                ..Default::default()
            }
        )
        .id();
}

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system());
    }
}
