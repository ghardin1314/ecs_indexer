use crate::prelude::*;
use bevy::prelude::Query;

pub fn read_events(query: Query<&EventTrigger>) {
    for event_trigger in query.iter() {
        println!("found event: {:?}", event_trigger.event);
    }
}
