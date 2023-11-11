use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Entity },
)]
pub struct DragonFireball {
    pub entity: Entity,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Entity },
)]
pub struct SmallFireball {
    pub entity: Entity,
    pub item: Slot,
}

#[derive(Default)]
#[MinecraftEntity(
     ancestors { Entity },
)]
pub struct Fireball {
    pub entity: Entity,
    pub item: Slot,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Entity },
)]
pub struct FireworkRocket {
    pub entity: Entity,
    pub item: Slot,
    pub used_by: Option<Eid>,
    pub is_shot_at_angle: bool,
}
