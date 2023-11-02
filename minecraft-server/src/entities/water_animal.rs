use super::*;

#[derive(Default)]
pub struct WaterAnimal {
    pub pathfinder_mob: PathfinderMob,
}

pub trait WaterAnimalDescendant: PathfinderMobDescendant {
    fn get_pathfinder_mob(&self) -> &WaterAnimal;
    fn get_pathfinder_mob_mut(&mut self) -> &mut WaterAnimal;
}

impl EntityDescendant for WaterAnimal {
    fn get_entity(&self) -> &Entity { self.pathfinder_mob.get_entity() }
    fn get_entity_mut(&mut self) -> &mut Entity { self.pathfinder_mob.get_entity_mut() }
}

impl LivingEntityDescendant for WaterAnimal {
    fn get_living_entity(&self) -> &LivingEntity { self.pathfinder_mob.get_living_entity() }
    fn get_living_entity_mut(&mut self) -> &mut LivingEntity { self.pathfinder_mob.get_living_entity_mut() }
}

impl MobDescendant for WaterAnimal {
    fn get_mob(&self) -> &Mob { self.pathfinder_mob.get_mob() }
    fn get_mob_mut(&mut self) -> &mut Mob { self.pathfinder_mob.get_mob_mut() }
}

impl PathfinderMobDescendant for WaterAnimal {
    fn get_pathfinder_mob(&self) -> &PathfinderMob { &self.pathfinder_mob }
    fn get_pathfinder_mob_mut(&mut self) -> &mut PathfinderMob { &mut self.pathfinder_mob }
}

impl WaterAnimalDescendant for WaterAnimal {
    fn get_pathfinder_mob(&self) -> &WaterAnimal { self }
    fn get_pathfinder_mob_mut(&mut self) -> &mut WaterAnimal { self }
}
