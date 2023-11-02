use super::*;

#[derive(Default)]
pub struct Squid {
    pub water_animal: WaterAnimal,
}

impl EntityDescendant for Squid {
    fn get_entity(&self) -> &Entity { self.water_animal.get_entity() }
    fn get_entity_mut(&mut self) -> &mut Entity { self.water_animal.get_entity_mut() }
}

impl LivingEntityDescendant for Squid {
    fn get_living_entity(&self) -> &LivingEntity { self.water_animal.get_living_entity() }
    fn get_living_entity_mut(&mut self) -> &mut LivingEntity { self.water_animal.get_living_entity_mut() }
}

impl MobDescendant for Squid {
    fn get_mob(&self) -> &Mob { self.water_animal.get_mob() }
    fn get_mob_mut(&mut self) -> &mut Mob { self.water_animal.get_mob_mut() }
}

impl PathfinderMobDescendant for Squid {
    fn get_pathfinder_mob(&self) -> &PathfinderMob { self.water_animal.get_pathfinder_mob() }
    fn get_pathfinder_mob_mut(&mut self) -> &mut PathfinderMob { self.water_animal.get_pathfinder_mob_mut() }
}

impl WaterAnimalDescendant for Squid {
    fn get_water_animal(&self) -> &WaterAnimal { &self.water_animal }
    fn get_water_animal_mut(&mut self) -> &mut WaterAnimal { &mut self.water_animal }
}
