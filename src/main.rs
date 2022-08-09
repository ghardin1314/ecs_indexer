mod components;
mod systems;

mod prelude {
    pub use crate::components::*;
    pub use crate::systems::*;
    pub use bevy::prelude::*;
    pub use serde::{Deserialize, Serialize};
}

use prelude::*;

fn main() {
    App::new()
        .add_startup_system(systems::load_config)
        .add_system(systems::read_events)
        .run();
}
