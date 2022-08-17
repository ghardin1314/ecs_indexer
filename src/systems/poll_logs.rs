use std::time::Instant;

use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::prelude::*;

pub struct NewLog(Log);

pub struct ActionFired {
    action: Entity,
    log: Log,
}

pub fn poll_logs(
    mut commands: Commands,
    tx: ResMut<PollLogsSender>,
    from_block: Res<StartBlock>,
    reorg_blocks: Res<ReorgBlocks>,
    current_block: Res<CurrentBlock>,
    is_polling: ResMut<PollingLogs>,
    provider: Res<Provider<Http>>,
) {
    if is_polling.0 || from_block.0 == current_block.0 {
        return;
    }

    // let (tx, rx) = unbounded::<Log>();
    let provider_cl = provider.clone();

    let mut from_block = from_block.0.clone();

    std::thread::spawn(move || loop {
        let start = Instant::now();

        println!("from block: {}", from_block);

        let to_block = from_block + 2000;

        println!("to block: {}", to_block);

        let filter = Filter::new().from_block(from_block).to_block(to_block);

        async {
            provider_cl
                .get_logs(&filter)
                .await
                .unwrap()
                .into_iter()
                .for_each(|log| tx.send(log).unwrap());
        }
        .compat_await();

        from_block = to_block + 1;
        let duration = start.elapsed();
        println!("Got logs in {:?}", duration);
    });

    commands.insert_resource(PollLogsReceiver(rx));
}

pub fn handle_polled_logs(receiver: ResMut<PollLogsReceiver>, mut events: EventWriter<NewLog>) {
    for log in receiver.try_iter() {
        events.send(NewLog(log));
    }
}

pub fn handle_event_triggers(
    mut log_events: EventReader<NewLog>,
    trigger_query: Query<
        (
            &EventTrigger,
            Option<&TriggerStartBlock>,
            Option<&EthAddress>,
            &Children,
        ),
        With<ActiveTrigger>,
    >,
    mut action_events: EventWriter<ActionFired>,
) {
    for log in log_events.iter() {
        let matching_sigs: Vec<(&EventTrigger, &Children)> = trigger_query
            .iter()
            .filter(|(trigger, start_block, address, _)| {
                let sig_match = matches_signature(&log.0, trigger);

                let addr_match = opt_matches_address(&log.0, address);

                let block_match = opt_past_start_block(&log.0, start_block);

                sig_match & addr_match & block_match
            })
            .map(|(trigger, _, _, children)| (trigger, children))
            .collect();

        matching_sigs.iter().for_each(|(trigger, children)| {
            println!(
                "found event {:?} with signature matching {:?}",
                log.0, trigger
            );

            children.into_iter().for_each(|action| {
                action_events.send(ActionFired {
                    action: *action,
                    log: log.0.clone(),
                })
            });
        })
    }
}

pub fn handle_create_contract_actions(
    actions_query: Query<&TriggerAction, With<CreateContractAction>>,
    mut events: EventReader<ActionFired>,
) {
    events.iter().for_each(|event| {
        let action = actions_query.get(event.action);

        println!("action triggered: {:?}", { action });

        debug!("action triggered: {:?}", { action });

        // TODO: activate all triggers associated with contract template
        // TODO: assign address and start block to triggers
    });
}

// fn matches_signature(log: &Log, trigger: &&EventTrigger) -> bool {
//     log.topics
//         .first()
//         // Some events dont have a first topic?
//         .unwrap_or(&H256::default())
//         .eq(&trigger.event.signature())
// }

// fn opt_matches_address(log: &Log, address: &Option<&EthAddress>) -> bool {
//     match address {
//         Some(address) => log.address.eq(&address.0),
//         None => true,
//     }
// }

// fn opt_past_start_block(log: &Log, start_block: &Option<&TriggerStartBlock>) -> bool {
//     match start_block {
//         Some(block) => log
//             .block_number
//             .map_or(true, |log_block| log_block.ge(&block.0)),
//         None => true,
//     }
// }
