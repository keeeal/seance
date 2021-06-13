use crate::concepts::{Concept, EvokesConcept};
use crate::ghost::ghost_bundle;
use crate::animation::{animation_bundle, AnimationDefinition, TALK_ANIMATION, BLINK_ANIMATION};
use crate::dialogue::{Line};
use bevy::prelude::{
    AppBuilder, AssetServer, Assets, Commands, IntoSystem, OrthographicCameraBundle, Plugin, Res,
    ResMut, SpriteSheetBundle, TextureAtlas, Transform, Vec2, Vec3,
};
use bevy_interact_2d::InteractionSource;
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

    let medium_talk_frames = vec![0, 1, 2];
    let medium_blink_frames = vec![3, 4, 5];
    let _medium = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: medium_atlas,
            transform: Transform::from_xyz(20., -760., 0.),
            ..Default::default()
        })
        .insert_bundle(animation_bundle(
            (BLINK_ANIMATION, medium_blink_frames),
            [("medium_talk".to_string(), (TALK_ANIMATION, medium_talk_frames))].iter().cloned().collect(),
        ))
        .id();

    // load mother
    let mother_texture = asset_server.load("characters/mother.png");
    let mother_atlas = texture_atlases.add(TextureAtlas::from_grid(
        mother_texture,
        Vec2::new(320.0, 505.0),
        3,
        4,
    ));

    let mother_default_frames = vec![0, 1, 2];
    let mother_happy_frames = vec![3, 4, 5];
    let mother_scared_frames = vec![6, 7, 8];
    let mother_talk_frames = vec![9, 10, 11];
    let _mother = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: mother_atlas,
            transform: Transform::from_xyz(-422., -833., 0.),
            ..Default::default()
        })
        .insert_bundle(animation_bundle(
            (BLINK_ANIMATION, mother_default_frames),
            [("mother_happy".to_string(), (BLINK_ANIMATION, mother_happy_frames)),
             ("mother_scared".to_string(), (BLINK_ANIMATION, mother_scared_frames)),
             ("mother_talk".to_string(), (TALK_ANIMATION, mother_talk_frames))].iter().cloned().collect(),
        ))
        .id();

    // load twin1
    let twin1_texture = asset_server.load("characters/twin1.png");
    let twin1_atlas = texture_atlases.add(TextureAtlas::from_grid(
        twin1_texture,
        Vec2::new(320.0, 640.0),
        3,
        4,
    ));

    let twin1_default_frames = vec![0, 1, 2];
    let twin1_happy_frames = vec![3, 4, 5];
    let twin1_scared_frames = vec![6, 7, 8];
    let twin1_talk_frames = vec![9, 10, 11];
    let _twin1 = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: twin1_atlas,
            transform: Transform::from_xyz(-201., -770., 0.),
            ..Default::default()
        })
        .insert_bundle(animation_bundle(
            (BLINK_ANIMATION, twin1_default_frames),
            [("twin1_happy".to_string(), (BLINK_ANIMATION, twin1_happy_frames)),
             ("twin1_scared".to_string(), (BLINK_ANIMATION, twin1_scared_frames)),
             ("twin1_talk".to_string(), (TALK_ANIMATION, twin1_talk_frames))].iter().cloned().collect(),
        ))
        .id();

    // load twin2
    let twin2_texture = asset_server.load("characters/twin2.png");
    let twin2_atlas = texture_atlases.add(TextureAtlas::from_grid(
        twin2_texture,
        Vec2::new(320.0, 640.0),
        3,
        4,
    ));

    let twin2_default_frames = vec![0, 1, 2];
    let twin2_happy_frames = vec![3, 4, 5];
    let twin2_scared_frames = vec![6, 7, 8];
    let twin2_talk_frames = vec![9, 10, 11];
    let _twin2 = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: twin2_atlas,
            transform: Transform::from_xyz(287., -779., 0.),
            ..Default::default()
        })
        .insert_bundle(animation_bundle(
            (BLINK_ANIMATION, twin2_default_frames),
            [("twin2_happy".to_string(), (BLINK_ANIMATION, twin2_happy_frames)),
             ("twin2_scared".to_string(), (BLINK_ANIMATION, twin2_scared_frames)),
             ("twin2_talk".to_string(), (TALK_ANIMATION, twin2_talk_frames))].iter().cloned().collect(),
        ))
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

    // load narrator overlay
    let overlay_texture = asset_server.load("blackScreen.png");
    let overlay_atlas = texture_atlases.add(TextureAtlas::from_grid(
        overlay_texture,
        Vec2::new(1920., 1280.),
        2,
        1,
    ));

    let vignette_frames = vec![1];
    let narrator_frames = vec![0];
    let _overlay = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: overlay_atlas,
            transform: (
                Transform::from_xyz(0., 0., 10.)
                * Transform::from_scale(Vec3::new(3., 3., 3.))
            ),
            ..Default::default()
        })
        .insert_bundle(animation_bundle(
            (AnimationDefinition::Simple, vignette_frames),
            [("narrator_talk".to_string(), (AnimationDefinition::Simple, narrator_frames))].iter().cloned().collect(),
        ))
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
                    "Narrator: The world is cold and dark as you wander the halls of a home you used ",
                    "to find great comfort in. Your desire to leave it all behind is ",
                    "palpable but still something keeps you here. The presence of the ones ",
                    "you love. You see your daughters crying and your wife sitting quietly ",
                    "on the bed you share. You reach out but no one notices. Even the ",
                    "mirror on the wall refuses to portray your presence. Is this a dream?",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(40),
                starts_animations: vec!["narrator_talk".to_string()],
                requires_spoken: vec![s1_pause],
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
                    "Narrator: Unconnected to the passage of time you watch strange happenings scare ",
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
                    "Narrator: Suddenly, a warm light draws you to your living room. Your family is ",
                    "congregated around the dining table with an old friend, a medium, ",
                    "Madam Gretchen. A seat sits empty beckoning you into the circle.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(19),
                ends_animations: vec!["narrator_talk".to_string()],
                requires_spoken: vec![s1_introduction_b],
                ..Default::default()
            }
        )
        .id();

    let s1_introduction_pause = commands
        .spawn()
        .insert(
            Line {
                text: "".to_string(),
                priority: 5,
                duration: Duration::from_secs(5),
                requires_spoken: vec![s1_introduction_c],
                ..Default::default()
            }
        )
        .id();

    let s1_medium_q1_a = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Madam Gretchen: As we join hands we focus our wills, Joining together the worlds of ",
                    "the dead and the living. We are reaching out to whoever haunts this ",
                    "place.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(13),
                starts_animations: vec!["medium_talk".to_string()],
                requires_spoken: vec![s1_introduction_pause],
                ..Default::default()
            }
        )
        .id();
    let s1_medium_q1_b = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Madam Gretchen: Is someone here? If there is someone with us, give us a sign?",
                ).to_string(),
                priority: 5,
                question: Some("If there is someone with us, give us a sign?".to_string()),
                duration: Duration::from_secs(7),
                ends_animations: vec!["medium_talk".to_string()],
                requires_spoken: vec![s1_medium_q1_a],
                ..Default::default()
            }
        )
        .id();

    let s1_narrator_q1_a = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Narrator: The question coupled by the warm light strengthens your resolve.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(6),
                requires_spoken: vec![s1_medium_q1_b],
                animations: vec!["narrator_talk".to_string()],
                audio: Some(asset_server.load("dialogue/NAR.S1.Q1.mp3")),
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
