use super::*;

#[derive(Default)]
pub struct AmbientCreature {
    pub mob: Mob,
}

pub trait AmbientCreatureDescendant: MobDescendant {
    fn get_ambient_creature(&self) -> &AmbientCreature;
    fn get_ambient_creature_mut(&mut self) -> &mut AmbientCreature;
}

impl EntityDescendant for AmbientCreature {
    fn get_entity(&self) -> &Entity { self.mob.get_entity() }
    fn get_entity_mut(&mut self) -> &mut Entity { self.mob.get_entity_mut() }
}

impl LivingEntityDescendant for AmbientCreature {
    fn get_living_entity(&self) -> &LivingEntity { self.mob.get_living_entity() }
    fn get_living_entity_mut(&mut self) -> &mut LivingEntity { self.mob.get_living_entity_mut() }
}

impl MobDescendant for AmbientCreature {
    fn get_mob(&self) -> &Mob { &self.mob }
    fn get_mob_mut(&mut self) -> &mut Mob { &mut self.mob }
}

impl AmbientCreatureDescendant for AmbientCreature {
    fn get_ambient_creature(&self) -> &AmbientCreature { self }
    fn get_ambient_creature_mut(&mut self) -> &mut AmbientCreature { self }
}
