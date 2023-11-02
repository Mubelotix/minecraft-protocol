use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Mob, LivingEntity, Entity)]
pub struct AmbientCreature {
    pub mob: Mob,
}
