use crate::{config::manifest::NotifyActionData, notify::components::NotifyAction, prelude::*};

pub fn spawn_notify_triggers(
    actions: &Vec<NotifyActionData>,
    commands: &mut Commands,
) -> Vec<Entity> {
    let mut entities = vec![];

    actions.iter().for_each(|action| {
        let action_entity = commands
            .spawn()
            .insert(TriggerAction)
            .insert(NotifyAction {
                endpoint: action.endpoint.clone(),
            })
            .id();

        entities.push(action_entity)
    });

    entities
}
