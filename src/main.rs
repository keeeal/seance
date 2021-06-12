use bevy::prelude::*;
use bevy_interact_2d::{Interactable, InteractionPlugin, InteractionSource, InteractionState};
use rand::prelude::*;

struct Clickable;

struct MoveTo { target: Vec3, vel: f32 }

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
        .insert(MoveTo {
            target: Vec3::new(5., 8., 0.),
            vel: 50.
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
    target_query: Query<&Transform>,
    mut moveable_query: Query<&mut MoveTo>
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return
    }

    for (_group, interact_list) in &interaction_state.ordered_interact_list_map {
        for (click_target, pos) in interact_list {
            if let Ok(target_coords) = target_query.get(*click_target) {
                info!("YESBOSS {} {}", target_coords.translation, pos);
                if let Ok(mut moveable) = moveable_query.single_mut() {
                    moveable.target = target_coords.translation
                }
            }
        }
    }
}

fn move_system(time: Res<Time>, mut q: Query<(&MoveTo, &mut Transform)>) {
    let delta = time.delta_seconds();

    for (move_to, mut t) in q.iter_mut() {
        let direction = move_to.target - t.translation;
        let distance = delta * move_to.vel;
        if direction.length() < distance {
            t.translation = move_to.target;
        } else {
            let norm_direction = direction.normalize();
            t.translation += distance * norm_direction;
        }
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
        .add_system(move_system.system())
        .run();
}
