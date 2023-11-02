use crate::prelude::*;
use minecraft_ecs_macros::tags;

tags! {
    Player {
        PositionComponent,
        HealthComponent,
    }
    Enemy {
        HealthComponent,
    }
}
