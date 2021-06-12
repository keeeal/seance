use bevy::prelude::*;
use bevy_interact_2d::{Interactable, InteractionPlugin, InteractionSource, InteractionState, Group};

struct Clickable;

struct MoveTo { target: Vec3, vel: f32 }
struct ConceptSource(String);

const WORLD: Group = Group(0);
const UI: Group = Group(1);

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

    // place some clickable entities
    let horse_texture = asset_server.load("horse.png");
    let horse_atlas = texture_atlases.add(
        TextureAtlas::from_grid(horse_texture, Vec2::new(168., 107.), 1, 1)
    );
    
    let horse = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: horse_atlas.clone(),
            transform: Transform::from_xyz(148., -242., 0.),
            ..Default::default()
        })
        .insert(Interactable {
            bounding_box: (Vec2::new(-84., -53.), Vec2::new(84., 53.)),
            groups: vec![WORLD],
            ..Default::default()
        })
        .insert(Clickable)
        .insert(ConceptSource(format!("horse")))
        .id();

    entities.push(horse);

    // load ghost
    let ghost_texture = asset_server.load("ghost.png");
    let ghost_atlas = texture_atlases.add(
        TextureAtlas::from_grid(ghost_texture, Vec2::new(128., 128.), 1, 1)
    );

    let ghost = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: ghost_atlas,
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .insert(MoveTo {
            target: Vec3::new(50., 80., 0.),
            vel: 50.
        })
        .id();
    
    entities.push(ghost);

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
    windows: Res<Windows>,
    mut moveable_query: Query<&mut MoveTo>,
    concept_query: Query<&ConceptSource>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return
    }

    if let Some(window) = windows.get(interaction_state.last_window_id) {
        // info!("YESBOSS {}", interaction_state.last_cursor_position);
        if let Ok(mut moveable) = moveable_query.single_mut() {
            moveable.target = Vec3::new(
                interaction_state.last_cursor_position.x - window.width() / 2.,
                interaction_state.last_cursor_position.y - window.height() / 2.,
                0.,
            )
        }
    }

    for (e, _) in interaction_state.get_group(WORLD) {
        if let Ok(src) = concept_query.get(e) {
            info!("{} clicked", src.0);
            break
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
