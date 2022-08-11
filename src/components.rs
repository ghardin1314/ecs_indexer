use crate::prelude::*;

use bevy::tasks::Task;
use ethers::{abi::Event, types::Address};

#[derive(Component)]
pub struct Contract {}

#[derive(Component, Debug)]
pub struct EventTrigger {
    pub event: Event,
    pub address: Option<Address>,
    pub start_block: Option<U64>
}

#[derive(Component)]
pub struct TriggerAction {}

#[derive(Component)]
pub struct StartBlock(usize);

#[derive(Component)]
pub struct CurrentBlockQuery(pub Task<U64>);
