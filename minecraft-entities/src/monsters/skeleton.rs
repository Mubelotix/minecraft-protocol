use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct AbstractSkeleton {
    pub monster: Monster,
}

impl TryAsEntityRef<AbstractSkeleton> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&AbstractSkeleton> {
        match self {
            AnyEntity::AbstractSkeleton(abstract_skeleton) => Some(&abstract_skeleton),
            AnyEntity::Skeleton(skeleton) => Some(&skeleton.abstract_skeleton),
            AnyEntity::WitherSkeleton(wither_skeleton) => Some(&wither_skeleton.abstract_skeleton),
            AnyEntity::Stray(stray) => Some(&stray.abstract_skeleton),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut AbstractSkeleton> {
        match self {
            AnyEntity::AbstractSkeleton(abstract_skeleton) => Some(abstract_skeleton),
            AnyEntity::Skeleton(skeleton) => Some(&mut skeleton.abstract_skeleton),
            AnyEntity::WitherSkeleton(wither_skeleton) => Some(&mut wither_skeleton.abstract_skeleton),
            AnyEntity::Stray(stray) => Some(&mut stray.abstract_skeleton),
            _ => None,
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractSkeleton, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Skeleton {
    pub abstract_skeleton: AbstractSkeleton,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractSkeleton, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct WitherSkeleton {
    pub abstract_skeleton: AbstractSkeleton,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractSkeleton, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Stray {
    pub abstract_skeleton: AbstractSkeleton,
}
