use crate::prelude::*;

use ethers::abi::Event;

#[derive(Component)]
struct Contract {}

#[derive(Component)]
pub struct EventTrigger {
    pub event: Event,
}

#[derive(Component)]
struct TriggerAction {}
