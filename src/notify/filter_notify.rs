use crate::{
    poll_logs::{events::NewLog, ActionFired},
    prelude::*,
};

use super::components::NotifyAction;

/// Filters all new logs for events with source creation triggers
pub fn filter_creation(
    mut log_events: EventReader<NewLog>,
    trigger_query: Query<
        (
            &EventTrigger,
            Option<&TriggerStartBlock>,
            Option<&EthAddress>,
        ),
        With<ActiveTrigger>,
    >,
    action_query: Query<Entity, (With<TriggerAction>, With<NotifyAction>)>,
    mut action_events: EventWriter<ActionFired>,
) {
    log_events.iter().for_each(|log| {
        // Look for

        let triggers: Vec<(
            &EventTrigger,
            Option<&TriggerStartBlock>,
            Option<&EthAddress>,
        )> = trigger_query
            .iter()
            .filter(|(trigger, start_block, address)| {
                let sig_match = matches_signature(&log.0, trigger);

                let addr_match = opt_matches_address(&log.0, address);

                let block_match = opt_past_start_block(&log.0, start_block);

                sig_match & addr_match & block_match
            })
            .collect();

        if triggers.len() > 0 {
            println!("found triggers {:?}", triggers);
        }

        trigger_query
            .iter()
            .filter(|(trigger, start_block, address)| {
                let sig_match = matches_signature(&log.0, trigger);

                let addr_match = opt_matches_address(&log.0, address);

                let block_match = opt_past_start_block(&log.0, start_block);

                sig_match & addr_match & block_match
            })
            .for_each(|(trigger, _, _)| {
                trigger.actions.iter().for_each(|action| {
                    if let Ok(action) = action_query.get(*action) {
                        // Found a notify action, fire event
                        // println!("Firing new notify event from log {:?}", log.0);
                        // TODO: Call handler directly?
                        action_events.send(ActionFired {
                            action,
                            log: log.0.clone(),
                        })
                    }
                })
            });
    });
}
