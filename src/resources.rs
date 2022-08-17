

use crate::prelude::*;
#[derive(Default, Debug)]
pub struct FromBlock(pub U64);

#[derive(Default)]
pub struct ReorgBlocks(pub U64);

#[derive(Default)]
pub struct PollingLogs(pub bool);

#[derive(Default)]
pub struct CurrentBlock(pub U64);


