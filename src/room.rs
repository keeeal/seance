use crate::animation::{animation_bundle, AnimationDefinition, BLINK_ANIMATION, TALK_ANIMATION};
use crate::concepts::{Concept, EvokesConcept, ConceptFilter};
use crate::ghost::{ghost_bundle, Clickable};
use crate::dialogue::{new_node, LineBundle, NodeBuilder, Choice, Jump, Question, Answer, Speaker, Music, Return, Clear};
use bevy::prelude::{
    AppBuilder, AssetServer, Assets, Commands, IntoSystem, OrthographicCameraBundle, Plugin, Res,
    ResMut, SpriteSheetBundle, TextureAtlas, Transform, Vec2, Vec3,
};
use bevy::sprite::Rect;
use bevy_interact_2d::{InteractionSource, Interactable, Group};
use std::time::Duration;

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut cam_bundle = OrthographicCameraBundle::new_2d();
    cam_bundle.orthographic_projection.scale = 3.;

    commands
        .spawn_bundle(cam_bundle)
        .insert(InteractionSource::default());

    let click_group = Group(0);
    // load background
    let background_texture = asset_server.load("background.png");
    let background_atlas = texture_atlases.add(TextureAtlas::from_grid(
        background_texture,
        Vec2::new(3840., 2160.),
        1,
        1,
    ));

    let _background = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: background_atlas,
            transform: Transform::from_xyz(1., 0., 0.),
            ..Default::default()
        })
        .id();

    let norman_concept = commands
        .spawn()
        .insert(Concept{
            description: "Norman Willoughby".to_string(),
            parents: vec![],
        })
        .id();

    let music_box_concept = commands
        .spawn()
        .insert(Concept{
            description: "Jewellery Box".to_string(),
            parents: vec![],
        })
        .id();

    let rocking_horse_concept = commands
        .spawn()
        .insert(Concept{
            description: "Rocking Horse".to_string(),
            parents: vec![],
        })
    .id();

    // load medium
    let medium_texture = asset_server.load("characters/medium.png");
    let medium_atlas = texture_atlases.add(TextureAtlas::from_grid(
        medium_texture,
        Vec2::new(250.0, 700.0),
        3,
        3,
    ));

    let medium_talk_frames = vec![0, 1, 2];
    let medium_blink_frames = vec![3, 4, 5];
    let _medium = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: medium_atlas,
            transform: Transform::from_xyz(20., -760., 0.),
            ..Default::default()
        })
        .insert_bundle(animation_bundle(
            (BLINK_ANIMATION, medium_blink_frames),
            [(
                "medium_talk".to_string(),
                (TALK_ANIMATION, medium_talk_frames),
            )]
            .iter()
            .cloned()
            .collect(),
        ))
        .id();

    let madam_gretchen_talk = "medium_talk";

    let madam_gretchen = Speaker {
        talk_animation: madam_gretchen_talk,
    };

    // load mother
    let mother_texture = asset_server.load("characters/mother.png");
    let mother_atlas = texture_atlases.add(TextureAtlas::from_grid(
        mother_texture,
        Vec2::new(320.0, 505.0),
        3,
        4,
    ));

    let mother_default_frames = vec![0, 1, 2];
    let mother_happy_frames = vec![3, 4, 5];
    let mother_scared_frames = vec![6, 7, 8];
    let mother_talk_frames = vec![9, 10, 11];
    let mother_leave_frames = vec![12];
    let _mother = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: mother_atlas,
            transform: Transform::from_xyz(-422., -833., 0.),
            ..Default::default()
        })
        .insert_bundle(animation_bundle(
            (BLINK_ANIMATION, mother_default_frames),
            [
                (
                    "mother_happy".to_string(),
                    (BLINK_ANIMATION, mother_happy_frames),
                ),
                (
                    "mother_scared".to_string(),
                    (BLINK_ANIMATION, mother_scared_frames),
                ),
                (
                    "mother_talk".to_string(),
                    (TALK_ANIMATION, mother_talk_frames),
                ),
                (
                    "mother_leave".to_string(),
                    (AnimationDefinition::Simple, mother_leave_frames),
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        ))
        .id();

    // load twin1
    let twin1_texture = asset_server.load("characters/twin1.png");
    let twin1_atlas = texture_atlases.add(TextureAtlas::from_grid(
        twin1_texture,
        Vec2::new(320.0, 640.0),
        3,
        4,
    ));

    let twin1_default_frames = vec![0, 1, 2];
    let twin1_happy_frames = vec![3, 4, 5];
    let twin1_scared_frames = vec![6, 7, 8];
    let twin1_talk_frames = vec![9, 10, 11];
    let twin1_leave_frames = vec![12];
    let _twin1 = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: twin1_atlas,
            transform: Transform::from_xyz(-201., -770., 0.),
            ..Default::default()
        })
        .insert_bundle(animation_bundle(
            (BLINK_ANIMATION, twin1_default_frames),
            [
                (
                    "twin1_happy".to_string(),
                    (BLINK_ANIMATION, twin1_happy_frames),
                ),
                (
                    "twin1_scared".to_string(),
                    (BLINK_ANIMATION, twin1_scared_frames),
                ),
                (
                    "twin1_talk".to_string(),
                    (TALK_ANIMATION, twin1_talk_frames),
                ),
                (
                    "twin1_leave".to_string(),
                    (AnimationDefinition::Simple, twin1_leave_frames),
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        ))
        .id();

    // load twin2
    let twin2_texture = asset_server.load("characters/twin2.png");
    let twin2_atlas = texture_atlases.add(TextureAtlas::from_grid(
        twin2_texture,
        Vec2::new(320.0, 640.0),
        3,
        4,
    ));

    let twin2_default_frames = vec![0, 1, 2];
    let twin2_happy_frames = vec![3, 4, 5];
    let twin2_scared_frames = vec![6, 7, 8];
    let twin2_talk_frames = vec![9, 10, 11];
    let twin2_leave_frames = vec![12];
    let _twin2 = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: twin2_atlas,
            transform: Transform::from_xyz(287., -779., 0.),
            ..Default::default()
        })
        .insert_bundle(animation_bundle(
            (BLINK_ANIMATION, twin2_default_frames),
            [
                (
                    "twin2_happy".to_string(),
                    (BLINK_ANIMATION, twin2_happy_frames),
                ),
                (
                    "twin2_scared".to_string(),
                    (BLINK_ANIMATION, twin2_scared_frames),
                ),
                (
                    "twin2_talk".to_string(),
                    (TALK_ANIMATION, twin2_talk_frames),
                ),
                (
                    "twin2_leave".to_string(),
                    (AnimationDefinition::Simple, twin2_leave_frames),
                ),

            ]
            .iter()
            .cloned()
            .collect(),
        ))
        .id();

    // load ghost
    let ghost_texture = asset_server.load("ghost.png");
    let ghost_atlas = texture_atlases.add(TextureAtlas::from_grid(
        ghost_texture,
        Vec2::new(128., 128.),
        1,
        1,
    ));

    let _ghost = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: ghost_atlas,
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .insert_bundle(ghost_bundle())
        .id();

    // load static objects
    let stationary_textures = asset_server.load("objects/stationary.png");
    // frame
    let mut frame_atlas =
        TextureAtlas::new_empty(stationary_textures.clone(), Vec2::new(960., 960.));
    frame_atlas.add_texture(Rect {
        min: Vec2::new(0., 0.),
        max: Vec2::new(7., 8.) * 24.,
    });
    frame_atlas.add_texture(Rect {
        min: Vec2::new(7., 8.) * 24.,
        max: Vec2::new(14., 16.) * 24.,
    });
    let frame_atlas_id = texture_atlases.add(frame_atlas);

    let _frame1 = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: frame_atlas_id.clone(),
            transform: (Transform::from_xyz(-900., 0., 0.) * Transform::from_scale(Vec3::new(2., 2., 2.))),
            ..Default::default()
        })
        .insert(Interactable {
            bounding_box: (Vec2::new(-168., -192.), Vec2::new(168., 192.)),
            groups: vec![click_group],
        })
        .insert(Clickable)
        .id();

    let _frame2 = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: frame_atlas_id.clone(),
            transform: (Transform::from_xyz(-450., 0., 0.) * Transform::from_scale(Vec3::new(2., 2., 2.))),
            ..Default::default()
        })
        .insert(Interactable {
            bounding_box: (Vec2::new(-168., -192.), Vec2::new(168., 192.)),
            groups: vec![click_group],
        })
        .insert(Clickable)
        .id();

    let _frame3 = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: frame_atlas_id.clone(),
            transform: (Transform::from_xyz(20., 60., 0.) * Transform::from_scale(Vec3::new(2., 2., 2.))),
            ..Default::default()
        })
        .insert(Interactable {
            bounding_box: (Vec2::new(-168., -192.), Vec2::new(168., 192.)),
            groups: vec![click_group],
        })
        .insert(Clickable)
        .insert(EvokesConcept(norman_concept))
        .id();

    // frame
    let mut musicbox_atlas =
        TextureAtlas::new_empty(stationary_textures.clone(), Vec2::new(960., 960.));
    musicbox_atlas.add_texture(Rect {
        min: Vec2::new(9., 13.) * 24.,
        max: Vec2::new(14., 18.) * 24.,
    });
    musicbox_atlas.add_texture(Rect {
        min: Vec2::new(14., 13.) * 24.,
        max: Vec2::new(19., 18.) * 24.,
    });
    let musicbox_atlas_id = texture_atlases.add(musicbox_atlas);

    let _music_box = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: musicbox_atlas_id.clone(),
            transform: (Transform::from_xyz(1292., -444., 0.) * Transform::from_scale(Vec3::new(2., 2., 2.))),
            ..Default::default()
        })
        .insert(Interactable {
            bounding_box: (Vec2::new(-1.5 * 24., -1.5 * 24.), Vec2::new(1.6 * 24., 1.7 * 24.)),
            groups: vec![click_group],
        })
        .insert(Clickable)
        .insert(EvokesConcept(music_box_concept))
        .id();

    let rockinghorse_texture = asset_server.load("objects/rocking_horse.png");
    let rockinghorse_atlas = texture_atlases.add(TextureAtlas::from_grid(
        rockinghorse_texture,
        Vec2::new(238., 250.),
        3,
        2,
    ));

    let _rocking_horse = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: rockinghorse_atlas,
            transform: (Transform::from_xyz(828., -612., 0.) * Transform::from_scale(Vec3::new(1., 1., 1.))),
            ..Default::default()
        })
        .insert(Interactable {
            bounding_box: (Vec2::new(-119., -125.), Vec2::new(119., 125.)),
            groups: vec![click_group],
        })
        .insert(Clickable)
        .insert(EvokesConcept(rocking_horse_concept))
        .id();


    // load narrator overlay
    let overlay_texture = asset_server.load("blackScreen.png");
    let overlay_atlas = texture_atlases.add(TextureAtlas::from_grid(
        overlay_texture,
        Vec2::new(1920., 1280.),
        2,
        1,
    ));

    let vignette_frames = vec![1];
    let narrator_frames = vec![0];
    let _overlay = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: overlay_atlas,
            transform: (
                Transform::from_xyz(0., 0., 10.)
                * Transform::from_scale(Vec3::new(3., 3., 3.))
            ),
            ..Default::default()
        })
        .insert_bundle(animation_bundle(
            (AnimationDefinition::Simple, vignette_frames),
            [(
                "narrator_talk".to_string(),
                (AnimationDefinition::Simple, narrator_frames),
            )]
            .iter()
            .cloned()
            .collect(),
        ))
        .id();

    let narrator_talk = "narrator_talk";

    let narrator = Speaker {
        talk_animation: narrator_talk,
    };

    let introduction_music = Music(asset_server.load("BGM_SC1_Introduction.mp3"));
    let piano_music = Music(asset_server.load("Repeating_Piano_Theme.mp3"));

    let s = Duration::from_secs(3);

    let (s1_introduction_builder, s1_introduction) = new_node(&mut commands);
    let (s1_q1_builder, s1_q1) = new_node(&mut commands);
    let (s1_q2_builder, s1_q2) = new_node(&mut commands);
    let (s1_a2_builder, s1_a2) = new_node(&mut commands);
    let (s1_q3_builder, s1_q3) = new_node(&mut commands);

    s1_introduction_builder
        .add(
            LineBundle::blank()
                .with(3 * s)
                .with(&narrator_talk)
                .with(&introduction_music),
            &mut commands,
        )
        .add(
            LineBundle::dialogue(narrator, concat!(
                "The world is cold and dark as you wander the halls of a home you used ",
                "to find great comfort in. Your desire to leave it all behind is ",
                "palpable but still something keeps you here. The presence of the ones ",
                "you love. You see your daughters crying and your wife sitting quietly ",
                "on the bed you share. You reach out but no one notices. Even the ",
                "mirror on the wall refuses to portray your presence. Is this a dream?",
            ))
            .with("dialogue/NAR.S1.Introduction.A.mp3"),
            &mut commands,
        )
        .add(
            LineBundle::dialogue(narrator, concat!(
                "Unconnected to the passage of time you watch strange happenings scare ",
                "your family. Are you responsible? Why can’t you leave?",
            ))
            .with("dialogue/NAR.S1.Introduction.B.mp3"),
            &mut commands,
        )
        .add(
            LineBundle::dialogue(narrator, concat!(
                "Suddenly, a warm light draws you to your living room. Your family is ",
                "congregated around the dining table with an old friend, a medium, ",
                "Madam Gretchen. A seat sits empty beckoning you into the circle.",
            ))
            .with("dialogue/NAR.S1.Introduction.C.mp3"),
            &mut commands,
        )
        .add(Jump(s1_q1), &mut commands);

    s1_q1_builder
        .add(
            LineBundle::blank()
                .with(5 * s)
                .with(&piano_music),
            &mut commands,
        )
        .add(
            LineBundle::dialogue(madam_gretchen, concat!(
                "As we join hands we focus our wills, Joining together the worlds of ",
                "the dead and the living. We are reaching out to whoever haunts this ",
                "place.",
            )),
            &mut commands,
        )
        .add(
            LineBundle::dialogue(madam_gretchen, concat!(
                "Is someone here? If there is someone with us, give us a sign?",
            ))
            .with(Question("If there is someone with us, give us a sign?")),
            &mut commands,
        )
        .add(
            LineBundle::dialogue(narrator, concat!(
                "The question coupled by the warm light strengthens your resolve.",
            ))
            .with("dialogue/NAR.S1.Q1.mp3"),
            &mut commands,
        )
        .add(
            Choice::new()
                .option(ConceptFilter::Any,
                    NodeBuilder::new(&mut commands)
                        .add(
                            LineBundle::dialogue(madam_gretchen, concat!(
                                "Madam Gretchen: Ah yes, I can see you still have some influence on the material plane. This will help us communicate.",
                            ))
                            .with(Answer),
                            &mut commands,
                        )
                        .add(Return, &mut commands)
                ),
            &mut commands,
        )
        .add(Jump(s1_q2), &mut commands);

    s1_q2_builder
        .add(
            LineBundle::blank()
                .with(1 * s),
            &mut commands,
        )
        .add(
            LineBundle::dialogue(madam_gretchen, concat!(
                "Who are you?",
            ))
            .with(Question("Who are you?")),
            &mut commands,
        )
        .add(
            LineBundle::dialogue(narrator, concat!(
                "“Norm! It’s me Norm” You call out but no sound ",
                "breaks the air. A portrait of a young and handsome Norman ",
                "sits boldly above the fireplace.",
            ))
            .with("dialogue/NAR.S1.Q2.mp3"),
            &mut commands,
        )
        .add(Jump(s1_a2), &mut commands);

    s1_a2_builder
        .add(
            Choice::new()
                .option(ConceptFilter::Concept(norman_concept),
                    NodeBuilder::new(&mut commands)
                        .add(
                            LineBundle::dialogue(madam_gretchen, concat!(
                                "I believe it is Norman who is with us.",
                            ))
                            .with(Answer),
                            &mut commands,
                        )
                        .add(Jump(s1_q3), &mut commands)
                )
                .option(ConceptFilter::Any, // TODO: repeatable
                    NodeBuilder::new(&mut commands)
                        .add(
                            LineBundle::dialogue(madam_gretchen, concat!(
                                "Hmm, that doesn't seem right.",
                            ))
                            .with(Clear(ConceptFilter::Any)),
                            &mut commands,
                        )
                        .add(Jump(s1_a2), &mut commands)
                ),
            &mut commands,
        );

    // let s1_margaret_q3_a = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Margaret: Wait, if it is Norm I want some proof...",
    //             ).to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(6),
    //             animations: vec!["mother_talk".to_string()],
    //             requires_spoken: vec![s1_medium_q2],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_margaret_q3_b = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Margaret: Norm, What did you make here for our daughters’ third birthday?",
    //             ).to_string(),
    //             priority: 5,
    //             question: Some("What did you make for our daughters’ third birthday?".to_string()),
    //             duration: Duration::from_secs(7),
    //             animations: vec!["mother_talk".to_string()],
    //             requires_spoken: vec![s1_margaret_q3_a],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_narrator_q3 = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Narrator: A memory of sitting by the fireplace on a cold, winter’s morning as ",
    //                 "your two daughters unwrap a handcrafted jewellery box plays in your mind.",
    //             ).to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(13),
    //             animations: vec!["narrator_talk".to_string()],
    //             requires_spoken: vec![s1_margaret_q3_b],
    //             audio: Some(asset_server.load("dialogue/NAR.S1.Q3.1.mp3")),
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_q3_pause = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: "".to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(1),
    //             requires_spoken: vec![s1_narrator_q3],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_narrator_a3_a = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Narrator: As one of your daughters opens the lid, music starts to ",
    //                 "play. The girls grimace but your wife smiles and a tear rolls down ",
    //                 "her face. She has heard this song before.",
    //             ).to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(4),
    //             clear_question: true,
    //             starts_animations: vec!["narrator_talk".to_string()],
    //             requires_spoken: vec![s1_q3_pause],
    //             requires_concepts: vec![music_box_concept],
    //             consumes_concepts: vec![music_box_concept],
    //             audio: Some(asset_server.load("dialogue/NAR.S1.Q3.2.mp3")),
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_narrator_a3 = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Narrator: As one of your daughters opens the lid, music starts to ",
    //                 "play. The girls grimace but your wife smiles and a tear rolls down ",
    //                 "her face. She has heard this song before.",
    //             ).to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(13),
    //             ends_animations: vec!["narrator_talk".to_string()],
    //             music: Some(asset_server.load("Music_Box_Sound.mp3")),
    //             requires_spoken: vec![s1_narrator_a3_a],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_margaret_a3_a = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: "Margaret: It’s really him. I used to sing this... Sniff *Looks down and tears*".to_string(),
    //             priority: 5,
    //             stop_audio: true,
    //             duration: Duration::from_secs(4),
    //             animations: vec!["mother_talk".to_string()],
    //             requires_spoken: vec![s1_narrator_a3],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_margaret_a3_b = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: "Margaret: It’s really him. I used to sing this... Sniff *Looks down and tears*".to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(3),
    //             starts_animations: vec!["mother_scared".to_string()],
    //             requires_spoken: vec![s1_margaret_a3_a],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_jasmine_a3_a = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: "*Jasmine jumps, and seems spooked by the music box*".to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(5),
    //             starts_animations: vec!["twin1_scared".to_string()],
    //             requires_spoken: vec![s1_margaret_a3_b],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_melina_a3_a = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: "Melina: Its ok its just dad".to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(5),
    //             animations: vec!["twin2_talk".to_string()],
    //             requires_spoken: vec![s1_jasmine_a3_a],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_jasmine_a3_b = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: "Jasmine: How do you know? I don’t remember my third birthday. Do you?".to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(7),
    //             animations: vec!["twin1_talk".to_string()],
    //             requires_spoken: vec![s1_melina_a3_a],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_melina_a3_b = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: "Melina: Well, ask something.".to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(4),
    //             animations: vec!["twin2_talk".to_string()],
    //             requires_spoken: vec![s1_jasmine_a3_b],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_jasmine_q4 = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: "Jasmine: Ok what was Melina’s favourite toy?".to_string(),
    //             priority: 5,
    //             question: Some("What was Melina’s favourite toy?".to_string()),
    //             music: Some(asset_server.load("Repeating_Piano_Theme.mp3")),
    //             duration: Duration::from_secs(6),
    //             animations: vec!["twin1_talk".to_string()],
    //             requires_spoken: vec![s1_melina_a3_b],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_narrator_q4 = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Narrator: Another memory plays. Melina forcefully rocks up and ",
    //                 "down the hallway on a rocking horse while Jasmine slides behind ",
    //                 "her tethered by a rope lasso. Jasmine seems utterly unimpressed ",
    //                 "by her capture, soon to be jailed in the bedroom.",
    //             ).to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(21),
    //             animations: vec!["narrator_talk".to_string()],
    //             requires_spoken: vec![s1_jasmine_q4],
    //             audio: Some(asset_server.load("dialogue/NAR.S1.Q4.mp3")),
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_melina_a4 = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Melina: Well it’s him alright. Why are you scaring us dad? Don't ",
    //                 "you like us anymore? Is it another one of your tests!?"
    //             ).to_string(),
    //             priority: 5,
    //             question: Some("Why are you scaring us?".to_string()),
    //             duration: Duration::from_secs(4),
    //             animations: vec!["twin2_talk".to_string()],
    //             requires_concepts: vec![rocking_horse_concept],
    //             consumes_concepts: vec![rocking_horse_concept],
    //             requires_spoken: vec![s1_narrator_q4],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let _s1_jasmine_a4_wrong = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: "Jasmine: No, that's wrong. I don't think it's him!".to_string(),
    //             priority: -5,
    //             duration: Duration::from_secs(6),
    //             animations: vec!["twin1_talk".to_string()],
    //             requires_any_concept: true,
    //             consumes_all_concepts: true,
    //             requires_spoken: vec![s1_narrator_q4],
    //             conflicts_spoken: vec![s1_melina_a4],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_narrator_q5 = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Narrator: You wish to console your daughter about the happenings ",
    //                 "but are unsure how to communicate. After all, you cannot talk. ",
    //                 "You feel another memory start to stir but before you can catch it is gone.",
    //             ).to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(19),
    //             animations: vec!["narrator_talk".to_string()],
    //             requires_spoken: vec![s1_melina_a4],
    //             audio: Some(asset_server.load("dialogue/NAR.S1.Q5.mp3")),
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_medium_a5_a = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Madam Gretchen: We may have to build up to that one Melina. I ",
    //                 "sense he doesn't know how to answer. Let's try to help him with ",
    //                 "objects he remembers from his life. They are easier for spirits ",
    //                 "to interact with.",
    //             ).to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(10),
    //             animations: vec!["medium_talk".to_string()],
    //             requires_spoken: vec![s1_narrator_q5],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_jasmine_a5 = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Jasmine: Why is he so weak now? He had no problem tipping over our ",
    //                 "bug collection in the loft!",
    //             ).to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(10),
    //             animations: vec!["twin1_talk".to_string()],
    //             consumes_all_concepts: true,
    //             clear_question: true,
    //             requires_spoken: vec![s1_medium_a5_a],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_medium_a5_b = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Madam Gretchen: Patience, all shall be revealed in time.",
    //             ).to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(6),
    //             animations: vec!["medium_talk".to_string()],
    //             requires_spoken: vec![s1_jasmine_a5],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_medium_a5_c = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Madam Gretchen: Now get something that represents the earth, and physicality. Maybe a coin.",
    //             ).to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(7),
    //             animations: vec!["medium_talk".to_string()],
    //             requires_spoken: vec![s1_medium_a5_b],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_margaret_a5 = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Margaret: I know I have a coin collection built up from his various ",
    //                 "trips away. He always brought back a new coin from everywhere he visited.",
    //             ).to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(12),
    //             animations: vec!["mother_talk".to_string()],
    //             requires_spoken: vec![s1_medium_a5_c],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_medium_a5_d = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Madam Gretchen: Excellent, go and get it and bring it into the room.",
    //             ).to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(7),
    //             animations: vec!["medium_talk".to_string()],
    //             requires_spoken: vec![s1_margaret_a5],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_medium_a5_e = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Madam Gretchen: Now girls, we need a cup. Maybe Norm’s favourite glass... ",
    //                 "or mug? and a book void of writing.",
    //             ).to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(8),
    //             animations: vec!["medium_talk".to_string()],
    //             starts_animations: vec!["mother_leave".to_string()],
    //             requires_spoken: vec![s1_medium_a5_d],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let s1_jasmine_and_melina_a5 = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "Jasmine and Melina: Yea we got it.",
    //             ).to_string(),
    //             priority: 5,
    //             duration: Duration::from_secs(7),
    //             animations: vec!["twin1_talk".to_string(), "twin2_talk".to_string()],
    //             requires_spoken: vec![s1_medium_a5_e],
    //             ..Default::default()
    //         }
    //     )
    //     .id();

    // let _s1_end = commands
    //     .spawn()
    //     .insert(
    //         Line {
    //             text: concat!(
    //                 "",
    //             ).to_string(),
    //             question: Some("This is the end of act 1. Acts 2 and 3 are not yet playable.".to_string()),
    //             priority: 5,
    //             duration: Duration::from_secs(1),
    //             starts_animations: vec!["twin1_leave".to_string(), "twin2_leave".to_string()],
    //             requires_spoken: vec![s1_jasmine_and_melina_a5],
    //             ..Default::default()
    //         }
    //     )
    //     .id();
}

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system());
    }
}
