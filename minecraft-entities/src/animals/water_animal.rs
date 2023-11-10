use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct WaterAnimal {
    pub pathfinder_mob: PathfinderMob,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Dolphin {
    pub water_animal: WaterAnimal,
    pub treasure_position: Option<Position>,
    pub has_fish: bool,
    pub moisture_level: usize,
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct AbstractFish {
    pub water_animal: WaterAnimal,
    pub from_bucket: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Cod {
    pub abstract_fish: AbstractFish,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct PufferFish {
    pub abstract_fish: AbstractFish,
    pub puff_state: usize,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Salmon {
    pub abstract_fish: AbstractFish,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct TropicalFish {
    pub abstract_fish: AbstractFish,
    pub variant: usize,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Tadpole {
    pub abstract_fish: AbstractFish,
}
