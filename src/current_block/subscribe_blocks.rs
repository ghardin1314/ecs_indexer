use crossbeam_channel::unbounded;

use crate::prelude::*;

use super::{resources::CurrentBlockReceiver, CurrentBlock};

/// Initiate with current block and subscribe to new block updates
pub fn subscribe_blocks(
    mut commands: Commands,
    provider_ws: Res<Provider<Ws>>,
    provider_http: Res<Provider<Http>>,
) {
    let (tx, rx) = unbounded::<Block<H256>>();

    // TODO: Error handling
    // Initiate with current block
    let current_block = provider_http.get_block_number().compat_await().unwrap();
    commands.insert_resource(CurrentBlock(current_block));

    let provider_cl = provider_ws.clone();

    std::thread::spawn(move || {
        async move {
            // TODO: Error handling and reconnecting
            let blocks = provider_cl.subscribe_blocks().await.unwrap();

            blocks
                .for_each(|block| async {
                    // TODO: Error handling
                    tx.send(block).unwrap();
                })
                .await;
        }
        .compat_await()
    });

    commands.insert_resource(CurrentBlockReceiver(rx))
}
