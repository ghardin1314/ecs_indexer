use crate::prelude::*;

use super::events::{ActionFired, NewLog};

pub fn filter_logs(
    trigger_query: Query<
        (
            &EventTrigger,
            Option<&TriggerStartBlock>,
            Option<&EthAddress>,
            // Assumption that each EventTrigger only has children who are actions
            &Children,
        ),
        With<ActiveTrigger>,
    >,
    mut log_events: EventReader<NewLog>,
    mut action_events: EventWriter<ActionFired>,
) {
    log_events.iter().for_each(|log| {
        trigger_query
            .iter()
            // find all events that match the signature of events we car about
            .filter(|(trigger, start_block, address, _)| {
                let sig_match = matches_signature(&log.0, trigger);

                let addr_match = opt_matches_address(&log.0, address);

                let block_match = opt_past_start_block(&log.0, start_block);

                sig_match & addr_match & block_match
            })
            // fire all actions associated with each event
            .for_each(|(_, _, _, actions)| {
                // debug!("found event {:?} with matching signature", log.0);

                actions.into_iter().for_each(|action| {
                    action_events.send(ActionFired {
                        action: *action,
                        log: log.0.clone(),
                    })
                })
            });
    })
}


