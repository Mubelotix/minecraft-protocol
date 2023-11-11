use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Turtle {
    pub animal: Animal,
    pub block_position: BlockPosition,
    pub has_egg: bool,
    pub is_laying_egg: bool,
    pub travel_position: Option<BlockPosition>,
    pub is_going_home: bool,
}
