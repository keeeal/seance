use bevy::prelude::*;
use bevy_interact_2d::{Group, Interactable, InteractionPlugin, InteractionSource, InteractionState};
use rand::prelude::*;

// Are groups really necessary?
// What benefit do they provide over just defining and reusing components?
const TRASH_GROUP: u8 = 0;
const TRASHCAN_GROUP: u8 = 1;


struct Clickable {
    groups: Vec<Group>
}


impl Default for Clickable {
    fn default() -> Self {
        Self {
            groups: vec![Group::default()],
        }
    }
}


fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(InteractionSource {
            groups: vec![Group(TRASHCAN_GROUP), Group(TRASH_GROUP)],
            ..Default::default()
    });


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
                groups: vec![Group(TRASH_GROUP)],
                bounding_box: (Vec2::new(-12., -12.), Vec2::new(12., 12.)),
                ..Default::default()
            })
            .insert(Clickable {
                groups: vec![Group(TRASH_GROUP)],
                ..Default::default()
            })
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


fn click_trash(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    interaction_state: Res<InteractionState>,
    clickables: Query<(Entity, &Clickable), With<Clickable>>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return
    }

    // Is this the only / best way to determine if the entity was clicked?
    for (entity, clickable) in clickables.iter() {
        for group in clickable.groups.iter() {
            if let Some(list) = interaction_state.ordered_interact_list_map.get(group) {
                if let Some((_, position)) = list.iter().find(|(e, _)| e == &entity) {
                    info!("trash clicked");
                    break;
                }
            }
        }
    }
}


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(InteractionPlugin)
        .add_startup_system(startup.system())
        .add_system(click_trash.system())
        .run();
}