use crate::prelude::*;

use bevy::tasks::Task;
use ethers::{abi::Event, types::Address};

#[derive(Component, Debug)]
pub struct EventTrigger {
    pub event: Event,
}
#[derive(Component)]
pub struct EthAddress(pub Address);

#[derive(Component)]
pub struct ContractTemplate;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum TriggerActionType {
    CreateContract,
    Debug
}
#[derive(Component, Debug)]
pub struct ActiveTrigger;

#[derive(Component, Debug)]
pub struct TriggerAction {
    pub action_type: TriggerActionType,
    pub template: Option<Entity>,
    pub field: Option<String>,
}

#[derive(Component)]
pub struct TriggerStartBlock(pub U64);

#[derive(Component)]
pub struct CurrentBlockQuery(pub Task<U64>);
