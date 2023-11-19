use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Mob, LivingEntity, Entity },
)]
pub struct Slime {
    pub mob: Mob,
    pub size: usize,
}
