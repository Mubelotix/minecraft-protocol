use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Mob, LivingEntity, Entity)]
pub struct Slime {
    pub mob: Mob,
    pub size: usize,
}
