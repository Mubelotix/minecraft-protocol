use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct BasePiglin {
    pub monster: Monster,
    pub is_immune: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { BasePiglin, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Piglin {
    pub base_piglin: BasePiglin,
    pub is_baby: bool,
    pub is_charging_crossbow: bool,
    pub is_dancing: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { BasePiglin, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct PiglinBrute {
    pub base_piglin: BasePiglin,
}
