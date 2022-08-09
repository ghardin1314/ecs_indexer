use crate::prelude::*;

use ethers::{
    abi::{Abi, AbiParser, Event},
    prelude::*,
    utils::Anvil,
};
use std::{fs::File, io::Read};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Manifest {
    contracts: Vec<ContractData>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ContractData {
    abi: String,
}

pub fn load_config(mut commands: Commands) {
    let manifest = open_manifest("./resources/manifest.yaml".to_owned()).unwrap();

    for contract_data in manifest.contracts.iter() {
        let file = File::open(contract_data.abi.clone()).unwrap();
        let abi = Abi::load(file).unwrap();

        for event in abi.events() {
            commands.spawn().insert(EventTrigger {
                event: event.clone(),
                address: None,
            });
        }
    }
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
