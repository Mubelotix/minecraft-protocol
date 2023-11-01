use crate::components::*;
use ecs_macros::tags;

tags! {
    Player {
        Position,
        Health,
    }
    Enemy {
        Health
    }
}
