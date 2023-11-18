use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { PathfinderMob, Mob, LivingEntity, Entity },
    descendants { Dolphin, Squid, AbstractFish... },
)]
pub struct WaterAnimal {
    pub pathfinder_mob: PathfinderMob,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Dolphin {
    pub water_animal: WaterAnimal,
    pub treasure_position: Option<Position>,
    pub has_fish: bool,
    pub moisture_level: usize,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Squid {
    pub water_animal: WaterAnimal,
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
    descendants { Cod, Pufferfish, Salmon, TropicalFish, Tadpole... },
)]
pub struct AbstractFish {
    pub water_animal: WaterAnimal,
    pub from_bucket: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Cod {
    pub abstract_fish: AbstractFish,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Pufferfish {
    pub abstract_fish: AbstractFish,
    pub puff_state: usize,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Salmon {
    pub abstract_fish: AbstractFish,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct TropicalFish {
    pub abstract_fish: AbstractFish,
    pub variant: usize,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Tadpole {
    pub abstract_fish: AbstractFish,
}
