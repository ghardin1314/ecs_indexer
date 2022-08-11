mod components;
mod resources;
mod systems;
mod utils;
mod prelude {
    pub use crate::components::*;
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

use prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
enum AppStage {
    Config,
    PollEvents,
}

fn main() {
    let api_key = env::var("API_KEY").expect("no api key provided");
    let provider_http =
        Provider::<Http>::try_from(format!("https://eth-mainnet.g.alchemy.com/v2/{}", api_key))
            .expect("Error connecting to Ethereum node");
    let provider_ws =
        Provider::<Ws>::connect(format!("wss://eth-mainnet.g.alchemy.com/v2/{}", api_key))
            .compat_await()
            .expect("Error connecting to websocket node");

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0,
        )))
        .add_plugins(MinimalPlugins)
        .insert_resource(provider_http)
        .insert_resource(provider_ws)
        .add_event::<systems::NewLog>()
        .add_startup_stage(AppStage::Config, SystemStage::parallel())
        .add_startup_stage(AppStage::PollEvents, SystemStage::parallel())
        .add_startup_system_to_stage(AppStage::Config, systems::load_config)
        .add_startup_system_to_stage(
            AppStage::PollEvents,
            systems::poll_logs.after(systems::load_config),
        )
        .add_system(systems::handle_polled_logs)
        .add_system(systems::handle_event_triggers)
        // .add_startup_system(systems::query_block)
        // .add_startup_system(systems::query_block_loop)
        // .add_startup_system(systems::create_logs_subscription)
        // .add_system(systems::handle_block)
        // .add_system(systems::read_events)
        // .add_system(systems::handle_block_loop)
        // .add_system(systems::handle_new_logs)
        // .add_system(systems::read_events)
        // .add_stage_before(
        //     CoreStage::Update,
        //     AppStages::GetEvents,
        //     SystemStage::parallel(),
        // )
        // .add_system_to_stage(AppStages::GetEvents, systems::query_block)
        // .add_system_to_stage(AppStages::GetEvents, systems::handle_block)
        .run();
}
