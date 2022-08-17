mod load_config;
mod load_providers;

use crate::prelude::*;
use load_config::load_config;
use load_providers::load_providers;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("Config", SystemStage::parallel())
            .add_startup_system_to_stage("Config", load_providers)
            .add_startup_system_to_stage("Config", load_config);
    }
}
