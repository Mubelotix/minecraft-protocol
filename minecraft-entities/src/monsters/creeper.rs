use super::*;

#[MinecraftEntity(
    parents { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Creeper {
    pub monster: Monster,
    pub state: i8,
    pub is_charged: bool,
    pub is_ignited: bool,
}

impl Default for Creeper {
    fn default() -> Self {
        Self {
            monster: Monster::default(),
            state: -1,
            is_charged: false,
            is_ignited: false,
        }
    }
}
