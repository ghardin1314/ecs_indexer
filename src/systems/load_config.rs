use crate::prelude::*;

use ethers::abi::{Abi, Event};
use std::{fs::File, io::Read};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Manifest {
    contracts: Vec<ContractData>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ContractData {
    name: String,
    abi: String,
    address: Option<Address>,
    start_block: Option<usize>,
    event_triggers: Option<Vec<EventTriggerConfig>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct EventTriggerConfig {
    event: String,
}

pub fn load_config(mut commands: Commands) {
    let manifest = open_manifest("./resources/manifest.yaml".to_owned()).unwrap();

    let mut global_start_block = U64::max_value();

    for contract_data in manifest.contracts.into_iter() {
        let file = File::open(contract_data.abi.clone()).unwrap();
        let abi = Abi::load(file).unwrap();

        let address = contract_data.address;
        let start_block = contract_data.start_block.map(|block| U64::from(block));

        println!("{:?}", start_block);

        // Update global start block with min passed from manifest
        if let Some(start_block) = start_block {
            global_start_block = global_start_block.min(start_block);
        }

        // Create event trigger components
        if let Some(event_triggers) = contract_data.event_triggers {
            let triggers = trigger_to_event(event_triggers, abi);

            triggers.into_iter().for_each(|event| {
                let entity = commands.spawn().insert(EventTrigger { event }).id();

                if let Some(address) = address {
                    commands.entity(entity).insert(EthAddress(address));
                }

                if let Some(start_block) = start_block {
                    commands
                        .entity(entity)
                        .insert(TriggerStartBlock(start_block));
                }
            })
        }
    }

    // If block still max, not start blocks passed. Start are first block
    if global_start_block.eq(&U64::max_value()) {
        global_start_block = U64::zero();
    }
    commands.insert_resource(StartBlock(global_start_block));
}

fn trigger_to_event(event_triggers: Vec<EventTriggerConfig>, abi: Abi) -> Vec<Event> {
    let mut events: Vec<Event> = vec![];

    for trigger in event_triggers {
        let abi_event = abi.event(&trigger.event).expect("Event not found");

        events.push(abi_event.clone());
    }

    events
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
