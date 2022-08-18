use crate::{
    config::manifest::SourceActionData, contract_creation::components::SourceAction, prelude::*,
};

pub fn spawn_source_actions(
    actions: &Vec<SourceActionData>,
    templates: &HashMap<String, Entity>,
    event: &Event,
    commands: &mut Commands,
) -> Vec<Entity> {
    let mut entites = vec![];

    actions.iter().for_each(|action| {
        let source_action = validate_source_action(action, templates, event);

        let action_entity = commands
            .spawn()
            .insert(TriggerAction)
            .insert(source_action)
            .id();

        entites.push(action_entity);
    });
    entites
}

fn validate_source_action(
    action: &SourceActionData,
    templates: &HashMap<String, Entity>,
    event: &Event,
) -> SourceAction {
    // TODO: Better error message
    let template = templates.get(&action.template).expect("Missing template");

    // TODO: Better error message
    event
        .inputs
        .iter()
        .find(|input| input.name.eq(&action.param))
        .expect("Event missing param");

    SourceAction {
        template: *template,
        param: action.param.clone(),
        event: event.clone(),
    }
}
