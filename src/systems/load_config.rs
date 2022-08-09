use crate::prelude::*;

use ethers::{
    abi::{Abi, AbiParser, Event},
    prelude::*,
    utils::Anvil,
};
use std::fs::File;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Manifest {
    contracts: Vac<ContractData>,
}

struct ContractData {
    abi: String,
}

pub fn load_config(mut commands: Commands) {
    let file = File::open("./resources/Gravity.json").unwrap();
    let abi = Abi::load(file).unwrap();

    for event in abi.events() {
        commands.spawn().insert(EventTrigger {
            event: event.clone(),
        });
    }
}
