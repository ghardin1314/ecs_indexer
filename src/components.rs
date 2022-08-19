use crate::prelude::*;

use bevy::tasks::Task;
use ethers::{abi::Event, types::Address};

#[derive(Component, Debug)]
pub struct EventTrigger {
    pub event: Event,
    pub actions: Vec<Entity>,
}
#[derive(Component, Debug)]
pub struct EthAddress(pub Address);

/// Tag for triggers when they become active
#[derive(Component, Debug)]
pub struct ActiveTrigger;

/// Tag for all actions that can be triggered
#[derive(Component, Debug)]
pub struct TriggerAction;

/// Defined for triggers that should only be active after certain block
#[derive(Component, Debug)]
pub struct TriggerStartBlock(pub U64);

/// Keeping for example. Not in use
#[derive(Component)]
pub struct CurrentBlockQuery(pub Task<U64>);
