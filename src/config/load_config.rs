use crate::prelude::*;

use super::resources::{FromBlock, ReorgBlocks};

use bevy::utils::HashMap;
use ethers::abi::{Abi, Event};
use std::{fs::File, io::Read};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Manifest {
    contracts: Vec<ContractData>,
    templates: Vec<ContractTemplateConfig>,
    reorg_blocks: Option<U64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct EventTriggerConfig {
    event: String,
    actions: Vec<TriggerActionConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum TriggerActionType {
    CreateContract,
    Debug,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct TriggerActionConfig {
    action_type: TriggerActionType,
    template: Option<String>,
    field: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ContractTemplateConfig {
    name: String,
    abi: String,
    event_triggers: Option<Vec<EventTriggerConfig>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ContractData {
    name: String,
    abi: String,
    address: Option<Address>,
    start_block: Option<usize>,
    event_triggers: Option<Vec<EventTriggerConfig>>,
}

pub fn load_config(mut commands: Commands) {
    let manifest = open_manifest("./resources/manifest.yaml".to_owned()).unwrap();

    let mut global_start_block = U64::max_value();

    let mut templates = HashMap::new();

    manifest.templates.into_iter().for_each(|template| {
        let abi = abi_from_path(template.abi);

        let mut event_trigger_entities = vec![];

        if let Some(event_triggers) = template.event_triggers {
            event_triggers.into_iter().for_each(|config| {
                let event = validate_event(&config, &abi);

                // TODO: Load all templates before spawning actions

                let actions_entities =
                    spawn_actions(config.actions, &event, &templates, &mut commands);

                let entity = commands
                    .spawn()
                    .insert(EventTrigger { event })
                    .push_children(&actions_entities)
                    .id();
                event_trigger_entities.push(entity);
            });
        }

        let template_entity = commands
            .spawn()
            .insert(ContractTemplate)
            .push_children(&event_trigger_entities)
            .id();

        templates.insert(template.name, template_entity);
    });

    manifest.contracts.into_iter().for_each(|contract_data| {
        let file = File::open(contract_data.abi.clone()).unwrap();
        let abi = Abi::load(file).unwrap();

        let address = contract_data.address;
        let start_block = contract_data.start_block.map(|block| U64::from(block));

        // Update global start block with min passed from manifest
        if let Some(start_block) = start_block {
            global_start_block = global_start_block.min(start_block);
        }

        // Create event trigger components
        if let Some(event_triggers) = contract_data.event_triggers {
            event_triggers.into_iter().for_each(|trigger_config| {
                let event = validate_event(&trigger_config, &abi);

                let actions_entities =
                    spawn_actions(trigger_config.actions, &event, &templates, &mut commands);

                let trigger_entity = commands
                    .spawn()
                    .insert(EventTrigger { event })
                    .insert(ActiveTrigger)
                    .push_children(&actions_entities)
                    .id();

                if let Some(address) = address {
                    commands.entity(trigger_entity).insert(EthAddress(address));
                }
                if let Some(start_block) = start_block {
                    commands
                        .entity(trigger_entity)
                        .insert(TriggerStartBlock(start_block));
                }
            });
        }
    });

    // If block still max, not start blocks passed. Start are first block
    if global_start_block.eq(&U64::max_value()) {
        global_start_block = U64::zero();
    }
    commands.insert_resource(FromBlock(global_start_block));

    // TODO: Add some defaults for reorg_blocks
    commands.insert_resource(ReorgBlocks(manifest.reorg_blocks.unwrap_or_default()));

    println!("Loaded config");
}

fn abi_from_path(path: String) -> Abi {
    let file = File::open(path).unwrap();
    Abi::load(file).unwrap()
}

fn validate_event(config: &EventTriggerConfig, abi: &Abi) -> Event {
    abi.event(&config.event).expect("Event not found").clone()
}

fn spawn_actions(
    actions: Vec<TriggerActionConfig>,
    event: &Event,
    templates: &HashMap<String, Entity>,
    commands: &mut Commands,
) -> Vec<Entity> {
    let mut actions_entities = vec![];
    actions.into_iter().for_each(|action| {
        let action_entity = commands.spawn().insert(TriggerAction).id();

        match action.action_type {
            TriggerActionType::CreateContract => {
                let (template, field) = validate_create_contract(&action, event, templates);
                commands.entity(action_entity).insert(CreateContractAction {
                    template,
                    field,
                    event: event.clone(),
                });
            }
            TriggerActionType::Debug => {
                commands.entity(action_entity).insert(DebugAction);
            }
        };

        actions_entities.push(action_entity);
    });

    actions_entities
}

fn validate_create_contract(
    action: &TriggerActionConfig,
    event: &Event,
    templates: &HashMap<String, Entity>,
) -> (Entity, String) {
    let template_name = action
        .template
        .as_ref()
        .expect("Template name not provided");
    let field = action.field.as_ref().expect("Template field not provided");

    let template = templates
        .get(template_name)
        .expect("Template for action not found");

    event
        .inputs
        .iter()
        .find(|input| input.name.eq(field))
        .expect("field not found in trigger event");

    (*template, field.to_string())
}

fn open_manifest(path: String) -> Option<Manifest> {
    match File::open(path) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let manifest: Manifest = serde_yaml::from_str(&content).unwrap();
            Some(manifest)
        }
        Err(error) => {
            panic!("No manifest file found")
        }
    }
}
