use bevy::utils::HashMap;

use crate::{
    config::manifest::{Manifest, TemplateData},
    contract_creation::components::SourceTemplate,
    prelude::*,
};

use super::event_triggers::spawn_event_triggers;

pub fn spawn_templates(
    manifest: &Manifest,
    contracts: &HashMap<String, (Abi, bool)>,
    commands: &mut Commands,
) -> HashMap<String, Entity> {
    match &manifest.templates {
        Some(templates) => {
            // spawn all templates first as they may be used in other templates actions
            let mut template_entities = spawn_template_entities(&templates, contracts, commands);

            // spawn all event triggers for each template
            templates.iter().for_each(|template| {
                let (abi, _) = contracts.get(&template.contract.name).unwrap();

                if let Some(event_triggers) = &template.event_triggers {
                    let event_trigger_entities =
                        spawn_event_triggers(event_triggers, abi, &mut template_entities, commands);

                    let entity = template_entities.get(&template.contract.name).unwrap();
                    commands
                        .entity(*entity)
                        .push_children(&event_trigger_entities);
                }
            });
            template_entities
        }
        None => HashMap::new(),
    }
}

fn spawn_template_entities(
    templates: &Vec<TemplateData>,
    contracts: &HashMap<String, (Abi, bool)>,
    commands: &mut Commands,
) -> HashMap<String, Entity> {
    let mut template_entities = HashMap::new();
    templates.iter().for_each(|template| {
        // assumption that is_template = true since we just loaded the manifest into this map
        let config = contracts.get(&template.contract.name).unwrap();

        let template_entity = commands
            .spawn()
            .insert(SourceTemplate {
                abi: config.0.clone(),
            })
            .id();

        template_entities.insert(template.contract.name.clone(), template_entity);
    });

    template_entities
}
