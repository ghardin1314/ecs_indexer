use bevy::tasks::AsyncComputeTaskPool;
use crossbeam_channel::{bounded, Receiver};
use futures_lite::future;

use crate::prelude::*;

#[derive(Deref)]
pub struct CurrentBlockReceiver(Receiver<U64>);

// pub fn query_block(mut commands: Commands, provider: Res<Provider<Http>>) {
//     let (tx, rx) = bounded::<U64>(10);

//     let provider_cl = provider.clone();
// 	println!("spawning task");

//     let ft = std::thread::spawn(move || async move {
//         println!("spawning task");
//         let block_number = provider_cl.get_block_number().await.unwrap();

//         tx.send(block_number).unwrap();
//         // }
//     }).join().unwrap();

//     commands.insert_resource(CurrentBlockReceiver(rx));
// }

// pub fn handle_block(receiver: ResMut<CurrentBlockReceiver>) {
//     for current_block in receiver.try_iter() {
//         println!("Current block: {:?}", current_block);
//     }
// }

pub fn query_block(mut commands: Commands, provider: Res<Provider<Http>>) {
    let provider_cl = provider.clone();
    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool.spawn(async move {
        let client = reqwest::blocking::Client::new();

        let data = "{\"id\": 1,\"jsonrpc\": \"2.0\",\"method\": \"eth_blockNumber\"}";

        let request = client
            .post("https://eth-mainnet.alchemyapi.io/v2/api-key")
            .body(data)
            .send()
            .unwrap()
            .text()
            .unwrap();

        println!("{}", request);

        let block_number: U64 = U64::from(1000);
        block_number

        // let block_number = provider_cl.get_block_number().await.unwrap();
        // block_number
    });

    commands.spawn().insert(CurrentBlockQuery(task));
}

pub fn handle_block(
    mut commands: Commands,
    mut block_queries: Query<(Entity, &mut CurrentBlockQuery)>,
) {
    for (entity, mut task) in &mut block_queries {
        if let Some(block_number) = future::block_on(future::poll_once(&mut task.0)) {
            println!("Current block: {:?}", block_number);

            commands.entity(entity).despawn();
        }
    }
}
