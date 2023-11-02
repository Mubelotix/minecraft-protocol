use super::*;

#[derive(Default)]
pub struct Mob {
    pub entity: Entity,
    pub living_entity: LivingEntity,
    pub no_ai: bool,
    pub is_left_handed: bool,
    pub is_aggressive: bool,
}

pub trait MobDescendant: EntityDescendant {
    fn get_mob(&self) -> &Mob;
    fn get_mob_mut(&mut self) -> &mut Mob;
}

impl EntityDescendant for Mob {
    fn get_entity(&self) -> &Entity { &self.entity }
    fn get_entity_mut(&mut self) -> &mut Entity { &mut self.entity }
}

impl LivingEntityDescendant for Mob {
    fn get_living_entity(&self) -> &LivingEntity { self.living_entity.get_living_entity() }
    fn get_living_entity_mut(&mut self) -> &mut LivingEntity { self.living_entity.get_living_entity_mut() }
}

impl MobDescendant for Mob {
    fn get_mob(&self) -> &Mob { self }
    fn get_mob_mut(&mut self) -> &mut Mob { self }
}
