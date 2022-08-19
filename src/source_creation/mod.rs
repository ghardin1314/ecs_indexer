use crate::{poll_logs::PollLogsStage, prelude::*};

pub mod components;
mod filter_creation;
mod handle_source_creation;

use filter_creation::filter_creation;
use handle_source_creation::handle_source_creation;

#[derive(StageLabel)]
pub struct CreateSourcesStage;

pub struct CreateSourcesPlugin;

impl Plugin for CreateSourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(PollLogsStage, CreateSourcesStage, SystemStage::parallel())
            .add_system_to_stage(CreateSourcesStage, filter_creation)
            .add_system_to_stage(
                CreateSourcesStage,
                handle_source_creation.after(filter_creation),
            );
    }
}
