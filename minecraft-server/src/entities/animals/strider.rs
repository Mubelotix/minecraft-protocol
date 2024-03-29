use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Strider {
    pub animal: Animal,
    pub boost_time: u16,
    pub is_shaking: bool,
    pub has_saddle: bool,
}
