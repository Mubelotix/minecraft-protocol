use super::*;

#[derive(Default)]
#[MinecraftEntity(
    parents { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Pig {
    pub animal: Animal,
    pub has_saddle: bool,
    pub boost_time: u16,
}
