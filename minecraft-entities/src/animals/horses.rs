use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct AbstractHorse {
    pub animal: Animal,
    pub mask: u8,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Horse {
    pub abstract_horse: AbstractHorse,
    pub variant: usize,
}

impl TryAsEntityRef<AbstractHorse> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&AbstractHorse> {
        match self {
            AnyEntity::Horse(horse) => return Some(&horse),
            AnyEntity::ZombieHorse(zombie_horse) => return Some(&zombie_horse.abstract_horse),
            AnyEntity::SkeletonHorse(skeleton_horse) => return Some(&skeleton_horse.abstract_horse),
            AnyEntity::Camel(camel) => return Some(&camel.abstract_horse),
            _ => (),
        }
        if let Some(chested_horse) = self.try_as_entity_ref::<ChestedHorse>() {
            return Some(chested_horse.get_abstract_horse())
        }
        None
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut AbstractHorse> {
        match self {
            AnyEntity::Horse(horse) => return Some(horse),
            AnyEntity::ZombieHorse(zombie_horse) => return Some(&mut zombie_horse.abstract_horse),
            AnyEntity::SkeletonHorse(skeleton_horse) => return Some(&mut skeleton_horse.abstract_horse),
            AnyEntity::Camel(camel) => return Some(&mut camel.abstract_horse),
            _ => (),
        }
        if let Some(chested_horse) = self.try_as_entity_mut::<ChestedHorse>() {
            return Some(chested_horse.get_abstract_horse_mut())
        }
        None
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct ZombieHorse {
    pub abstract_horse: AbstractHorse,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct SkeletonHorse {
    pub abstract_horse: AbstractHorse,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Camel {
    pub abstract_horse: AbstractHorse,
    pub is_dashing: bool,
    pub last_pose_change_tick: usize,
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct ChestedHorse {
    pub abstract_horse: AbstractHorse,
    pub has_chest: bool,
}

impl TryAsEntityRef<ChestedHorse> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&ChestedHorse> {
        match self {
            AnyEntity::ChestedHorse(chested_horse) => return Some(&chested_horse),
            AnyEntity::Mule(mule) => return Some(&mule.chested_horse),
            AnyEntity::Donkey(donkey) => return Some(&donkey.chested_horse),
            _ => (),
        }
        if let Some(lama) = self.try_as_entity_ref::<Llama>() {
            return Some(lama.get_chested_horse())
        }
        None
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut ChestedHorse> {
        match self {
            AnyEntity::ChestedHorse(chested_horse) => return Some(chested_horse),
            AnyEntity::Mule(mule) => return Some(&mut mule.chested_horse),
            AnyEntity::Donkey(donkey) => return Some(&mut donkey.chested_horse),
            _ => (),
        }
        if let Some(lama) = self.try_as_entity_mut::<Llama>() {
            return Some(lama.get_chested_horse_mut())
        }
        None
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { ChestedHorse, AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Mule {
    pub chested_horse: ChestedHorse,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { ChestedHorse, AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Donkey {
    pub chested_horse: ChestedHorse,
}

#[MinecraftEntity(
    inheritable, parents { ChestedHorse, AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Llama {
    pub chested_horse: ChestedHorse,
    /// Strength (number of columns of 3 slots in the llama's inventory once a chest is equipped)
    pub stength: u8,
    /// Carpet color (a dye color, or -1 if no carpet equipped)
    pub carpet_color: i16,
    pub variant: u8,
}

impl Default for Llama {
    fn default() -> Self {
        Self {
            chested_horse: ChestedHorse::default(),
            stength: 0,
            carpet_color: -1,
            variant: 0,
        }
    }
}

impl TryAsEntityRef<Llama> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&Llama> {
        match self {
            AnyEntity::Llama(llama) => Some(&llama),
            AnyEntity::TraderLlama(trader_llama) => Some(&trader_llama.llama),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut Llama> {
        match self {
            AnyEntity::Llama(llama) => Some(llama),
            AnyEntity::TraderLlama(trader_llama) => Some(&mut trader_llama.llama),
            _ => None,
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Llama, ChestedHorse, AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct TraderLlama {
    pub llama: Llama,
}


#[derive(Default)]
#[MinecraftEntity(
    parents { Entity },
)]
pub struct LlamaSpit {
    pub entity: Entity,
}
