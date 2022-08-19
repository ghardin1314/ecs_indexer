use crate::{poll_logs::ActionFired, prelude::*};

use super::components::NotifyAction;

pub fn handle_notify(
    actions_query: Query<&NotifyAction, With<TriggerAction>>,
    mut events: EventReader<ActionFired>,
) {
    events.iter().for_each(|event| {
        if let Ok(notify_action) = actions_query.get(event.action) {
            println!(
                "Notify Action address: {:?} at block: {:?} \n",
                event.log.address, event.log.block_number
            )
        }
    })
}
