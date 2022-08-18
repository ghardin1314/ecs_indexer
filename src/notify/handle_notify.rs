use crate::{poll_logs::ActionFired, prelude::*};

use super::components::NotifyAction;

pub fn handle_notify(
    actions_query: Query<&NotifyAction, With<TriggerAction>>,
    mut events: EventReader<ActionFired>,
) {
    events.iter().for_each(|event| {
        println!("got event in notify: {:?}", event);
        if let Ok(notify_action) = actions_query.get(event.action) {
            println!(
                "Notify Action Triggered {:?} by log: {:?}",
                notify_action, event.log
            )
        }
    })
}
