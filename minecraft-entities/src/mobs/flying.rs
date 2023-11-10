use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Mob, LivingEntity, Entity },
)]
pub struct Flying {
    pub mob: Mob,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Flying, Mob, LivingEntity, Entity },
)]
pub struct Ghast {
    pub flying: Flying,
    pub is_attacking: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Flying, Mob, LivingEntity, Entity },
)]
pub struct Phantom {
    pub flying: Flying,
    pub size: usize,
}
