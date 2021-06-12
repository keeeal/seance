use bevy::prelude::*;
use bevy_interact_2d::{Interactable, InteractionPlugin, InteractionSource, InteractionState};
use rand::prelude::*;

struct Clickable;


fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(InteractionSource::default());

    // load textures from sprite sheets
    let trashcan_texture = asset_server.load("trashcan.png");
    let trashcan_atlas = texture_atlases.add(
        TextureAtlas::from_grid(trashcan_texture, Vec2::new(24., 24.), 2, 1)
    );

    let trash_texture = asset_server.load("trash.png");
    let trash_atlas = texture_atlases.add(
        TextureAtlas::from_grid(trash_texture, Vec2::new(24., 24.), 3, 1)
    );

    // define a trashcan and add to the entities vector
    let trashcan = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: trashcan_atlas,
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .id();

    let mut entities = vec![trashcan];

    // define three trash bags and push them onto entities too
    for i in 0..3 {
        let trash = commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: trash_atlas.clone(),
                transform: Transform::from_xyz(
                    random::<f32>()*100.-50., random::<f32>()*100.-50.,0.
                ),
                sprite: TextureAtlasSprite::new(i),
                ..Default::default()
            })
            .insert(Interactable {
                bounding_box: (Vec2::new(-12., -12.), Vec2::new(12., 12.)),
                ..Default::default()
            })
            .insert(Clickable)
            .id();

        entities.push(trash);
    }

    // spawn each entity at 3x scale
    commands
        .spawn()
        .insert_bundle((
            Transform {
                scale: Vec3::new(3., 3., 3.),
                ..Default::default()
            },
            GlobalTransform::default(),
        ))
        .push_children(&entities)
        .id();
    
    // what is the purpose of .id() ??
}


fn click(
    mouse_button_input: Res<Input<MouseButton>>,
    interaction_state: Res<InteractionState>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return
    }

    if !interaction_state.ordered_interact_list_map.is_empty() {
        info!("entity clicked");
    }
}


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(InteractionPlugin)
        .add_startup_system(startup.system())
        .add_system(click.system())
        .run();
}