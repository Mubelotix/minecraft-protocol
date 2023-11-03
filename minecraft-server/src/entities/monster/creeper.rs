use super::*;

#[derive(Default)]
#[inherit(Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Creeper {
    pub monster: Monster,
    pub state: i8,
    pub is_charged: bool,
    pub is_ignited: bool,
}
