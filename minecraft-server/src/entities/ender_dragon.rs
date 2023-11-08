use super::*;

#[inherit(Mob, LivingEntity, Entity)]
pub struct EnderDragon {
    pub mob: Mob,
    pub phase: usize,
}

impl Default for EnderDragon {
    fn default() -> Self {
        Self {
            mob: Mob::default(),
            phase: 10,
        }
    }
}
