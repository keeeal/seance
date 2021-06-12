use bevy::prelude::{
    Commands, Res, AssetServer, ResMut, Assets, TextureAtlas,
    OrthographicCameraBundle, Vec2, SpriteSheetBundle,
    Transform, Plugin, AppBuilder, IntoSystem,
};
use bevy_interact_2d::{Interactable, InteractionSource};
use crate::ghost::{Clickable, ghost_bundle};
use crate::concepts::{Concept, EvokesConcept};


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
    let background_atlas = texture_atlases.add(
        TextureAtlas::from_grid(background_texture, Vec2::new(1280., 720.), 1, 1)
    );

    let _background = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: background_atlas,
            transform: Transform::from_xyz(1., 0., 0.),
            ..Default::default()
        })
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
        .insert(
            Concept{
                description: String::from("Youth/Horse"),
                parents: vec![],
            }
        )
        .id();

    let time = commands
        .spawn()
        .insert(
            Concept{
                description: String::from("Time"),
                parents: vec![],
            }
        )
        .id();

    let _past = commands
        .spawn()
        .insert(
            Concept{
                description: String::from("Past"),
                parents: vec![[youth, time].iter().copied().collect()],
            }
        )
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
