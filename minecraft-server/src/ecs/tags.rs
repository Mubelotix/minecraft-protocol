use crate::components::*;
use minecraft_ecs_macros::tags;

tags! {
    Player {
        Position,
        Health,
    }
    Enemy {
        Health
    }
}
