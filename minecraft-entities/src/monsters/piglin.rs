use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { Monster, PathfinderMob, Mob, LivingEntity, Entity },
    descendants { Piglin, PiglinBrute },
)]
pub struct BasePiglin {
    pub monster: Monster,
    pub is_immune: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { BasePiglin, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Piglin {
    pub base_piglin: BasePiglin,
    pub is_baby: bool,
    pub is_charging_crossbow: bool,
    pub is_dancing: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { BasePiglin, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct PiglinBrute {
    pub base_piglin: BasePiglin,
}
