use crate::{config::manifest::Manifest, prelude::*};

use super::event_triggers::spawn_event_triggers;

pub fn spawn_sources(
    manifest: &Manifest,
    contracts: &HashMap<String, (Abi, bool)>,
    template_entities: &mut HashMap<String, Entity>,
    commands: &mut Commands,
) -> U64 {
    let mut global_start_block = U64::max_value();

    manifest.sources.iter().for_each(|source| {
        let address = source.address;

        let start_block = source.start_block;

        let (abi, _) = contracts.get(&source.contract.name).unwrap();

        // Update global start block with min passed from manifest
        if let Some(start_block) = start_block {
            global_start_block = global_start_block.min(U64::from(start_block));
        }

        if let Some(event_triggers) = &source.event_triggers {
            let event_trigger_entites =
                spawn_event_triggers(event_triggers, abi, template_entities, commands);

            // Set all event triggers as active with context info
            event_trigger_entites.iter().for_each(|entity| {
                commands.entity(*entity).insert(ActiveTrigger);

                if let Some(address) = address {
                    commands.entity(*entity).insert(EthAddress(address));
                }

                if let Some(start_block) = start_block {
                    commands
                        .entity(*entity)
                        .insert(TriggerStartBlock(U64::from(start_block)));
                }
            })
        }
    });

    if global_start_block.eq(&U64::max_value()) {
        global_start_block = U64::zero();
    }

    global_start_block
}
