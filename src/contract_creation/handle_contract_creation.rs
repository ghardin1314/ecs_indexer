use ethers::abi::{RawLog, Token};

use crate::{poll_logs::ActionFired, prelude::*};

pub fn handle_contract_creation(
    actions_query: Query<(&TriggerAction, &CreateContractAction)>,
    templates_query: Query<&Children, With<ContractTemplate>>,
    event_triggers_query: Query<Entity, With<EventTrigger>>,
    mut events: EventReader<ActionFired>,
    mut commands: Commands,
) {
    events.iter().for_each(|event| {
        if let Ok((_, create_contract)) = actions_query.get(event.action) {
            println!("action triggered: {:?}", { create_contract });

            // parse log from event
            let log = create_contract
                .event
                .parse_log(RawLog {
                    topics: event.log.topics.clone(),
                    data: event.log.data.to_vec(),
                })
                .unwrap();

            // Get address from event param
            let address = &log
                .params
                .iter()
                .find(|param| param.name == create_contract.field)
                .unwrap()
                .value;

            let from_block = event.log.block_number.unwrap();

            // get children of template
            let children = templates_query
                .get(create_contract.template)
                .expect("Found create contract action with no template");

            children.iter().for_each(|child| {
                let event_trigger = event_triggers_query
                    .get(*child)
                    .expect("Missing event trigger from template");

                // Make Trigger
                commands
                    .entity(event_trigger)
                    .insert(ActiveTrigger)
                    .insert(TriggerStartBlock(from_block));

                if let Token::Address(address) = address {
                    commands.entity(event_trigger).insert(EthAddress(*address));
                }
            })
        };
    })
}
