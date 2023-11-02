use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(LivingEntity, Entity)]
pub struct Mob {
    pub entity: Entity,
    pub living_entity: LivingEntity,
    pub no_ai: bool,
    pub is_left_handed: bool,
    pub is_aggressive: bool,
}
