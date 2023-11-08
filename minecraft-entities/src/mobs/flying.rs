use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Mob, LivingEntity, Entity)]
pub struct Flying {
    pub mob: Mob,
}

#[derive(Default)]
#[inherit(Flying, Mob, LivingEntity, Entity)]
pub struct Ghast {
    pub flying: Flying,
    pub is_attacking: bool,
}

#[derive(Default)]
#[inherit(Flying, Mob, LivingEntity, Entity)]
pub struct Phantom {
    pub flying: Flying,
    pub size: usize,
}
