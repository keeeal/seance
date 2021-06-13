use bevy::prelude::{
    Plugin, AppBuilder, Query, With, Res, Time, Commands, Entity, IntoSystem, EventWriter,
};
use crate::concepts::Evoked;
use std::time::Duration;

pub struct Line {
    pub text: String,
    pub priority: i32,
    pub duration: Duration,
    pub repeatable: bool,
    pub responds_to_concepts: Vec<Entity>,
    pub groups: Vec<Entity>,
    pub animations: Vec<String>,
    pub requires_concepts: Vec<Entity>,
    pub consumes_concepts: Vec<Entity>,
    pub requires_spoken: Vec<Entity>,
    pub conflicts_spoken: Vec<Entity>,
}

pub struct Spoken(Vec<Duration>);

pub struct Speaking;

pub struct AnimationStartEvent(String);
pub struct AnimationEndEvent(String);

pub fn progress_dialogue(
    speaking_query: Query<(Entity, &Line, &Spoken), With<Speaking>>,
    lines_query: Query<(Entity, &Line)>,
    spoken_query: Query<&Spoken>,
    concept_query: Query<Entity, With<Evoked>>,
    time: Res<Time>,
    mut commands: Commands,
    mut start_event_writer: EventWriter<AnimationStartEvent>,
    mut end_event_writer: EventWriter<AnimationEndEvent>,
) {
    // If a line is currently being spoken, check if it is done
    if let Some((entity, line, Spoken(timestamps))) = speaking_query.iter().next() {
        if let Some(recent_timestamp) = timestamps.iter().max() {
            if time.time_since_startup() > *recent_timestamp + line.duration {
                commands
                    .entity(entity)
                    .remove::<Speaking>();

                // End animations
                for animation in &line.animations {
                    end_event_writer.send(AnimationEndEvent(animation.clone()));
                }
            }

            return
        }
    }

    // If no lines are being spoken, and there is a line that can be spoken, say it
    if let Some((entity, line)) = lines_query
        .iter()
        .filter(|(entity, line)| {
            // Check concept requirements
            for concept in &line.requires_concepts {
                if let Err(_) = concept_query.get(*concept) {
                    return false;
                }
            }

            for concept in &line.responds_to_concepts {
                if let Err(_) = concept_query.get(*concept) {
                    return false;
                }
            }

            // Check for concept responses
            for concept in &line.responds_to_concepts {
                if let Ok(Spoken(timestamps)) = spoken_query.get(*concept) {
                    if let Some(max_time) = timestamps.iter().max() {
                        if time.time_since_startup() < *max_time + line.duration / 2 {
                            continue;
                        }
                    }
                }
                return false;
            }

            // Check dialogue requirements and conflicts
            if !line.repeatable {
                if let Ok(_) = spoken_query.get(*entity) {
                    return false;
                }
            }
            for dialogue in &line.requires_spoken {
                if let Err(_) = spoken_query.get(*dialogue) {
                    return false;
                }
            }
            for dialogue in &line.conflicts_spoken {
                if let Ok(_) = spoken_query.get(*dialogue) {
                    return false;
                }
            }

            return true;
        })
        .max_by_key(|(_, line)| line.priority)
    {
        // Update line
        if let Ok(Spoken(timestamps)) = spoken_query.get(entity) {
            let mut timestamps = timestamps.clone();
            timestamps.push(time.time_since_startup());
            commands
                .entity(entity)
                .insert(Spoken(timestamps))
                .insert(Speaking);
        } else {
            commands
                .entity(entity)
                .insert(Spoken(vec![time.time_since_startup()]))
                .insert(Speaking);
        }

        // Update groups
        for group in &line.groups {
            if let Ok(Spoken(timestamps)) = spoken_query.get(*group) {
                let mut timestamps = timestamps.clone();
                timestamps.push(time.time_since_startup());
                commands
                    .entity(*group)
                    .insert(Spoken(timestamps));
            } else {
                commands
                    .entity(*group)
                    .insert(Spoken(vec![time.time_since_startup()]));
            }
        }

        // Start animations
        for animation in &line.animations {
            start_event_writer.send(AnimationStartEvent(animation.clone()));
        }

        // Consume concepts
        for concept in &line.consumes_concepts {
            commands
                .entity(*concept)
                .remove::<Evoked>();
        }
    }

}

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(progress_dialogue.system())
            .add_event::<AnimationStartEvent>()
            .add_event::<AnimationEndEvent>();
    }
}
