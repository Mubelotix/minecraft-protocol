use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Mob, LivingEntity, Entity },
)]
pub struct Flying {
    pub mob: Mob,
}

impl TryAsEntityRef<Flying> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&Flying> {
        match self {
            AnyEntity::Flying(flying) => Some(&flying),
            AnyEntity::Bat(bat) => Some(&bat.flying),
            AnyEntity::Ghast(ghast) => Some(&ghast.flying),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut Flying> {
        match self {
            AnyEntity::Flying(flying) => Some(flying),
            AnyEntity::Bat(bat) => Some(&mut bat.flying),
            AnyEntity::Ghast(ghast) => Some(&mut ghast.flying),
            _ => None,
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Flying, Mob, LivingEntity, Entity },
)]
pub struct Ghast {
    pub flying: Flying,
    pub is_attacking: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Flying, Mob, LivingEntity, Entity },
)]
pub struct Phantom {
    pub flying: Flying,
    pub size: usize,
}
