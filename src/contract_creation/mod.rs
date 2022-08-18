use crate::{poll_logs::PollLogsStage, prelude::*};

pub mod components;
mod handle_contract_creation;

use handle_contract_creation::handle_contract_creation;

#[derive(StageLabel)]
pub struct CreateContractsStage;

pub struct CreateContractsPlugin;

impl Plugin for CreateContractsPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(PollLogsStage, CreateContractsStage, SystemStage::parallel())
            .add_system_to_stage(CreateContractsStage, handle_contract_creation);
    }
}
