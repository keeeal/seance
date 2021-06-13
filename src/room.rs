use crate::concepts::{Concept, EvokesConcept};
use crate::ghost::ghost_bundle;
use crate::animation::{animation_bundle, AnimationDefinition, TALK_ANIMATION, BLINK_ANIMATION};
use crate::dialogue::{Line};
use bevy::prelude::{
    AppBuilder, AssetServer, Assets, Commands, IntoSystem, OrthographicCameraBundle, Plugin, Res,
    ResMut, SpriteSheetBundle, TextureAtlas, Transform, Vec2, Vec3,
};
use bevy_interact_2d::InteractionSource;
use std::time::Duration;

pub fn startup(
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

    let music_box = commands
        .spawn()
        .insert(Concept{
            description: "Jewellery Box".to_string(),
            parents: vec![],
        })
        .id();

    let rocking_horse = commands
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
            [("medium_talk".to_string(), (TALK_ANIMATION, medium_talk_frames))].iter().cloned().collect(),
        ))
        .id();

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
            [("mother_happy".to_string(), (BLINK_ANIMATION, mother_happy_frames)),
             ("mother_scared".to_string(), (BLINK_ANIMATION, mother_scared_frames)),
             ("mother_talk".to_string(), (TALK_ANIMATION, mother_talk_frames)),
             ("mother_leave".to_string(), (AnimationDefinition::Simple, mother_leave_frames))].iter().cloned().collect(),
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
            [("twin1_happy".to_string(), (BLINK_ANIMATION, twin1_happy_frames)),
             ("twin1_scared".to_string(), (BLINK_ANIMATION, twin1_scared_frames)),
             ("twin1_leave".to_string(), (AnimationDefinition::Simple, twin1_leave_frames)),
             ("twin1_talk".to_string(), (TALK_ANIMATION, twin1_talk_frames))].iter().cloned().collect(),
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
            [("twin2_happy".to_string(), (BLINK_ANIMATION, twin2_happy_frames)),
             ("twin2_scared".to_string(), (BLINK_ANIMATION, twin2_scared_frames)),
             ("twin2_leave".to_string(), (AnimationDefinition::Simple, twin2_leave_frames)),
             ("twin2_talk".to_string(), (TALK_ANIMATION, twin2_talk_frames))].iter().cloned().collect(),
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
            [("narrator_talk".to_string(), (AnimationDefinition::Simple, narrator_frames))].iter().cloned().collect(),
        ))
        .id();

    let s1_pause = commands
        .spawn()
        .insert(
            Line {
                text: "".to_string(),
                priority: 5,
                duration: Duration::from_secs(3),
                ..Default::default()
            }
        )
        .id();

    let s1_introduction_a = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Narrator: The world is cold and dark as you wander the halls of a home you used ",
                    "to find great comfort in. Your desire to leave it all behind is ",
                    "palpable but still something keeps you here. The presence of the ones ",
                    "you love. You see your daughters crying and your wife sitting quietly ",
                    "on the bed you share. You reach out but no one notices. Even the ",
                    "mirror on the wall refuses to portray your presence. Is this a dream?",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(40),
                starts_animations: vec!["narrator_talk".to_string()],
                requires_spoken: vec![s1_pause],
                audio: Some(asset_server.load("dialogue/NAR.S1.Introduction.mp3")),
                ..Default::default()
            }
        )
        .id();
    let s1_introduction_b = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Narrator: Unconnected to the passage of time you watch strange happenings scare ",
                    "your family. Are you responsible? Why can’t you leave?",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(15),
                requires_spoken: vec![s1_introduction_a],
                ..Default::default()
            }
        )
        .id();
    let s1_introduction_c = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Narrator: Suddenly, a warm light draws you to your living room. Your family is ",
                    "congregated around the dining table with an old friend, a medium, ",
                    "Madam Gretchen. A seat sits empty beckoning you into the circle.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(19),
                ends_animations: vec!["narrator_talk".to_string()],
                requires_spoken: vec![s1_introduction_b],
                ..Default::default()
            }
        )
        .id();

    let s1_introduction_pause = commands
        .spawn()
        .insert(
            Line {
                text: "".to_string(),
                priority: 5,
                duration: Duration::from_secs(5),
                requires_spoken: vec![s1_introduction_c],
                ..Default::default()
            }
        )
        .id();

    let s1_medium_q1_a = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Madam Gretchen: As we join hands we focus our wills, Joining together the worlds of ",
                    "the dead and the living. We are reaching out to whoever haunts this ",
                    "place.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(13),
                starts_animations: vec!["medium_talk".to_string()],
                requires_spoken: vec![s1_introduction_pause],
                ..Default::default()
            }
        )
        .id();
    let s1_medium_q1_b = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Madam Gretchen: Is someone here? If there is someone with us, give us a sign?",
                ).to_string(),
                priority: 5,
                question: Some("If there is someone with us, give us a sign?".to_string()),
                duration: Duration::from_secs(7),
                ends_animations: vec!["medium_talk".to_string()],
                requires_spoken: vec![s1_medium_q1_a],
                ..Default::default()
            }
        )
        .id();

    let s1_narrator_q1_a = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Narrator: The question coupled by the warm light strengthens your resolve.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(6),
                requires_spoken: vec![s1_medium_q1_b],
                animations: vec!["narrator_talk".to_string()],
                audio: Some(asset_server.load("dialogue/NAR.S1.Q1.mp3")),
                ..Default::default()
            }
        )
        .id();

    let s1_medium_a1 = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Madam Gretchen: Ah yes, I can see you still have some influence on the material plane. This will help us communicate.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(5),
                requires_spoken: vec![s1_narrator_q1_a],
                requires_any_concept: true,
                consumes_all_concepts: true,
                clear_question: true,
                animations: vec!["medium_talk".to_string()],
                ..Default::default()
            }
        )
        .id();

    let s1_q2_pause = commands
        .spawn()
        .insert(
            Line {
                text: "".to_string(),
                priority: 5,
                duration: Duration::from_secs(1),
                requires_spoken: vec![s1_medium_a1],
                ..Default::default()
            }
        )
        .id();

    let s1_medium_q2 = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Madam Gretchen: Who are you?",
                ).to_string(),
                priority: 5,
                question: Some("Who are you?".to_string()),
                duration: Duration::from_secs(5),
                animations: vec!["medium_talk".to_string()],
                requires_spoken: vec![s1_q2_pause],
                ..Default::default()
            }
        )
        .id();

    let s1_narrator_q2 = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Narrator: “Norm! It’s me Norm” You call out but no sound ",
                    "breaks the air. A portrait of a young and handsome Norman ",
                    "sits boldly above the fireplace.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(13),
                animations: vec!["narrator_talk".to_string()],
                requires_spoken: vec![s1_medium_q2],
                audio: Some(asset_server.load("dialogue/NAR.S1.Q2.mp3")),
                ..Default::default()
            }
        )
        .id();

    let s1_medium_q2 = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Madam Gretchen: I believe it is Norman who is with us.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(6),
                animations: vec!["medium_talk".to_string()],
                requires_concepts: vec![norman_concept],
                consumes_concepts: vec![norman_concept],
                clear_question: true,
                requires_spoken: vec![s1_narrator_q2],
                ..Default::default()
            }
        )
        .id();

    let _s1_medium_q2_wrong = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Madam Gretchen: Hmm, that doesn't seem right.",
                ).to_string(),
                priority: -5,
                duration: Duration::from_secs(6),
                animations: vec!["medium_talk".to_string()],
                requires_any_concept: true,
                consumes_all_concepts: true,
                repeatable: true,
                requires_spoken: vec![s1_narrator_q2],
                conflicts_spoken: vec![s1_medium_q2],
                ..Default::default()
            }
        )
        .id();

    let s1_margaret_q3_a = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Margaret: Wait, if it is Norm I want some proof...",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(6),
                animations: vec!["mother_talk".to_string()],
                requires_spoken: vec![s1_medium_q2],
                ..Default::default()
            }
        )
        .id();

    let s1_margaret_q3_b = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Margaret: Norm, What did you make here for our daughters’ third birthday?",
                ).to_string(),
                priority: 5,
                question: Some("What did you make for our daughters’ third birthday?".to_string()),
                duration: Duration::from_secs(7),
                animations: vec!["mother_talk".to_string()],
                requires_spoken: vec![s1_margaret_q3_a],
                ..Default::default()
            }
        )
        .id();

    let s1_narrator_q3 = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Narrator: A memory of sitting by the fireplace on a cold, winter’s morning as ",
                    "your two daughters unwrap a handcrafted jewellery box plays in your mind.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(13),
                animations: vec!["narrator_talk".to_string()],
                requires_spoken: vec![s1_margaret_q3_b],
                audio: Some(asset_server.load("dialogue/NAR.S1.Q3.1.mp3")),
                ..Default::default()
            }
        )
        .id();

    let s1_q3_pause = commands
        .spawn()
        .insert(
            Line {
                text: "".to_string(),
                priority: 5,
                duration: Duration::from_secs(1),
                requires_spoken: vec![s1_narrator_q3],
                ..Default::default()
            }
        )
        .id();

    let s1_narrator_a3 = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Narrator: As one of your daughters opens the lid, music starts to ",
                    "play. The girls grimace but your wife smiles and a tear rolls down ",
                    "her face. She has heard this song before.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(17),
                animations: vec!["narrator_talk".to_string()],
                starts_animations: vec!["music_box".to_string()],
                requires_spoken: vec![s1_q3_pause],
                requires_concepts: vec![music_box],
                audio: Some(asset_server.load("dialogue/NAR.S1.Q3.2.mp3")),
                ..Default::default()
            }
        )
        .id();

    let s1_margaret_a3_a = commands
        .spawn()
        .insert(
            Line {
                text: "Margaret: It’s really him. I used to sing this... Sniff *Looks down and tears*".to_string(),
                priority: 5,
                duration: Duration::from_secs(4),
                animations: vec!["mother_talk".to_string()],
                requires_spoken: vec![s1_narrator_a3],
                ..Default::default()
            }
        )
        .id();

    let s1_margaret_a3_b = commands
        .spawn()
        .insert(
            Line {
                text: "Margaret: It’s really him. I used to sing this... Sniff *Looks down and tears*".to_string(),
                priority: 5,
                duration: Duration::from_secs(3),
                starts_animations: vec!["mother_scared".to_string()],
                requires_spoken: vec![s1_margaret_a3_a],
                ..Default::default()
            }
        )
        .id();

    let s1_jasmine_a3_a = commands
        .spawn()
        .insert(
            Line {
                text: "*Jasmine jumps, and seems spooked by the music box*".to_string(),
                priority: 5,
                duration: Duration::from_secs(5),
                starts_animations: vec!["twin1_scared".to_string()],
                requires_spoken: vec![s1_margaret_a3_b],
                ..Default::default()
            }
        )
        .id();

    let s1_melina_a3_a = commands
        .spawn()
        .insert(
            Line {
                text: "Melina: Its ok its just dad".to_string(),
                priority: 5,
                duration: Duration::from_secs(5),
                animations: vec!["twin2_talk".to_string()],
                requires_spoken: vec![s1_jasmine_a3_a],
                ..Default::default()
            }
        )
        .id();

    let s1_jasmine_a3_b = commands
        .spawn()
        .insert(
            Line {
                text: "Jasmine: How do you know? I don’t remember my third birthday. Do you?".to_string(),
                priority: 5,
                duration: Duration::from_secs(7),
                animations: vec!["twin1_talk".to_string()],
                requires_spoken: vec![s1_melina_a3_a],
                ..Default::default()
            }
        )
        .id();

    let s1_melina_a3_b = commands
        .spawn()
        .insert(
            Line {
                text: "Melina: Well, ask something.".to_string(),
                priority: 5,
                duration: Duration::from_secs(4),
                animations: vec!["twin2_talk".to_string()],
                requires_spoken: vec![s1_jasmine_a3_b],
                ..Default::default()
            }
        )
        .id();

    let s1_jasmine_q4 = commands
        .spawn()
        .insert(
            Line {
                text: "Jasmine: Ok what was Melina’s favourite toy?".to_string(),
                priority: 5,
                question: Some("What was Melina’s favourite toy?".to_string()),
                duration: Duration::from_secs(6),
                animations: vec!["twin1_talk".to_string()],
                requires_spoken: vec![s1_melina_a3_b],
                ..Default::default()
            }
        )
        .id();

    let s1_narrator_q4 = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Narrator: Another memory plays. Melina forcefully rocks up and ",
                    "down the hallway on a rocking horse while Jasmine slides behind ",
                    "her tethered by a rope lasso. Jasmine seems utterly unimpressed ",
                    "by her capture, soon to be jailed in the bedroom.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(21),
                animations: vec!["narrator_talk".to_string()],
                requires_spoken: vec![s1_jasmine_q4],
                audio: Some(asset_server.load("dialogue/NAR.S1.Q4.mp3")),
                ..Default::default()
            }
        )
        .id();

    let s1_melina_a4 = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Melina: Well it’s him alright. Why are you scaring us dad? Don't ",
                    "you like us anymore? Is it another one of your tests!?"
                ).to_string(),
                priority: 5,
                question: Some("Why are you scaring us?".to_string()),
                duration: Duration::from_secs(4),
                animations: vec!["twin2_talk".to_string()],
                requires_concepts: vec![rocking_horse],
                consumes_concepts: vec![rocking_horse],
                requires_spoken: vec![s1_narrator_q4],
                ..Default::default()
            }
        )
        .id();

    let _s1_jasmine_a4_wrong = commands
        .spawn()
        .insert(
            Line {
                text: "Jasmine: No, that's wrong. I don't think it's him!".to_string(),
                priority: -5,
                duration: Duration::from_secs(6),
                animations: vec!["twin1_talk".to_string()],
                requires_any_concept: true,
                consumes_all_concepts: true,
                requires_spoken: vec![s1_narrator_q4],
                conflicts_spoken: vec![s1_melina_a4],
                ..Default::default()
            }
        )
        .id();

    let s1_narrator_q5 = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Narrator: You wish to console your daughter about the happenings ",
                    "but are unsure how to communicate. After all, you cannot talk. ",
                    "You feel another memory start to stir but before you can catch it is gone.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(19),
                animations: vec!["narrator_talk".to_string()],
                requires_spoken: vec![s1_melina_a4],
                audio: Some(asset_server.load("dialogue/NAR.S1.Q5.mp3")),
                ..Default::default()
            }
        )
        .id();

    let s1_medium_a5_a = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Madam Gretchen: We may have to build up to that one Melina. I ",
                    "sense he doesn't know how to answer. Let's try to help him with ",
                    "objects he remembers from his life. They are easier for spirits ",
                    "to interact with.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(10),
                animations: vec!["medium_talk".to_string()],
                requires_spoken: vec![s1_narrator_q5],
                ..Default::default()
            }
        )
        .id();

    let s1_jasmine_a5 = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Jasmine: Why is he so weak now? He had no problem tipping over our ",
                    "bug collection in the loft!",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(10),
                animations: vec!["twin1_talk".to_string()],
                consumes_all_concepts: true,
                clear_question: true,
                requires_spoken: vec![s1_medium_a5_a],
                ..Default::default()
            }
        )
        .id();

    let s1_medium_a5_b = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Madam Gretchen: Patience, all shall be revealed in time.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(6),
                animations: vec!["medium_talk".to_string()],
                requires_spoken: vec![s1_jasmine_a5],
                ..Default::default()
            }
        )
        .id();

    let s1_medium_a5_c = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Madam Gretchen: Now get something that represents the earth, and physicality. Maybe a coin.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(7),
                animations: vec!["medium_talk".to_string()],
                requires_spoken: vec![s1_medium_a5_b],
                ..Default::default()
            }
        )
        .id();

    let s1_margaret_a5 = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Margaret: I know I have a coin collection built up from his various ",
                    "trips away. He always brought back a new coin from everywhere he visited.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(12),
                animations: vec!["mother_talk".to_string()],
                requires_spoken: vec![s1_medium_a5_c],
                ..Default::default()
            }
        )
        .id();

    let s1_medium_a5_d = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Madam Gretchen: Excellent, go and get it and bring it into the room.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(7),
                animations: vec!["medium_talk".to_string()],
                requires_spoken: vec![s1_margaret_a5],
                ..Default::default()
            }
        )
        .id();

    let s1_medium_a5_e = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Madam Gretchen: Now girls, we need a cup. Maybe Norm’s favourite glass... ",
                    "or mug? and a book void of writing.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(8),
                animations: vec!["medium_talk".to_string()],
                starts_animations: vec!["mother_leave".to_string()],
                requires_spoken: vec![s1_medium_a5_d],
                ..Default::default()
            }
        )
        .id();

    let s1_jasmine_and_melina_a5 = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "Jasmine and Melina: Yea we got it.",
                ).to_string(),
                priority: 5,
                duration: Duration::from_secs(7),
                animations: vec!["twin1_talk".to_string(), "twin2_talk".to_string()],
                requires_spoken: vec![s1_medium_a5_e],
                ..Default::default()
            }
        )
        .id();

    let s1_end = commands
        .spawn()
        .insert(
            Line {
                text: concat!(
                    "",
                ).to_string(),
                question: Some("This is the end of act 1. Acts 2 and 3 are not yet playable.".to_string()),
                priority: 5,
                duration: Duration::from_secs(1),
                starts_animations: vec!["twin1_leave".to_string(), "twin2_leave".to_string()],
                requires_spoken: vec![s1_jasmine_and_melina_a5],
                ..Default::default()
            }
        )
        .id();
}

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system());
    }
}
