use bevy::tasks::AsyncComputeTaskPool;

use super::components::CurrentLogsFetch;
use crate::prelude::*;

pub fn fetch_logs(
    mut commands: Commands,
    init_block: ResMut<FromBlock>,
    provider: Res<Provider<Http>>,
    log_query: Query<(Entity, &CurrentLogsFetch)>,
) {
    // if query exists, there is still a live query. Don't start a new one
    if !log_query.is_empty() {
        return;
    }

    let from_block = init_block.0.clone();
    // TODO: Use 2k or latest block minus reorg length
    let to_block = from_block + 2000;
    let filter = Filter::new().from_block(from_block).to_block(to_block);

    let provider_cl = provider.clone();
    let thread_pool = AsyncComputeTaskPool::get();

    println!(
        "fetching logs from block {:?} to block {:?}",
        from_block, to_block
    );

    let task = thread_pool.spawn(async move { provider_cl.get_logs(&filter).await }.to_compat());

    // The handle_logs system wont get this entity until the next cycle but that is fine
    commands.spawn().insert(CurrentLogsFetch {
        task,
        from_block,
        to_block,
    });
}
