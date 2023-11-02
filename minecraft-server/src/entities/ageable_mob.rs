use super::*;

#[inherit(PathfinderMob, Mob, LivingEntity, Entity)]
pub struct AgeableMob {
    pub is_baby: bool,
}

impl Default for AgeableMob {
    fn default() -> Self {
        Self { is_baby: false }
    }
}
