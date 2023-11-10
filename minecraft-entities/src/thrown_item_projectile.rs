use super::*;

#[MinecraftEntity(
    inheritable, parents { Entity },
)]
pub struct ThrownItemProjectile {
    pub entity: Entity,
    pub item: Slot,
}

impl Default for ThrownItemProjectile {
    fn default() -> Self {
        ThrownItemProjectile {
            entity: Entity::default(),
            item: Slot {item: None},
        }
    }
}

#[MinecraftEntity(
    parents { ThrownItemProjectile, Entity },
)]
pub struct ThrownEgg {
    pub thrown_item_projectile: ThrownItemProjectile,
}

impl Default for ThrownEgg {
    fn default() -> Self {
        ThrownEgg {
            thrown_item_projectile: ThrownItemProjectile {
                entity: Entity::default(),
                item: Slot {item: Some(SlotItem {
                    item_id: Item::Egg,
                    item_count: 1,
                    nbt_data: NbtTag::Null
                })},
            }
        }
    }
}

#[MinecraftEntity(
    parents { ThrownItemProjectile, Entity },
)]
pub struct ThrownEnderPearl {
    pub thrown_item_projectile: ThrownItemProjectile,
}

impl Default for ThrownEnderPearl {
    fn default() -> Self {
        ThrownEnderPearl {
            thrown_item_projectile: ThrownItemProjectile {
                entity: Entity::default(),
                item: Slot {item: Some(SlotItem {
                    item_id: Item::EnderPearl,
                    item_count: 1,
                    nbt_data: NbtTag::Null
                })},
            }
        }
    }
}

#[MinecraftEntity(
    parents { ThrownItemProjectile, Entity },
)]
pub struct ThrownExperienceBottle {
    pub thrown_item_projectile: ThrownItemProjectile,
}

impl Default for ThrownExperienceBottle {
    fn default() -> Self {
        ThrownExperienceBottle {
            thrown_item_projectile: ThrownItemProjectile {
                entity: Entity::default(),
                item: Slot {item: Some(SlotItem {
                    item_id: Item::ExperienceBottle,
                    item_count: 1,
                    nbt_data: NbtTag::Null
                })},
            }
        }
    }
}

#[MinecraftEntity(
    parents { ThrownItemProjectile, Entity },
)]
pub struct ThrownPotion {
    pub thrown_item_projectile: ThrownItemProjectile,
}

impl Default for ThrownPotion {
    fn default() -> Self {
        ThrownPotion {
            thrown_item_projectile: ThrownItemProjectile {
                entity: Entity::default(),
                item: Slot {item: Some(SlotItem {
                    item_id: Item::SplashPotion,
                    item_count: 1,
                    nbt_data: NbtTag::Null
                })},
            }
        }
    }
}

#[MinecraftEntity(
    parents { ThrownItemProjectile, Entity },
)]
pub struct Snowball {
    pub thrown_item_projectile: ThrownItemProjectile,
}

impl Default for Snowball {
    fn default() -> Self {
        Snowball {
            thrown_item_projectile: ThrownItemProjectile {
                entity: Entity::default(),
                item: Slot {item: Some(SlotItem {
                    item_id: Item::Snowball,
                    item_count: 1,
                    nbt_data: NbtTag::Null
                })},
            }
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Entity },
)]
pub struct EyeOfEnder {
    pub entity: Entity,
    pub item: Slot,
}

