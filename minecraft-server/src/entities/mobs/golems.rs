use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { PathfinderMob, Mob, LivingEntity, Entity },
    descendants { IronGolem, SnowGolem, Shulker },
)]
pub struct AbstractGolem {
    pub pathfinder_mob: PathfinderMob,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractGolem, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct IronGolem {
    pub abstract_golem: AbstractGolem,
    pub is_player_created: bool,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractGolem, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct SnowGolem {
    pub abstract_golem: AbstractGolem,
    pub has_pumpkin_hat: bool,
}
