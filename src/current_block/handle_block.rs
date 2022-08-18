use crate::prelude::*;

use super::resources::{CurrentBlock, CurrentBlockReceiver};

pub fn handle_block(
    receiver: ResMut<CurrentBlockReceiver>,
    mut commands: Commands,
    current_block: Res<CurrentBlock>,
) {
    for block in receiver.try_iter() {
        if let Some(block_number) = block.number {
            println!("Got block: {:?}", block.number);
            // If there is a reorg, we will get a lower blocknumber
            // Since we are just polling lagged by the max expected reorg, we dont need to deal with it here.
            // TODO: Log reorg occurance. Possibly panic if reorg is longer than our max expected.
            if block_number.gt(&current_block.0) {
                commands.insert_resource(CurrentBlock(block_number));
            }
        }
    }
}
