use ethers::abi::AbiParser;

use crate::{config::manifest::EventTriggerData, prelude::*};

use super::source_actions::spawn_source_actions;

pub fn spawn_event_triggers(
    event_triggers: &Vec<EventTriggerData>,
    abi: &Abi,
    template_entities: &mut HashMap<String, Entity>,
    commands: &mut Commands,
) -> Vec<Entity> {
    let mut event_trigger_entities = vec![];

    event_triggers.iter().for_each(|event_trigger| {
        let event = validate_event_trigger(event_trigger, abi);

        let event_trigger_entity = commands
            .spawn()
            .insert(EventTrigger {
                event: event.clone(),
            })
            .id();

        if let Some(source_actions) = &event_trigger.source_actions {
            let source_action_entities =
                spawn_source_actions(source_actions, &template_entities, &event, commands);
            commands
                .entity(event_trigger_entity)
                .push_children(&source_action_entities);
        }

        // TODO: NotifyTriggers

        event_trigger_entities.push(event_trigger_entity);
    });

    event_trigger_entities
}

fn validate_event_trigger(event_trigger: &EventTriggerData, abi: &Abi) -> Event {
    let my_event = AbiParser::default()
        .parse_event(&format!("event {}", event_trigger.signature))
        // TODO: Better error handling
        .expect("Invalid event signature");

    abi.events()
        .find(|event| event.signature().eq(&my_event.signature()))
        .expect("Cannot find event with signature")
        .clone()
}
