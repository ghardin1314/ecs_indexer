use crate::prelude::*;

use bevy::tasks::Task;
use crossbeam_channel::Receiver;
use ethers::{abi::Event, types::Address};

#[derive(Component)]
pub struct Contract {}

#[derive(Component)]
pub struct EventTrigger {
    pub event: Event,
    pub address: Option<Address>,
}

#[derive(Component)]
pub struct TriggerAction {}

#[derive(Component)]
pub struct StartBlock(usize);

#[derive(Component)]
pub struct CurrentBlockQuery(pub Task<U64>);
