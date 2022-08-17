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

/// Tag for triggers when they become active
#[derive(Component, Debug)]
pub struct ActiveTrigger;

/// Tag for all actions that can be triggered
#[derive(Component, Debug)]
pub struct TriggerAction;

/// Action to activate new triggers from templates
#[derive(Component, Debug)]
pub struct CreateContractAction {
    pub template: Entity,
    pub field: String,
    pub event: Event
}

#[derive(Component, Debug)]
pub struct DebugAction;

/// Defined for triggers that should only be active after certain block
#[derive(Component)]
pub struct TriggerStartBlock(pub U64);

/// Keeping for example. Not in use
#[derive(Component)]
pub struct CurrentBlockQuery(pub Task<U64>);
