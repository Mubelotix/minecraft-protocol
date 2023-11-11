use super::*;

mod piglin;
pub use piglin::*;
mod blaze;
pub use blaze::*;
mod creeper;
pub use creeper::*;
mod endermite;
pub use endermite::*;
mod giant;
pub use giant::*;
mod guardian;
pub use guardian::*;
mod silverfish;
pub use silverfish::*;
mod raider;
pub use raider::*;
mod vex;
pub use vex::*;
mod skeleton;
pub use skeleton::*;
mod spider;
pub use spider::*;
mod warden;
pub use warden::*;
mod wither;
pub use wither::*;
mod zoglin;
pub use zoglin::*;
mod zombies;
pub use zombies::*;
mod enderman;
pub use enderman::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Monster {
    pub pathfinder_mob: PathfinderMob,
}

impl TryAsEntityRef<Monster> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&Monster> {
        match self {
            AnyEntity::Monster(monster) => return Some(&monster),
            _ => (),
        }
        if let Some(base_piglin) = <Self as TryAsEntityRef<BasePiglin>>::try_as_entity_ref(self) {
            return Some(&base_piglin.monster)
        }
        if let Some(guardian) = <Self as TryAsEntityRef<Guardian>>::try_as_entity_ref(self) {
            return Some(&guardian.monster)
        }
        None
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut Monster> {
        match self {
            AnyEntity::Monster(monster) => return Some(monster),
            _ => (),
        }
        if <Self as TryAsEntityRef<BasePiglin>>::try_as_entity_ref(self).is_some() {
            return <Self as TryAsEntityRef<BasePiglin>>::try_as_entity_mut(self).map(|piglin| &mut piglin.monster)
        }
        if <Self as TryAsEntityRef<Guardian>>::try_as_entity_ref(self).is_some() {
            return <Self as TryAsEntityRef<Guardian>>::try_as_entity_mut(self).map(|guardian| &mut guardian.monster)
        }
        None
    }
}

