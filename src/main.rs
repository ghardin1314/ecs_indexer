mod components;
mod systems;

mod prelude {
    pub use crate::components::*;
    pub use crate::systems::*;
    pub use bevy::{app::ScheduleRunnerSettings, prelude::*, utils::Duration};
    pub use ethers::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use std::env;
}

use prelude::*;

#[derive(StageLabel)]
enum AppStages {
    GetEvents,
}

fn main() {
    let api_key = env::var("API_KEY").expect("no api key provided");
    let provider =
        Provider::<Http>::try_from(format!("https://eth-mainnet.g.alchemy.com/v2/{}", api_key))
            .expect("Error connecting to Ethereum node");

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .insert_resource(provider)
        .add_startup_system(systems::load_config)
        .add_startup_system(systems::query_block)
        // .add_system(systems::read_events)
        .add_stage_before(
            CoreStage::Update,
            AppStages::GetEvents,
            SystemStage::parallel(),
        )
        // .add_system_to_stage(AppStages::GetEvents, systems::query_block)
        .add_system_to_stage(AppStages::GetEvents, systems::handle_block)
        .run();
}
