use crate::{prelude::*, source_creation::CreateSourcesStage};
use bevy::prelude::StageLabel;

pub mod components;
mod filter_notify;
mod handle_notify;

use filter_notify::filter_creation;
use handle_notify::handle_notify;

#[derive(StageLabel)]
pub struct HandleNotifyStage;

pub struct HandleNotifyPlugin;

impl Plugin for HandleNotifyPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CreateSourcesStage,
            HandleNotifyStage,
            SystemStage::parallel(),
        )
        .add_system_to_stage(HandleNotifyStage, filter_creation)
        .add_system_to_stage(HandleNotifyStage, handle_notify.after(filter_creation));
    }
}
