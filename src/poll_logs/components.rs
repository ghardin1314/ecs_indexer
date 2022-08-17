use bevy::tasks::Task;

use crate::prelude::*;


#[derive(Component, Debug)]
pub struct CurrentLogsFetch {
    pub task: Task<Result<Vec<Log>, ProviderError>>,
    pub from_block: U64,
    pub to_block: U64,
}
