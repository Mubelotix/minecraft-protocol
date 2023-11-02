use super::*;

#[derive(Default)]
#[inherit(Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Sniffer {
    pub sniffer_state: SnifferState,
    pub drop_seed_at_tick: usize,
}
