use crate::prelude::*;

pub struct NewLog(pub Log);

pub struct ActionFired {
    pub action: Entity,
    pub log: Log,
}
