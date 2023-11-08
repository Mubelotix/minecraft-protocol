use super::*;

#[derive(Default)]
#[inherit(BasePiglin, Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Piglin {
    pub base_piglin: BasePiglin,
    pub is_baby: bool,
    pub is_charging_crossbow: bool,
    pub is_dancing: bool,
}

#[derive(Default)]
#[inherit(BasePiglin, Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct PiglinBrute {
    pub base_piglin: BasePiglin,
}
