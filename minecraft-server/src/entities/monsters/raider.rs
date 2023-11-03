use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Raider {
    pub monster: Monster,
    pub is_celebrating: bool,
}

#[derive(Default)]
#[inheritable]
#[inherit(Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct AbstractIllager {
    pub raider: Raider,
}
