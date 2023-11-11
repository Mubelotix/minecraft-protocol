use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, ancestors { AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct AbstractVillager {
    pub ageable_mob: AgeableMob,
    pub head_shake_timer: u32,
}

impl TryAsEntityRef<AbstractVillager> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&AbstractVillager> {
        match self {
            AnyEntity::AbstractVillager(abstract_villager) => return Some(&abstract_villager),
            AnyEntity::Villager(villager) => return Some(&villager.abstract_villager),
            AnyEntity::WanderingTrader(wandering_trader) => return Some(&wandering_trader.abstract_villager),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut AbstractVillager> {
        match self {
            AnyEntity::AbstractVillager(abstract_villager) => return Some(abstract_villager),
            AnyEntity::Villager(villager) => return Some(&mut villager.abstract_villager),
            AnyEntity::WanderingTrader(wandering_trader) => return Some(&mut wandering_trader.abstract_villager),
            _ => None,
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractVillager, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Villager {
    pub abstract_villager: AbstractVillager,
    pub villager_data: Vec<u8>,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractVillager, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct WanderingTrader {
    pub abstract_villager: AbstractVillager,
}
