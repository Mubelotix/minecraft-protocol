use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Mob, LivingEntity, Entity)]
pub struct PathfinderMob {
    pub mob: Mob,
}
