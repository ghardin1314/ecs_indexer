use crate::prelude::*;

use super::{components::CurrentLogsFetch, events::NewLog};

pub fn handle_logs(
    mut commands: Commands,
    mut log_query: Query<(Entity, &mut CurrentLogsFetch)>,
    mut events: EventWriter<NewLog>,
    mut from_block: ResMut<FromBlock>,
) {
    // Check if logs query exists. If not, we have returned from the last query and haven't started a new one yet
    if let Ok((entity, mut task)) = log_query.get_single_mut() {
        // Check if provider query has returned any logs yet
        if let Some(logs) = future::block_on(future::poll_once(&mut task.task)) {
            match logs {
                Ok(logs) => {
                    logs.into_iter().for_each(|log| events.send(NewLog(log)));

                    commands.entity(entity).despawn();
                    
                    from_block.0 = task.to_block + 1;
                }
                Err(err) => {
                    // TODO: Better error handling
                    println!("Got provider error: {:?}", err);
                }
            }
        }
    }
}
