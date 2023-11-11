use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
    descendants { Mooshroom },
)]
pub struct Cow {
    pub animal: Animal,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Cow, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Mooshroom {
    pub cow: Cow,
    pub variant: u8, // In the doc it is a string 
}
