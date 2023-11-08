use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Sniffer {
    pub animal: Animal,
    pub sniffer_state: u8,
    pub drop_seed_at_tick: usize,
}
