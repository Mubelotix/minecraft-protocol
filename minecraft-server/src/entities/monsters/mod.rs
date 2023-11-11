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
    inheritable,
    ancestors { PathfinderMob, Mob, LivingEntity, Entity },
    descendants { Blaze, Creeper, Enderman, Endermite, Giant, Raider, Silverfish, Spider, Vex, Warden, Wither, Zoglin, Zombie, AbstractSkeleton...,BasePiglin..., Guardian... },
)]
pub struct Monster {
    pub pathfinder_mob: PathfinderMob,
}
