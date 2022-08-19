mod components;
pub mod events;
mod fetch_logs;
mod filter_logs;
mod handle_logs;
mod resources;

use crate::prelude::*;
use fetch_logs::fetch_logs;
use filter_logs::filter_logs;
use handle_logs::handle_logs;

pub use events::ActionFired;

#[derive(StageLabel)]
pub struct PollLogsStage;

pub struct PollEventsPlugin;

impl Plugin for PollEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(CoreStage::PreUpdate, PollLogsStage, SystemStage::parallel())
            .add_system_to_stage(PollLogsStage, fetch_logs)
            .add_system_to_stage(PollLogsStage, handle_logs.after(fetch_logs))
            // .add_system_to_stage(PollLogsStage, filter_logs.after(handle_logs))
            .add_event::<events::NewLog>()
            .add_event::<events::ActionFired>();
    }
}
