use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct AbstractFish {
    pub water_animal: WaterAnimal,
    pub from_bucket: bool,
}


#[derive(Default)]
#[inherit(AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Cod {
    pub abstract_fish: AbstractFish,
}

#[derive(Default)]
#[inherit(AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct PufferFish {
    pub abstract_fish: AbstractFish,
    pub puff_state: usize,
}

#[derive(Default)]
#[inherit(AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Salmon {
    pub abstract_fish: AbstractFish,
}

#[derive(Default)]
#[inherit(AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct TropicalFish {
    pub abstract_fish: AbstractFish,
    pub variant: usize,
}

#[derive(Default)]
#[inherit(AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Tadpole {
    pub abstract_fish: AbstractFish,
}
