mod components;
mod config;
mod contract_creation;
mod current_block;
mod poll_logs;
mod utils;
mod prelude {
    pub use crate::components::*;
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
        .add_plugin(current_block::CurrentBlockPlugin)
        .add_plugin(poll_logs::PollEventsPlugin)
        .add_plugin(contract_creation::CreateContractsPlugin)
        .run();
}
