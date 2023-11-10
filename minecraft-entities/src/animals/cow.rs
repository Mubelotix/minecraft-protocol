use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Cow {
    pub animal: Animal,
}

impl TryAsEntityRef<Cow> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&Cow> {
        match self {
            AnyEntity::Cow(cow) => return Some(&cow),
            AnyEntity::Mooshroom(mooshroom) => return Some(&mooshroom.cow),
            _ => (),
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Cow, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Mooshroom {
    pub cow: Cow,
    pub variant: u8, // In the doc it is a string 
}
