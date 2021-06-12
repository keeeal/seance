use bevy::prelude::{
    Plugin, Res, Transform, Input, MouseButton, AppBuilder,
    Entity, Vec3, Query, With, EventWriter, Time, IntoSystem,
};
use bevy_interact_2d::{InteractionPlugin, InteractionState};

pub struct Clickable;

pub struct MoveTo {
    target: Option<(Entity, Vec3)>,
    vel: f32,
    interact_radius: f32,
}

pub struct GhostInteractionEvent {
    pub ghost: Entity,
    pub target: Entity,
}

pub fn ghost_bundle() -> (MoveTo,) {
    (
        MoveTo {
            target: None,
            vel: 50.,
            interact_radius: 100.,
        },
    )
}

fn click(
    mouse_button_input: Res<Input<MouseButton>>,
    interaction_state: Res<InteractionState>,
    mut moveable_query: Query<&mut MoveTo>,
    target_query: Query<&Transform, With<Clickable>>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    for (e, _) in interaction_state.get_group(Default::default()) {
        if let Ok(transform) = target_query.get(e) {
            if let Ok(mut moveable) = moveable_query.single_mut() {
                moveable.target = Some((e, transform.translation))
            }
            break;
        }
    }
}

fn movement(
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

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(InteractionPlugin)
            .add_system(click.system())
            .add_system(movement.system())
            .add_event::<GhostInteractionEvent>();
    }
}
