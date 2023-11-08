use super::*;

#[derive(Default)]
#[inherit(Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Zoglin {
    pub monster: Monster,
    pub is_baby: bool,
}
