use crate::ecs::components::*;
use tags_macros::tags;

tags! {
    Player {
        Position,
        Health,
    }
    Enemy {
        Health
    }
}