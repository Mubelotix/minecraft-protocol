use super::*;

#[derive(Default)]
#[MinecraftEntity(
    parents { AmbientCreature, Mob, LivingEntity, Entity },
)]
pub struct Bat {
    pub ambient_creature: AmbientCreature,
    pub is_hanging: bool,
}
