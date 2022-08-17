mod load_config;
mod load_providers;
mod resources;

use crate::prelude::*;
use load_config::load_config;
use load_providers::load_providers;

pub use resources::*;

#[derive(StageLabel)]
pub struct LoadConfigStage;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage(LoadConfigStage, SystemStage::parallel())
            .add_startup_system_to_stage(LoadConfigStage, load_providers)
            .add_startup_system_to_stage(LoadConfigStage, load_config);
    }
}
