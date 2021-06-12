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

    // load background
    let background_texture = asset_server.load("background.png");
    let background_atlas = texture_atlases.add(
        TextureAtlas::from_grid(background_texture, Vec2::new(1280., 720.), 1, 1)
    );

    let background = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: background_atlas,
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .id();
    
    let mut entities = vec![background];

    // load sprites
    let trash_texture = asset_server.load("trash.png");
    let trash_atlas = texture_atlases.add(
        TextureAtlas::from_grid(trash_texture, Vec2::new(24., 24.), 3, 1)
    );

    // place some clickable entities
    for i in 0..3 {
        let trash = commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: trash_atlas.clone(),
                sprite: TextureAtlasSprite::new(i),
                transform: Transform::from_xyz(
                    random::<f32>()*100.-50., random::<f32>()*100.-50.,0.
                ),
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

    // spawn each entity
    commands
        .spawn()
        .insert_bundle((
            Transform::default(),
            GlobalTransform::default(),
        ))
        .push_children(&entities);
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
        .insert_resource(WindowDescriptor {
            title: String::from("Seance"),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(InteractionPlugin)
        .add_startup_system(startup.system())
        .add_system(click.system())
        .run();
}
