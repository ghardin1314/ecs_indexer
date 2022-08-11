use crate::prelude::*;
use crossbeam_channel::{unbounded, Receiver};
use ethers::types::{Filter, Log};
use futures_lite::{future, Future};

#[derive(Deref)]
pub struct CurrentLogsReceiver(Receiver<Log>);

pub fn create_logs_subscription(mut commands: Commands, provider: Res<Provider<Ws>>) {
    let (tx, rx) = unbounded::<Log>();

    let provider_cl = provider.clone();

    println!("creating subscription");

    std::thread::spawn(move || {
        async {
            println!("spawned subscription");

            let filter = Filter::new();

            let logs = provider_cl.subscribe_logs(&filter).await.unwrap();

            println!("spawned subscription");

            logs.for_each(|log| async {
                tx.send(log).unwrap();
            })
            .await;
        }
        .compat_await();
    });

    println!("creating resource");

    commands.insert_resource(CurrentLogsReceiver(rx));
}

pub fn handle_new_logs(receiver: ResMut<CurrentLogsReceiver>) {
    for log in receiver.try_iter() {
        // print!("Got log: {:?}", log);
    }
}
