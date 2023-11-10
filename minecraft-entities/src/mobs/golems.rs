use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct AbstractGolem {
    pub pathfinder_mob: PathfinderMob,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractGolem, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct IronGolem {
    pub abstract_golem: AbstractGolem,
    pub is_player_created: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractGolem, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct SnowGolem {
    pub abstract_golem: AbstractGolem,
    pub has_pumpkin_hat: bool,
}
