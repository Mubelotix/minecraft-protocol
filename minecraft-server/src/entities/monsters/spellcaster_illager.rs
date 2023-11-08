use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(AbstractIllager, Raider, Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct SpellcasterIllager {
    pub abstract_illager:  AbstractIllager,
    pub spell: u8,
}
