use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct AbstractVillager {
    pub ageable_mob: AgeableMob,
    pub head_shake_timer: u32,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractVillager, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Villager {
    pub abstract_villager: AbstractVillager,
    pub villager_data: Vec<u8>,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractVillager, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct WanderingTrader {
    pub abstract_villager: AbstractVillager,
}
