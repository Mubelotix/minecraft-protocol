use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Guardian {
    pub monster: Monster,
    pub is_retracting_spikes: bool,
    pub target_eid: Eid,
}

impl TryAsEntityRef<Guardian> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&Guardian> {
        match self {
            AnyEntity::Guardian(guardian) => Some(&guardian),
            AnyEntity::ElderGuardian(elder_guardian) => Some(&elder_guardian.guardian),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut Guardian> {
        match self {
            AnyEntity::Guardian(guardian) => Some(guardian),
            AnyEntity::ElderGuardian(elder_guardian) => Some(&mut elder_guardian.guardian),
            _ => None,
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Guardian, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct ElderGuardian {
    pub guardian: Guardian,
}

