use crate::prelude::*;

/// Current starting block of log polling
#[derive(Default, Debug)]
pub struct FromBlock(pub U64);

/// Maximum expected reorg length
#[derive(Default)]
pub struct ReorgBlocks(pub U64);
