use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(PathfinderMob, Mob, LivingEntity, Entity)]
pub struct WaterAnimal {
    pub pathfinder_mob: PathfinderMob,
}
