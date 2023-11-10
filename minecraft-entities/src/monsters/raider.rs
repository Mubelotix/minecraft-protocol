use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Raider {
    pub monster: Monster,
    pub is_celebrating: bool,
}

impl TryAsEntityRef<Raider> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&Raider> {
        match self {
            AnyEntity::Raider(raider) => return Some(&raider),
            AnyEntity::Witch(witch) => return Some(&witch.raider),
            _ => (),
        }
        if let Some(abstract_illager) = <Self as TryAsEntityRef<AbstractIllager>>::try_as_entity_ref(self) {
            return Some(&abstract_illager.raider)
        }
        None
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut Raider> {
        match self {
            AnyEntity::Raider(raider) => return Some(raider),
            AnyEntity::Witch(witch) => return Some(&mut witch.raider),
            _ => (),
        }
        if let Some(abstract_illager) = <Self as TryAsEntityRef<AbstractIllager>>::try_as_entity_mut(self) {
            return Some(&mut abstract_illager.raider)
        }
        None
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Witch {
    pub raider: Raider,
    pub is_drinking_potion: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct AbstractIllager {
    pub raider: Raider,
}

impl TryAsEntityRef<AbstractIllager> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&AbstractIllager> {
        match self {
            AnyEntity::AbstractIllager(abstract_illager) => return Some(&abstract_illager),
            AnyEntity::Vindicator(vindicator) => return Some(&vindicator.abstract_illager),
            AnyEntity::Pillager(pillager) => return Some(&pillager.abstract_illager),
            _ => (),
        }
        if let Some(spellcaster_illager) = <Self as TryAsEntityRef<SpellcasterIllager>>::try_as_entity_ref(self) {
            return Some(&spellcaster_illager.abstract_illager)
        }
        None
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut AbstractIllager> {
        match self {
            AnyEntity::AbstractIllager(abstract_illager) => return Some(abstract_illager),
            AnyEntity::Vindicator(vindicator) => return Some(&mut vindicator.abstract_illager),
            AnyEntity::Pillager(pillager) => return Some(&mut pillager.abstract_illager),
            _ => (),
        }
        if let Some(spellcaster_illager) = <Self as TryAsEntityRef<SpellcasterIllager>>::try_as_entity_mut(self) {
            return Some(&mut spellcaster_illager.abstract_illager)
        }
        None
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractIllager, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Vindicator {
    pub abstract_illager: AbstractIllager,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractIllager, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Pillager {
    pub abstract_illager: AbstractIllager,
    pub is_charging: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { AbstractIllager, Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct SpellcasterIllager {
    pub abstract_illager:  AbstractIllager,
    pub spell: u8,
}

impl TryAsEntityRef<SpellcasterIllager> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&SpellcasterIllager> {
        match self {
            AnyEntity::SpellcasterIllager(spellcaster_illager) => Some(&spellcaster_illager),
            AnyEntity::Illusioner(illusioner) => Some(&illusioner.spellcaster_illager),
            AnyEntity::Ravager(ravager) => Some(&ravager.spellcaster_illager),
            AnyEntity::Evoker(evoker) => Some(&evoker.spellcaster_illager),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut SpellcasterIllager> {
        match self {
            AnyEntity::SpellcasterIllager(spellcaster_illager) => Some(spellcaster_illager),
            AnyEntity::Illusioner(illusioner) => Some(&mut illusioner.spellcaster_illager),
            AnyEntity::Ravager(ravager) => Some(&mut ravager.spellcaster_illager),
            AnyEntity::Evoker(evoker) => Some(&mut evoker.spellcaster_illager),
            _ => None,
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { SpellcasterIllager, AbstractIllager, Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Illusioner {
    pub spellcaster_illager:  SpellcasterIllager,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { SpellcasterIllager, AbstractIllager, Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Ravager {
    pub spellcaster_illager:  SpellcasterIllager,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { SpellcasterIllager, AbstractIllager, Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Evoker {
    pub spellcaster_illager:  SpellcasterIllager,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Entity },
)]
pub struct EvokerFangs {
    pub entity: Entity,
}
