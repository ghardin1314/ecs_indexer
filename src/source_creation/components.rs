use ethers::abi::{Abi, Event};

use crate::prelude::*;

/// Tag for all actions that can be triggered
#[derive(Component, Debug)]
pub struct SourceAction {
    pub template: Entity,
    pub param: String,
    pub event: Event,
}

#[derive(Component)]
pub struct SourceTemplate {
    pub abi: Abi,
}
