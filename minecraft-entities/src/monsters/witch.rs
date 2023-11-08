use super::*;

#[derive(Default)]
#[inherit(Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Witch {
    pub raider: Raider,
    pub is_drinking_potion: bool,
}
