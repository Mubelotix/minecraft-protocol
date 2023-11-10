use super::*;

mod villagers;
pub use villagers::*;
mod golems;
pub use golems::*;
mod ender_dragon;
pub use ender_dragon::*; 
mod slime;
pub use slime::*;
mod flying;
pub use flying::*;
mod bat;
pub use bat::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { LivingEntity, Entity },
)]
pub struct Mob {
    pub living_entity: LivingEntity,
    pub no_ai: bool,
    pub is_left_handed: bool,
    pub is_aggressive: bool,
}

impl TryAsEntityRef<Mob> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&Mob> {
        match self {
            AnyEntity::Mob(mob) => return Some(&mob),
            _ => (),
        }
        if let Some(ambient_creature) = self.try_as_entity_ref::<AmbientCreature>() {
            return ambient_creature.mob.try_as_entity_ref();
        }
        if let Some(pathfinder_mob) = self.try_as_entity_ref::<PathfinderMob>() {
            return pathfinder_mob.mob.try_as_entity_ref();
        }
        None
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut Mob> {
        match self {
            AnyEntity::Mob(mob) => return Some(mob),
            _ => (),
        }
        if let Some(ambient_creature) = self.try_as_entity_mut::<AmbientCreature>() {
            return ambient_creature.mob.try_as_entity_mut();
        }
        if let Some(pathfinder_mob) = self.try_as_entity_mut::<PathfinderMob>() {
            return pathfinder_mob.mob.try_as_entity_mut();
        }
        None
    }
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Mob, LivingEntity, Entity },
)]
pub struct AmbientCreature {
    pub mob: Mob,
}

impl TryAsEntityRef<AmbientCreature> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&AmbientCreature> {
        match self {
            AnyEntity::AmbientCreature(ambient_creature) => return Some(&ambient_creature),
            AnyEntity::Bat(bat) => return Some(&bat.ambient_creature),
            _ => (),
        }
        None
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut AmbientCreature> {
        match self {
            AnyEntity::AmbientCreature(ambient_creature) => return Some(ambient_creature),
            AnyEntity::Bat(bat) => return Some(&mut bat.ambient_creature),
            _ => (),
        }
        None
    }
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Mob, LivingEntity, Entity },
)]
pub struct PathfinderMob {
    pub mob: Mob,
}

impl TryAsEntityRef<PathfinderMob> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&PathfinderMob> {
        match self {
            AnyEntity::PathfinderMob(pathfinder_mob) => return Some(&pathfinder_mob),
            _ => (),
        }
        if let Some(ageable_mob) = self.try_as_entity_ref::<AgeableMob>() {
            return ageable_mob.pathfinder_mob.try_as_entity_ref();
        }
        None
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut PathfinderMob> {
        match self {
            AnyEntity::PathfinderMob(pathfinder_mob) => return Some(pathfinder_mob),
            _ => (),
        }
        if let Some(ageable_mob) = self.try_as_entity_mut::<AgeableMob>() {
            return ageable_mob.pathfinder_mob.try_as_entity_mut();
        }
        None
    }
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct AgeableMob {
    pub pathfinder_mob: PathfinderMob,
    pub is_baby: bool,
}

impl TryAsEntityRef<AgeableMob> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&AgeableMob> {
        match self {
            AnyEntity::AgeableMob(ageable_mob) => return Some(&ageable_mob),
            _ => (),
        }
        if let Some(villager) = self.try_as_entity_ref::<AbstractVillager>() {
            return villager.try_as_entity_ref();
        }
        None
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut AgeableMob> {
        match self {
            AnyEntity::AgeableMob(ageable_mob) => return Some(&mut ageable_mob),
            _ => (),
        }
        if let Some(villager) = self.try_as_entity_mut::<AbstractVillager>() {
            return villager.try_as_entity_mut();
        }
        None
    }
}
