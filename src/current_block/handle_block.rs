use crate::prelude::*;

use super::resources::{CurrentBlock, CurrentBlockReceiver};

pub fn handle_block(receiver: ResMut<CurrentBlockReceiver>, mut commands: Commands) {
    for block in receiver.try_iter() {
        if let Some(block_number) = block.number {
            println!("Got block: {:?}", block.number);
            commands.insert_resource(CurrentBlock(block_number));
        }
    }
}
