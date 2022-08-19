use ethers::abi::{RawLog, Token};

use crate::{poll_logs::ActionFired, prelude::*};

use super::components::{SourceAction, SourceTemplate};

pub fn handle_source_creation(
    actions_query: Query<(&TriggerAction, &SourceAction)>,
    templates_query: Query<&Children, With<SourceTemplate>>,
    event_triggers_query: Query<&EventTrigger>,
    mut events: EventReader<ActionFired>,
    mut commands: Commands,
) {
    events.iter().for_each(|event| {
        if let Ok((_, create_source)) = actions_query.get(event.action) {
            // parse log from event
            let log = create_source
                .event
                .parse_log(RawLog {
                    topics: event.log.topics.clone(),
                    data: event.log.data.to_vec(),
                })
                .unwrap();

            // println!(
            //     "source action topics: {:?} at block {:?} \n",
            //     log.params, event.log.block_number
            // );

            // Get address from event param
            let address = &log
                .params
                .iter()
                .find(|param| param.name == create_source.param)
                .unwrap()
                .value;

            let from_block = event.log.block_number.unwrap();

            // get children of template
            let children = templates_query
                .get(create_source.template)
                .expect("Found create source action with no template");

            children.iter().for_each(|child| {
                let event_trigger = event_triggers_query
                    .get(*child)
                    .expect("Missing event trigger from template");

                let event = event_trigger.event.clone();
                let actions = event_trigger.actions.clone();

                let new_trigger = commands
                    .spawn()
                    .insert(EventTrigger { event, actions })
                    .insert(ActiveTrigger)
                    .insert(TriggerStartBlock(from_block))
                    .id();

                if let Token::Address(address) = address {
                    commands.entity(new_trigger).insert(EthAddress(*address));
                    println!("Making new trigger with address {:?}", address);
                }
            })
        };
    })
}
