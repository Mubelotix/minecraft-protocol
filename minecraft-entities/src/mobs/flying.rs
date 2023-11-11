use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { Mob, LivingEntity, Entity },
    descendants { Ghast, Phantom },
)]
pub struct Flying {
    pub mob: Mob,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Flying, Mob, LivingEntity, Entity },
)]
pub struct Ghast {
    pub flying: Flying,
    pub is_attacking: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Flying, Mob, LivingEntity, Entity },
)]
pub struct Phantom {
    pub flying: Flying,
    pub size: usize,
}
