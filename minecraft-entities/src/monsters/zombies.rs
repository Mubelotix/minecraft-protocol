use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, ancestors { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Zombie {
    pub monster: Monster,
    pub is_baby: bool,
    pub unused: isize,
    pub is_becoming_drowned: bool,
}

impl TryAsEntityRef<Zombie> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&Zombie> {
        match self {
            AnyEntity::Zombie(zombie) => Some(&zombie),
            AnyEntity::ZombieVillager(zombie_villager) => Some(&zombie_villager.zombie),
            AnyEntity::Husk(husk) => Some(&husk.zombie),
            AnyEntity::Drowned(drowned) => Some(&drowned.zombie),
            AnyEntity::ZombifiedPiglin(zombified_piglin) => Some(&zombified_piglin.zombie),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut Zombie> {
        match self {
            AnyEntity::Zombie(zombie) => Some(zombie),
            AnyEntity::ZombieVillager(zombie_villager) => Some(&mut zombie_villager.zombie),
            AnyEntity::Husk(husk) => Some(&mut husk.zombie),
            AnyEntity::Drowned(drowned) => Some(&mut drowned.zombie),
            AnyEntity::ZombifiedPiglin(zombified_piglin) => Some(&mut zombified_piglin.zombie),
            _ => None,
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Zombie, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct ZombieVillager {
    pub zombie: Zombie,
    pub is_converting: bool,
    pub villager_data: Vec<u8>,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Zombie, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Husk {
    pub zombie: Zombie,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Zombie, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Drowned {
    pub zombie: Zombie,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Zombie, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct ZombifiedPiglin {
    pub zombie: Zombie,
}
