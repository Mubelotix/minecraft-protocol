use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct AbstractVillager {
    pub ageable_mob: AgeableMob,
    pub head_shake_timer: u32,
}

#[derive(Default)]
#[inherit(AbstractVillager, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Villager {
    pub abstract_villager: AbstractVillager,
    pub villager_data: Vec<u8>,
}
