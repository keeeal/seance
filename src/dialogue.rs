use bevy::prelude::{
    Commands, Plugin, AppBuilder, IntoSystem, TextBundle, Style, AlignSelf,
    PositionType, Rect, Val, Text, TextStyle, Color, TextAlignment, info,
    HorizontalAlign, VerticalAlign, Res, AssetServer, Time, Query, With, UiCameraBundle,
    Entity, EventWriter, Handle, Size, AlignContent, AlignItems, Bundle,
};
use bevy_kira_audio::AudioSource;
use crate::concepts::Evoked;
use crate::question_display::{SetQuestionEvent, ClearQuestionEvent};
use crate::audio::{PlayAudioEvent, StopAudioEvent, Channel};
use std::time::Duration;
use std::ops::Deref;

#[must_use]
pub struct NodeStub<T>{
    node: T,
    next: Entity,
}

#[must_use]
pub struct NodeBuilder<'a, T>{
    node: T,
    next: Entity,
    commands: &'a mut Commands<'a>,
}

pub trait NodePart<Builder> {
    type Return;

    fn add_to(self, builder: Builder) -> Self::Return;
}

impl<T> NodeStub<T> {
    pub fn new(commands: &mut Commands) -> (NodeStub<T>, T)
    where T: From<Entity> + Copy {
        let next = commands.spawn().id();
        let node = T::from(next);
        (
            NodeStub {
                node,
                next,
            },
            node,
        )
    }

    pub fn with_commands<'a>(self, commands: &'a mut Commands<'a>) -> NodeBuilder<'a, T> {
        NodeBuilder {
            node: self.node,
            next: self.next,
            commands,
        }
    }
}

impl<'a, T> NodeBuilder<'a, T> {
    pub fn add<P>(self, part: P) -> P::Return
    where P: NodePart<NodeBuilder<'a, T>> {
        part.add_to(self)
    }
}

//TODO: implement
#[derive(Bundle)]
struct LineBundle{
    line: Line,
}

struct NextLine(Entity);

impl<'a, T> NodePart<NodeBuilder<'a, T>> for LineBundle {
    type Return = NodeBuilder<'a, T>;

    fn add_to(self, builder: NodeBuilder<'a, T>) -> NodeBuilder<'a, T> {
        let next = builder.commands.spawn().id();
        builder.commands
            .entity(builder.next)
            .insert_bundle(self)
            .insert(NextLine(next));

        NodeBuilder{
            next,
            ..builder
        }
    }
}

//TODO: implement
struct JumpChoice;
struct TunnelChoice;

impl JumpChoice {
    fn bundle(self) -> () {
        ()
    }
}

impl TunnelChoice {
    fn bundle(self, next: Entity) -> () {
        ()
    }
}

impl<'a, T> NodePart<NodeBuilder<'a, T>> for TunnelChoice {
    type Return = NodeBuilder<'a, T>;

    fn add_to(self, builder: NodeBuilder<'a, T>) -> NodeBuilder<'a, T> {
        let next = builder.commands.spawn().id();
        builder.commands
            .entity(builder.next)
            .insert_bundle(self.bundle(next));

        NodeBuilder{
            next,
            ..builder
        }
    }
}

impl<'a> NodePart<NodeBuilder<'a, TreeNode>> for JumpChoice {
    type Return = ();

    fn add_to(self, builder: NodeBuilder<'a, TreeNode>) -> () {
        builder.commands
            .entity(builder.next)
            .insert_bundle(self.bundle());
    }
}

struct Return;

impl<'a> NodeBuilder<'a, TunnelNode> {
    pub fn tunnel_return(self) -> () {
        self.commands
            .entity(self.next)
            .insert(Return);
    }
}

struct GameOver;

impl<'a> NodeBuilder<'a, TreeNode> {
    pub fn game_over(self) -> () {
        self.commands
            .entity(self.next)
            .insert(GameOver);
    }
}

#[derive(Copy, Clone)]
pub struct TreeNode(Entity);

impl Deref for TreeNode {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Entity> for TreeNode {
    fn from(entity: Entity) -> Self {
        TreeNode(entity)
    }
}

#[derive(Copy, Clone)]
pub struct TunnelNode(Entity);

impl Deref for TunnelNode {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Entity> for TunnelNode {
    fn from(entity: Entity) -> Self {
        TunnelNode(entity)
    }
}

pub struct Line {
    pub text: String,
    pub priority: i32,
    pub duration: Duration,
    pub audio: Option<Handle<AudioSource>>,
    pub music: Option<Handle<AudioSource>>,
    pub stop_audio: bool,
    pub stop_music: bool,
    pub repeatable: bool,
    pub responds_to_concepts: Vec<Entity>,
    pub groups: Vec<Entity>,
    pub animations: Vec<String>,
    pub question: Option<String>,
    pub clear_question: bool,
    pub starts_animations: Vec<String>,
    pub ends_animations: Vec<String>,
    pub requires_concepts: Vec<Entity>,
    pub consumes_concepts: Vec<Entity>,
    pub requires_any_concept: bool,
    pub consumes_all_concepts: bool,
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
            music: None,
            stop_audio: false,
            stop_music: false,
            repeatable: false,
            responds_to_concepts: vec![],
            groups: vec![],
            animations: vec![],
            question: None,
            clear_question: false,
            starts_animations: vec![],
            ends_animations: vec![],
            requires_concepts: vec![],
            consumes_concepts: vec![],
            requires_any_concept: false,
            consumes_all_concepts: false,
            requires_spoken: vec![],
            conflicts_spoken: vec![],
        }
    }
}

pub struct Spoken(Vec<Duration>);

pub struct Speaking;

pub struct AnimationStartEvent(pub String);
pub struct AnimationEndEvent(pub String);

pub fn progress_dialogue(
    speaking_query: Query<(Entity, &Line, &Spoken), With<Speaking>>,
    lines_query: Query<(Entity, &Line)>,
    spoken_query: Query<&Spoken>,
    concept_query: Query<Entity, With<Evoked>>,
    time: Res<Time>,
    mut commands: Commands,
    mut ev_play: EventWriter<PlayAudioEvent>,
    mut ev_stop: EventWriter<StopAudioEvent>,
    mut start_event_writer: EventWriter<AnimationStartEvent>,
    mut end_event_writer: EventWriter<AnimationEndEvent>,
    mut clear_question_event_writer: EventWriter<ClearQuestionEvent>,
    mut set_question_event_writer: EventWriter<SetQuestionEvent>,
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
                    info!("End {}", animation);
                    end_event_writer.send(AnimationEndEvent(animation.clone()));
                }
                for animation in &line.ends_animations {
                    info!("End {}", animation);
                    end_event_writer.send(AnimationEndEvent(animation.clone()));
                }

                if line.clear_question {
                    clear_question_event_writer.send(ClearQuestionEvent);
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

            if line.requires_any_concept {
                if let None = concept_query.iter().next() {
                    return false
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
            ev_play.send(PlayAudioEvent { channel: Channel::Dialogue, handle: audio_handle.clone() } );
        }
        if let Some(music_handle) = &line.music {
            ev_play.send(PlayAudioEvent { channel: Channel::Music, handle: music_handle.clone() } );
        }
        if line.stop_audio {
            ev_stop.send(StopAudioEvent { channel: Channel::Dialogue } );
        }
        if line.stop_music {
            ev_stop.send(StopAudioEvent { channel: Channel::Music } );
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
            info!("Start {}", animation);
            start_event_writer.send(AnimationStartEvent(animation.clone()));
        }
        for animation in &line.starts_animations {
            info!("Start {}", animation);
            start_event_writer.send(AnimationStartEvent(animation.clone()));
        }

        if let Some(question) = &line.question {
            set_question_event_writer.send(SetQuestionEvent(question.clone()));
        }

        // Consume concepts
        for concept in &line.consumes_concepts {
            commands
                .entity(*concept)
                .remove::<Evoked>();
        }
        if line.consumes_all_concepts {
            for concept in concept_query.iter() {
                commands
                    .entity(concept)
                    .remove::<Evoked>();
            }
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
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                max_size: Size {
                    width: Val::Px(1280. - 2. * 15.),
                    height: Val::Px(200.0),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TextBox {
            style: TextStyle {
                font: asset_server.load("GloriaHallelujah-Regular.ttf"),
                font_size: 30.0,
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
