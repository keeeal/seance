use bevy::prelude::*;
use bevy_interact_2d::{
    Group, Interactable, InteractionPlugin, InteractionSource, InteractionState,
};

struct Clickable;

struct MoveTo {
    target: Option<(Entity, Vec3)>,
    vel: f32,
    interact_radius: f32,
}
struct ConceptSource(String);

struct GhostInteractionEvent {
    ghost: Entity,
    target: Entity,
}

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
    let background_atlas = texture_atlases.add(TextureAtlas::from_grid(
        background_texture,
        Vec2::new(1280., 720.),
        1,
        1,
    ));

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
    let horse_atlas = texture_atlases.add(TextureAtlas::from_grid(
        horse_texture,
        Vec2::new(168., 107.),
        1,
        1,
    ));
    let horse = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: horse_atlas.clone(),
            transform: Transform::from_xyz(148., -242., 0.),
            ..Default::default()
        })
        .insert(Interactable {
            bounding_box: (Vec2::new(-84., -53.), Vec2::new(84., 53.)),
            groups: vec![WORLD]
        })
        .insert(Clickable)
        .insert(ConceptSource(format!("horse")))
        .id();

    entities.push(horse);

    // load ghost
    let ghost_texture = asset_server.load("ghost.png");
    let ghost_atlas = texture_atlases.add(TextureAtlas::from_grid(
        ghost_texture,
        Vec2::new(128., 128.),
        1,
        1,
    ));

    let ghost = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: ghost_atlas,
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .insert(MoveTo {
            target: None,
            vel: 50.,
            interact_radius: 100.,
        })
        .id();
    entities.push(ghost);

    // spawn each entity
    commands
        .spawn()
        .insert_bundle((Transform::default(), GlobalTransform::default()))
        .push_children(&entities);
}

fn click(
    mouse_button_input: Res<Input<MouseButton>>,
    interaction_state: Res<InteractionState>,
    mut moveable_query: Query<&mut MoveTo>,
    concept_query: Query<(&ConceptSource, &Transform)>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    for (e, _) in interaction_state.get_group(WORLD) {
        if let Ok((src, transform)) = concept_query.get(e) {
            info!("{} clicked", src.0);

            if let Ok(mut moveable) = moveable_query.single_mut() {
                moveable.target = Some((e, transform.translation))
            }
            break;
        }
    }
}

fn move_system(
    time: Res<Time>,
    mut q: Query<(Entity, &mut MoveTo, &mut Transform)>,
    mut event_writer: EventWriter<GhostInteractionEvent>,
) {
    let delta = time.delta_seconds();

    for (ghost, mut move_to, mut t) in q.iter_mut() {
        if let Some((target, coords)) = move_to.target {
            let direction = coords - t.translation;
            let distance = delta * move_to.vel;
            if direction.length() < move_to.interact_radius {
                event_writer.send(GhostInteractionEvent { ghost, target });
                move_to.target = None;
            } else {
                let norm_direction = direction.normalize();
                t.translation += distance * norm_direction;
            }
        }
    }
}

fn ghost_interactions(mut event_reader: EventReader<GhostInteractionEvent>) {
    for GhostInteractionEvent { ghost, target } in event_reader.iter() {
        eprintln!("Entity {:?} interacted with {:?}", ghost, target);
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
        .add_system(ghost_interactions.system())
        .add_event::<GhostInteractionEvent>()
        .run();
}
