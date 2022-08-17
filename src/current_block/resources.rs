use crossbeam_channel::Receiver;

use crate::prelude::*;

#[derive(Deref)]
pub struct CurrentBlockReceiver(pub Receiver<Block<H256>>);

#[derive(Default)]
pub struct CurrentBlock(pub U64);
