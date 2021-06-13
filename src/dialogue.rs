use bevy::prelude::{
    Commands, Plugin, AppBuilder, IntoSystem, TextBundle, Style, AlignSelf,
    PositionType, Rect, Val, Text, TextStyle, Color, TextAlignment,
    HorizontalAlign, VerticalAlign, Res, AssetServer, Time, Query, With, UiCameraBundle,
    Entity, EventWriter, Handle, AudioSource, Audio, Size, AlignContent, AlignItems,
};
use crate::concepts::Evoked;
use std::time::Duration;

pub struct Line {
    pub text: String,
    pub priority: i32,
    pub duration: Duration,
    pub audio: Option<Handle<AudioSource>>,
    pub repeatable: bool,
    pub responds_to_concepts: Vec<Entity>,
    pub groups: Vec<Entity>,
    pub animations: Vec<String>,
    pub requires_concepts: Vec<Entity>,
    pub consumes_concepts: Vec<Entity>,
    pub requires_spoken: Vec<Entity>,
    pub conflicts_spoken: Vec<Entity>,
}

impl Default for Line {
    fn default() -> Line {
        Line {
                text: "".to_string(),
                priority: 0,
                duration: Duration::from_secs(3),
                audio: None,
                repeatable: false,
                responds_to_concepts: vec![],
                groups: vec![],
                animations: vec![],
                requires_concepts: vec![],
                consumes_concepts: vec![],
                requires_spoken: vec![],
                conflicts_spoken: vec![],
        }
    }
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
    audio: Res<Audio>,
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

        // Play audio
        if let Some(audio_handle) = &line.audio {
            audio.play(audio_handle.clone());
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

fn dialogue_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(3. * 5.0),
                    left: Val::Px(3. * 15.0),
                    right: Val::Px(3. * 15.0),
                    ..Default::default()
                },
                max_size: Size {
                    width: Val::Px(3840. - 2. * 3. * 15.),
                    height: Val::Px(3. * 200.0),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TextBox {
            style: TextStyle {
                font: asset_server.load("GloriaHallelujah-Regular.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
            alignment: TextAlignment {
                horizontal: HorizontalAlign::Left,
                vertical: VerticalAlign::Top,
            }
        });
}

struct TextBox {
    style: TextStyle,
    alignment: TextAlignment,
}

fn render_lines(
    mut commands: Commands,
    text: Query<(Entity, &TextBox)>,
    lines: Query<&Line, With<Speaking>>,
) {
    if let Some(line) = lines.iter().next() {
        for (e, t) in text.iter() {
            commands
                .entity(e)
                .insert(Text::with_section(
                    &line.text,
                    t.style.clone(),
                    t.alignment,
                ));
        }
    }
}

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(progress_dialogue.system())
            .add_event::<AnimationStartEvent>()
            .add_event::<AnimationEndEvent>();
        app.add_startup_system(dialogue_startup.system());
        app.add_system(render_lines.system());
    }
}
