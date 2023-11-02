use super::*;

#[derive(Default)]
#[inherit(PathfinderMob, Mob, LivingEntity, Entity)]
pub struct AgeableMob {
    pub is_baby: bool,
}
