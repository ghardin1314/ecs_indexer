use std::time::Instant;

use crossbeam_channel::{unbounded, Receiver};

use crate::prelude::*;

#[derive(Deref)]
pub struct PollLogsReceiver(Receiver<Log>);

pub struct NewLog(Log);

pub fn poll_logs(
    mut commands: Commands,
    start_block: Res<StartBlock>,
    provider: Res<Provider<Http>>,
) {
    let (tx, rx) = unbounded::<Log>();
    let provider_cl = provider.clone();

    let mut from_block = start_block.0.clone();

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

pub fn handle_event_triggers(mut events: EventReader<NewLog>, query: Query<&EventTrigger>) {
    for log in events.iter() {
        let matching_sigs: Vec<&EventTrigger> = query
            .iter()
            .filter(|trigger| {
                log.0
                    .topics
                    .first()
                    // Some events dont have a first topic?
                    .unwrap_or(&H256::default())
                    .eq(&trigger.event.signature())
            })
            .collect();

        matching_sigs
            .iter()
            .for_each(|trigger| println!("found event {:?} with signature matching {:?}", log.0, trigger))
    }
}
