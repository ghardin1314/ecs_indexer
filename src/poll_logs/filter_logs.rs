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

fn matches_signature(log: &Log, trigger: &&EventTrigger) -> bool {
    log.topics
        .first()
        // Some events dont have a first topic?
        .unwrap_or(&H256::default())
        .eq(&trigger.event.signature())
}

fn opt_matches_address(log: &Log, address: &Option<&EthAddress>) -> bool {
    match address {
        Some(address) => log.address.eq(&address.0),
        None => true,
    }
}

fn opt_past_start_block(log: &Log, start_block: &Option<&TriggerStartBlock>) -> bool {
    match start_block {
        Some(block) => log
            .block_number
            .map_or(true, |log_block| log_block.ge(&block.0)),
        None => true,
    }
}
