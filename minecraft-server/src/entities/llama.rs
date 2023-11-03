use super::*;

#[inheritable]
#[inherit(ChestedHorse, AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Llama {
    pub chested_horse: ChestedHorse,
    /// Strength (number of columns of 3 slots in the llama's inventory once a chest is equipped)	
    pub stength: u8,
    /// Carpet color (a dye color, or -1 if no carpet equipped)	
    pub carpet_color: i16,
    pub variant: u8,
}

impl Default for Llama {
    fn default() -> Self {
        Self {
            chested_horse: ChestedHorse::default(),
            stength: 0,
            carpet_color: -1,
            variant: 0,
        }
    }
}

#[derive(Default)]
#[inherit(Llama, ChestedHorse, AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct TraderLlama {
    pub llama: Llama,
}
