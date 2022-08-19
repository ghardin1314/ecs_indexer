use crate::prelude::*;

pub trait AwaitFt {
    fn compat_await(self) -> <Self as Future>::Output
    where
        Self: Future + Sized,
    {
        future::block_on(self.to_compat())
    }

    fn to_compat(self) -> async_compat::Compat<Self>
    where
        Self: Future + Sized,
    {
        Compat::new(self)
    }
}

impl<J: Future> AwaitFt for J {}

// trait AwaitFt {
//     fn await_ft<T>(self: &Self, ft: impl Future<Output = T>) -> T {
//         future::block_on(Compat::new(ft))
//     }
// }

// impl<T: Future> AwaitFt for T {
//     fn await_ft<U>(self: Self) -> U {
//         future::block_on(Compat::new(self))
//     }
// }

pub fn matches_signature(log: &Log, trigger: &&EventTrigger) -> bool {
    log.topics
        .first()
        // Some events dont have a first topic?
        .unwrap_or(&H256::default())
        .eq(&trigger.event.signature())
}

pub fn opt_matches_address(log: &Log, address: &Option<&EthAddress>) -> bool {
    match address {
        Some(address) => log.address.eq(&address.0),
        None => true,
    }
}

pub fn opt_past_start_block(log: &Log, start_block: &Option<&TriggerStartBlock>) -> bool {
    match start_block {
        Some(block) => log
            .block_number
            .map_or(true, |log_block| log_block.ge(&block.0)),
        None => true,
    }
}
