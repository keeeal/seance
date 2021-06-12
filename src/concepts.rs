use bevy::prelude::{
    Entity, Res, Query, With, Without, Plugin, IntoSystem, Commands, Time,
    Added, AppBuilder, info, EventReader,
};
use crate::ghost::GhostInteractionEvent;
use std::collections::HashSet;
use std::time::Duration;

pub struct Concept {
    pub description: String,
    pub parents: Vec<HashSet<Entity>>,
}

pub struct Evoked(Duration);

pub struct EvokesConcept(pub Entity);
pub struct ClearsConcepts;

fn evoke_concepts(
    evoke_query: Query<&EvokesConcept>,
    clear_query: Query<&ClearsConcepts>,
    concept_query: Query<&Concept>,
    evoked_concept_query: Query<(Entity, &Concept), With<Evoked>>,
    mut commands: Commands,
    time: Res<Time>,
    mut ev_interaction: EventReader<GhostInteractionEvent>,
) {
    for GhostInteractionEvent { ghost: _, target } in ev_interaction.iter() {
        if let Ok(&EvokesConcept(concept)) = evoke_query.get(*target) {
            commands
                .entity(concept)
                .insert(Evoked(time.time_since_startup()));
            if let Ok(c) = concept_query.get(concept) {
                info!("Activated item to evoke {}", c.description);
            }
            break
        }

        if let Ok(_) = clear_query.get(*target) {
            for (e, c) in evoked_concept_query.iter() {
                commands
                    .entity(e)
                    .remove::<Evoked>();
                info!("Cleared concept {}", c.description);
            }
            break
        }
    }
}

fn join_concepts(
    new_query: Query<(), (With<Concept>, Added<Evoked>)>,
    evoked_query: Query<(Entity, &Evoked), With<Concept>>,
    concept_query: Query<(Entity, &Concept), Without<Evoked>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if let None = new_query.iter().next() {
        return
    }

    let evoked_set: HashSet<Entity> = evoked_query.iter()
        .map(|(e, _)| e)
        .collect();

    let to_join = concept_query.iter()
        .flat_map(
            |(entity, concept)| concept.parents.iter()
                .filter(
                    |parents| evoked_set.is_superset(parents)
                )
                .map(
                    move |parents| (entity, parents.clone())
                )
        )
        .max_by_key(
            |(_, parents)| parents.iter().map(
                |e| evoked_query.get(*e).unwrap().1.0
            ).collect::<Vec<Duration>>()
        );

    if let Some((entity, parents)) = to_join {
        commands
            .entity(entity)
            .insert(Evoked(time.time_since_startup()));
        if let Ok((_, c)) = concept_query.get(entity) {
            info!("Joined concepts to evoke {}", c.description);
        }

        for parent in parents.iter() {
            commands.entity(*parent)
                .remove::<Evoked>();
        }
    }
}

pub struct ConceptPlugin;

impl Plugin for ConceptPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(evoke_concepts.system())
            .add_system(join_concepts.system());
    }
}
