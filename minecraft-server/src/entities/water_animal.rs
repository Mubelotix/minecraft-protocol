use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(PathfinderMob, Mob, LivingEntity, Entity)]
pub struct WaterAnimal {
    pub pathfinder_mob: PathfinderMob,
}

#[derive(Default)]
#[inherit(WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Dolphin {
    pub water_animal: WaterAnimal,
    pub treasure_position: Option<Position>,
    pub has_fish: bool,
    pub moisture_level: usize,
}
