use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(PathfinderMob, Mob, LivingEntity, Entity)]
pub struct AgeableMob {
    pub pathfinder_mob: PathfinderMob,
    pub is_baby: bool,
}
