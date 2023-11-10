use super::*;

#[derive(Default)]
#[MinecraftEntity(
    parents { Mob, LivingEntity, Entity },
)]
pub struct Slime {
    pub mob: Mob,
    pub size: usize,
}
