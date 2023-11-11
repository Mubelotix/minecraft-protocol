use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, ancestors { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Cow {
    pub animal: Animal,
}

impl TryAsEntityRef<Cow> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&Cow> {
        match self {
            AnyEntity::Cow(cow) => return Some(&cow),
            AnyEntity::Mooshroom(mooshroom) => return Some(&mooshroom.cow),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut Cow> {
        match self {
            AnyEntity::Cow(cow) => return Some(cow),
            AnyEntity::Mooshroom(mooshroom) => return Some(&mut mooshroom.cow),
            _ => None,
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Cow, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Mooshroom {
    pub cow: Cow,
    pub variant: u8, // In the doc it is a string 
}
