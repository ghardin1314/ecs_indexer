use crate::prelude::*;

use bevy::tasks::Task;
use ethers::{abi::Event, types::Address};

#[derive(Component)]
pub struct Contract {}

#[derive(Component, Debug)]
pub struct EventTrigger {
    pub event: Event,
}
#[derive(Component)]
pub struct EthAddress(pub Address);

#[derive(Component)]
pub struct TriggerAction {}

#[derive(Component)]
pub struct TriggerStartBlock(pub U64);

#[derive(Component)]
pub struct CurrentBlockQuery(pub Task<U64>);
