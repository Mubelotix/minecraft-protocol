use super::*;

#[derive(Default)]
pub struct Bat {
    pub ambient_creature: AmbientCreature,
}

pub trait AmbientCreatureDescendant: MobDescendant {
    fn get_ambient_creature(&self) -> &Bat;
    fn get_ambient_creature_mut(&mut self) -> &mut Bat;
}

impl EntityDescendant for Bat {
    fn get_entity(&self) -> &Entity { self.ambient_creature.get_entity() }
    fn get_entity_mut(&mut self) -> &mut Entity { self.ambient_creature.get_entity_mut() }
}

impl LivingEntityDescendant for Bat {
    fn get_living_entity(&self) -> &LivingEntity { self.ambient_creature.get_living_entity() }
    fn get_living_entity_mut(&mut self) -> &mut LivingEntity { self.ambient_creature.get_living_entity_mut() }
}

impl MobDescendant for Bat {
    fn get_mob(&self) -> &Mob { self.ambient_creature.get_mob() }
    fn get_mob_mut(&mut self) -> &mut Mob { self.ambient_creature.get_mob_mut() }
}

impl AmbientCreatureDescendant for Bat {
    fn get_ambient_creature(&self) -> &Bat { self }
    fn get_ambient_creature_mut(&mut self) -> &mut Bat { self }
}
