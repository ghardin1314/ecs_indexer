use crate::{config::LoadConfigStage, prelude::*};

mod handle_block;
mod resources;
mod subscribe_blocks;

use handle_block::handle_block;
use subscribe_blocks::subscribe_blocks;

pub use resources::*;

#[derive(StageLabel)]
pub struct InitiateBlockSubscriptionStage;

pub struct CurrentBlockPlugin;

impl Plugin for CurrentBlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage_after(
            LoadConfigStage,
            InitiateBlockSubscriptionStage,
            SystemStage::parallel(),
        )
        .add_startup_system_to_stage(InitiateBlockSubscriptionStage, subscribe_blocks)
        .add_system(handle_block);
    }
}
