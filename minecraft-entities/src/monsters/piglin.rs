use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct BasePiglin {
    pub monster: Monster,
    pub is_immune: bool,
}

impl TryAsEntityRef<BasePiglin> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&BasePiglin> {
        match self {
            AnyEntity::BasePiglin(base_piglin) => Some(&base_piglin),
            AnyEntity::Piglin(piglin) => Some(&piglin.base_piglin),
            AnyEntity::PiglinBrute(piglin_brute) => Some(&piglin_brute.base_piglin),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut BasePiglin> {
        match self {
            AnyEntity::BasePiglin(base_piglin) => Some(base_piglin),
            AnyEntity::Piglin(piglin) => Some(&mut piglin.base_piglin),
            AnyEntity::PiglinBrute(piglin_brute) => Some(&mut piglin_brute.base_piglin),
            _ => None,
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { BasePiglin, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Piglin {
    pub base_piglin: BasePiglin,
    pub is_baby: bool,
    pub is_charging_crossbow: bool,
    pub is_dancing: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { BasePiglin, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct PiglinBrute {
    pub base_piglin: BasePiglin,
}
