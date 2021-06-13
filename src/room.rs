use crate::concepts::{Concept, EvokesConcept};
use crate::ghost::{ghost_bundle, Clickable};
use crate::animation::{animation_bundle, AnimationDefinition, TALK_ANIMATION};
use rand::Rng;
use bevy::prelude::{
    AppBuilder, AssetServer, Assets, Commands, IntoSystem, OrthographicCameraBundle, Plugin, Res,
    ResMut, SpriteSheetBundle, TextureAtlas, Transform, Vec2,
};
use bevy_interact_2d::{Interactable, InteractionSource};

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

    let youth = commands
        .spawn()
        .insert(Concept {
            description: String::from("Youth/Horse"),
            parents: vec![],
        })
        .id();

    let time = commands
        .spawn()
        .insert(Concept {
            description: String::from("Time"),
            parents: vec![],
        })
        .id();

    let _past = commands
        .spawn()
        .insert(Concept {
            description: String::from("Past"),
            parents: vec![[youth, time].iter().copied().collect()],
        })
        .id();

    // place some clickable entities
    let horse_texture = asset_server.load("horse.png");
    let horse_atlas = texture_atlases.add(TextureAtlas::from_grid(
        horse_texture,
        Vec2::new(168., 107.),
        1,
        1,
    ));
    let _horse = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: horse_atlas,
            transform: Transform::from_xyz(148., -242., 0.),
            ..Default::default()
        })
        .insert(Interactable {
            bounding_box: (Vec2::new(-84., -53.), Vec2::new(84., 53.)),
            ..Default::default()
        })
        .insert(Clickable)
        .insert(EvokesConcept(youth))
        .id();

    let clock_texture = asset_server.load("clock.png");
    let clock_atlas = texture_atlases.add(TextureAtlas::from_grid(
        clock_texture,
        Vec2::new(120., 300.),
        1,
        1,
    ));
    let _clock = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: clock_atlas,
            transform: Transform::from_xyz(-200., -72., 0.),
            ..Default::default()
        })
        .insert(Interactable {
            bounding_box: (Vec2::new(-60., -150.), Vec2::new(60., 150.)),
            ..Default::default()
        })
        .insert(Clickable)
        .insert(EvokesConcept(time))
        .id();

    // commands
    //     .entity(entities[1])
    //     .insert(ClearsConcepts).id();

    // commands
    //     .entity(entities[3])
    //     .insert(EvokesConcept(time));

    // spawn each entity
    // commands
    //     .spawn()
    //     .insert_bundle((
    //         Transform::default(),
    //         GlobalTransform::default(),
    //     ))
    //     .push_children(&entities);
}

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system());
    }
}
