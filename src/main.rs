mod components;
mod config;
mod contract_creation;
mod poll_logs;
mod resources;
mod systems;
mod utils;
mod prelude {
    pub use crate::components::*;
    pub use crate::config::*;
    pub use crate::contract_creation::*;
    pub use crate::poll_logs::*;
    pub use crate::resources::*;
    pub use crate::systems::*;
    pub use crate::utils::*;
    pub use async_compat::Compat;
    pub use bevy::{app::ScheduleRunnerSettings, prelude::*, utils::Duration};
    pub use ethers::prelude::*;
    pub use futures_lite::{future, Future};
    pub use serde::{Deserialize, Serialize};
    pub use std::env;
}

use dotenv::dotenv;
use prelude::*;

fn main() {
    // TODO remove for prod
    dotenv().ok();


    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(config::ConfigPlugin)
        .add_plugin(poll_logs::PollEventsPlugin)
        // .add_event::<systems::ActionFired>()
        // .add_startup_stage(AppStartupStage::PollEvents, SystemStage::parallel())
        // .add_startup_system_to_stage(
        //     AppStartupStage::PollEvents,
        //     systems::poll_logs.after(systems::load_config),
        // )
        // .add_startup_system_to_stage(
        //     AppStartupStage::PollEvents,
        //     systems::initiate_block_subscription.after(systems::load_config),
        // )
        // .add_stage_after(
        //     CoreStage::PreUpdate,
        //     AppStage::ProcessLogs,
        //     SystemStage::parallel(),
        // )
        // .add_system_to_stage(AppStage::ProcessLogs, systems::handle_polled_logs)
        // .add_system_to_stage(AppStage::ProcessLogs, systems::handle_new_block)
        // .add_system_to_stage(
        //     AppStage::ProcessLogs,
        //     systems::handle_event_triggers.after(systems::handle_polled_logs),
        // )
        // .add_stage_after(
        //     AppStage::ProcessLogs,
        //     AppStage::HandleContractCreation,
        //     SystemStage::parallel(),
        // )
        // .add_system(systems::handle_create_contract_actions)
        // .add_stage_after(
        //     AppStage::HandleContractCreation,
        //     AppStage::HandleActions,
        //     SystemStage::parallel(),
        // )
        .run();
}
