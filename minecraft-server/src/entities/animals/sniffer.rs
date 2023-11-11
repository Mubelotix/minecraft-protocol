use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Sniffer {
    pub animal: Animal,
    pub sniffer_state: u8,
    pub drop_seed_at_tick: usize,
}
