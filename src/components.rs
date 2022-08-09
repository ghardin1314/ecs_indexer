use crate::prelude::*;

use ethers::{abi::Event, types::Address};

#[derive(Component)]
struct Contract {}

#[derive(Component)]
pub struct EventTrigger {
    pub event: Event,
    pub address: Option<Address>,
}

#[derive(Component)]
struct TriggerAction {}

#[derive(Component)]
struct StartBlock(usize);
