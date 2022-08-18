use crate::prelude::*;

pub struct NewLog(pub Log);

#[derive(Debug)]
pub struct ActionFired {
    pub action: Entity,
    pub log: Log,
}
