use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct WaterAnimal {
    pub pathfinder_mob: PathfinderMob,
}

impl TryAsEntityRef<WaterAnimal> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&WaterAnimal> {
        match self {
            AnyEntity::WaterAnimal(water_animal) => return Some(&water_animal),
            AnyEntity::Dolphin(dolphin) => return Some(&dolphin.water_animal),
            AnyEntity::Squid(squid) => return Some(&squid.water_animal),
            _ => (),
        }
        if let Some(fish) = self.try_as_entity_ref::<AbstractFish>() {
            return Some(fish.get_water_animal())
        }
        None
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut WaterAnimal> {
        match self {
            AnyEntity::WaterAnimal(water_animal) => return Some(water_animal),
            AnyEntity::Dolphin(dolphin) => return Some(&mut dolphin.water_animal),
            AnyEntity::Squid(squid) => return Some(&mut squid.water_animal),
            _ => (),
        }
        if let Some(fish) = self.try_as_entity_mut::<AbstractFish>() {
            return Some(fish.get_water_animal_mut())
        }
        None
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Dolphin {
    pub water_animal: WaterAnimal,
    pub treasure_position: Option<Position>,
    pub has_fish: bool,
    pub moisture_level: usize,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Squid {
    pub water_animal: WaterAnimal,
}

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct AbstractFish {
    pub water_animal: WaterAnimal,
    pub from_bucket: bool,
}

impl TryAsEntityRef<AbstractFish> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&AbstractFish> {
        match self {
            AnyEntity::AbstractFish(abstract_fish) => Some(&abstract_fish),
            AnyEntity::Cod(cod) => Some(&cod.abstract_fish),
            AnyEntity::PufferFish(puffer_fish) => Some(&puffer_fish.abstract_fish),
            AnyEntity::Salmon(salmon) => Some(&salmon.abstract_fish),
            AnyEntity::TropicalFish(tropical_fish) => Some(&tropical_fish.abstract_fish),
            AnyEntity::Tadpole(tadpole) => Some(&tadpole.abstract_fish),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut AbstractFish> {
        match self {
            AnyEntity::AbstractFish(abstract_fish) => Some(abstract_fish),
            AnyEntity::Cod(cod) => Some(&mut cod.abstract_fish),
            AnyEntity::PufferFish(puffer_fish) => Some(&mut puffer_fish.abstract_fish),
            AnyEntity::Salmon(salmon) => Some(&mut salmon.abstract_fish),
            AnyEntity::TropicalFish(tropical_fish) => Some(&mut tropical_fish.abstract_fish),
            AnyEntity::Tadpole(tadpole) => Some(&mut tadpole.abstract_fish),
            _ => None,
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Cod {
    pub abstract_fish: AbstractFish,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct PufferFish {
    pub abstract_fish: AbstractFish,
    pub puff_state: usize,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Salmon {
    pub abstract_fish: AbstractFish,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct TropicalFish {
    pub abstract_fish: AbstractFish,
    pub variant: usize,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractFish, WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Tadpole {
    pub abstract_fish: AbstractFish,
}
