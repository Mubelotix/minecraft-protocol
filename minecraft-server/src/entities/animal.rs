use super::*;

#[derive(Default)]
#[inherit(AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Animal;

