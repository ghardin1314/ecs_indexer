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
