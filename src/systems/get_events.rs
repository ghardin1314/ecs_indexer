use async_compat::Compat;
use bevy::tasks::AsyncComputeTaskPool;
use crossbeam_channel::{bounded, Receiver};
use futures_lite::{future, Future};

use crate::prelude::*;

#[derive(Deref)]
pub struct CurrentBlockReceiver(Receiver<U64>);

pub fn query_block(mut commands: Commands, provider: Res<Provider<Http>>) {
    let provider_cl = provider.clone();
    let thread_pool = AsyncComputeTaskPool::get();

    // `Compat` here transforms a tokio future into a generic future. Not sure effect on performance
    let task = thread_pool.spawn(Compat::new(async move {
        provider_cl.get_block_number().await.unwrap()
    }));

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

pub fn query_block_loop(mut commands: Commands, provider: Res<Provider<Http>>) {
    let provider_cl = provider.clone();

    let (tx, rx) = bounded::<U64>(10);

    std::thread::spawn(move || loop {
        let block_number = await_ft(provider_cl.get_block_number()).unwrap();

        tx.send(block_number).unwrap();
        let on_sec = std::time::Duration::from_secs(5);
        std::thread::sleep(on_sec);
    });

    commands.insert_resource(CurrentBlockReceiver(rx));
}

pub fn handle_block_loop(mut commands: Commands, receiver: ResMut<CurrentBlockReceiver>) {
    for block_number in receiver.try_iter() {
        println!("Current loop block: {:?}", block_number);
    }
}


fn await_ft<T>(ft: impl Future<Output = T>) -> T {
	future::block_on(Compat::new(ft))
}