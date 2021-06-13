use crate::concepts::{Concept, Evoked};

use bevy::prelude::{
    AlignSelf, AppBuilder, AssetServer, Color, Commands, EventReader,
    HorizontalAlign, IntoSystem, Plugin, PositionType, Query, Rect, Res, Size, Style, Text,
    TextAlignment, TextBundle, TextSection, TextStyle, Val, VerticalAlign,
};

pub struct QuestionDisplay;

pub struct SetQuestionEvent(pub String);
pub struct ClearQuestionEvent;

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(3. * 5.0),
                    left: Val::Px(3. * 15.0),
                    ..Default::default()
                },
                max_size: Size {
                    width: Val::Px(3840. - 2. * 3. * 15.),
                    height: Val::Px(3. * 200.0),
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text {
                sections: vec![],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            ..Default::default()
        })
        .insert(QuestionDisplay);
}

fn question_system(
    mut text_query: Query<(&QuestionDisplay, &mut Text)>,
    asset_server: Res<AssetServer>,
    mut ev_set: EventReader<SetQuestionEvent>,
    mut ev_clear: EventReader<ClearQuestionEvent>,
    concept_query: Query<(&Concept, &Evoked)>,
) {
    if let Ok((_, mut text)) = text_query.single_mut() {
        for SetQuestionEvent(q) in ev_set.iter() {
            text.sections = vec![TextSection {
                value: q.to_string() + "\n",
                style: TextStyle {
                    font: asset_server.load("GloriaHallelujah-Regular.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            }];
        }

        for ClearQuestionEvent in ev_clear.iter() {
            text.sections = vec![];
        }

        if !text.sections.is_empty() {
            text.sections.truncate(1);
            let mut concepts = concept_query
                .iter()
                .collect::<Vec<_>>();
            concepts.sort_by_key(|(_, Evoked(timestamp))| timestamp);
            for (c, _) in &concepts {
                text.sections.push(TextSection {
                    value: c.description.to_string() + "\n",
                    style: TextStyle {
                        font: asset_server.load("GloriaHallelujah-Regular.ttf"),
                        font_size: 80.0,
                        color: Color::WHITE,
                    },
                });
            }
        }
    }
}

pub struct QuestionDisplayPlugin;

impl Plugin for QuestionDisplayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(question_system.system())
            .add_event::<SetQuestionEvent>()
            .add_event::<ClearQuestionEvent>();
    }
}
