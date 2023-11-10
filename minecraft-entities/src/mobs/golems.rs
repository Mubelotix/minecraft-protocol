use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct AbstractGolem {
    pub pathfinder_mob: PathfinderMob,
}

impl TryAsEntityRef<AbstractGolem> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&AbstractGolem> {
        match self {
            AnyEntity::AbstractGolem(abstract_golem) => Some(&abstract_golem),
            AnyEntity::IronGolem(iron_golem) => Some(&iron_golem.abstract_golem),
            AnyEntity::SnowGolem(snow_golem) => Some(&snow_golem.abstract_golem),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut AbstractGolem> {
        match self {
            AnyEntity::AbstractGolem(abstract_golem) => Some(abstract_golem),
            AnyEntity::IronGolem(iron_golem) => Some(&mut iron_golem.abstract_golem),
            AnyEntity::SnowGolem(snow_golem) => Some(&mut snow_golem.abstract_golem),
            _ => None,
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractGolem, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct IronGolem {
    pub abstract_golem: AbstractGolem,
    pub is_player_created: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractGolem, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct SnowGolem {
    pub abstract_golem: AbstractGolem,
    pub has_pumpkin_hat: bool,
}
