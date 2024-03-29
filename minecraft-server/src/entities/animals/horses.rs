use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
    descendants { Horse, ZombieHorse, SkeletonHorse, Camel, ChestedHorse... },
)]
pub struct AbstractHorse {
    pub animal: Animal,
    pub mask: u8,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Horse {
    pub abstract_horse: AbstractHorse,
    pub variant: usize,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct ZombieHorse {
    pub abstract_horse: AbstractHorse,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct SkeletonHorse {
    pub abstract_horse: AbstractHorse,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Camel {
    pub abstract_horse: AbstractHorse,
    pub is_dashing: bool,
    pub last_pose_change_tick: usize,
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
    descendants { Mule, Donkey, Llama... },
)]
pub struct ChestedHorse {
    pub abstract_horse: AbstractHorse,
    pub has_chest: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { ChestedHorse, AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Mule {
    pub chested_horse: ChestedHorse,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { ChestedHorse, AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Donkey {
    pub chested_horse: ChestedHorse,
}

#[MinecraftEntity(
    inheritable,
    ancestors { ChestedHorse, AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
    descendants { TraderLlama },
)]
pub struct Llama {
    pub chested_horse: ChestedHorse,
    /// Strength (number of columns of 3 slots in the llama's inventory once a chest is equipped)
    pub stength: u8,
    /// Carpet color (a dye color, or -1 if no carpet equipped)
    pub carpet_color: i16,
    pub variant: u8,
}

impl Default for Llama {
    fn default() -> Self {
        Self {
            chested_horse: ChestedHorse::default(),
            stength: 0,
            carpet_color: -1,
            variant: 0,
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Llama, ChestedHorse, AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct TraderLlama {
    pub llama: Llama,
}


#[derive(Default)]
#[MinecraftEntity(
    ancestors { Entity },
)]
pub struct LlamaSpit {
    pub entity: Entity,
}
