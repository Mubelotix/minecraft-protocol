use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Zombie {
    pub monster: Monster,
    pub is_baby: bool,
    pub unused: isize,
    pub is_becoming_drowned: bool,
}

#[derive(Default)]
#[inherit(Zombie, Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct ZombieVillager {
    pub zombie: Zombie,
    pub is_converting: bool,
    pub villager_data: Vec<u8>,
}

#[derive(Default)]
#[inherit(Zombie, Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Husk {
    pub zombie: Zombie,
}

#[derive(Default)]
#[inherit(Zombie, Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Drowned {
    pub zombie: Zombie,
}

#[derive(Default)]
#[inherit(Zombie, Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct ZombifiedPiglin {
    pub zombie: Zombie,
}
