use crossbeam_channel::{unbounded, Receiver};

use crate::prelude::*;

#[derive(Deref)]
pub struct CurrentBlockReceiver(Receiver<Block<H256>>);



pub fn initiate_block_subscription(mut commands: Commands, provider: Res<Provider<Ws>>) {
    let (tx, rx) = unbounded::<Block<H256>>();

    let provider_cl = provider.clone();

    std::thread::spawn(move || {
        async {
            let blocks = provider_cl.subscribe_blocks().await.unwrap();

            blocks
                .for_each(|block| async {
                    tx.send(block).unwrap();
                })
                .await;
        }
        .compat_await();
    });

    commands.insert_resource(CurrentBlockReceiver(rx));
}

pub fn handle_new_block(receiver: ResMut<CurrentBlockReceiver>, mut commands: Commands) {
    for block in receiver.try_iter() {
        if let Some(block_number) = block.number {
            print!("Got block: {:?}", block);
            commands.insert_resource(CurrentBlock(block_number));
        }
    }
}
