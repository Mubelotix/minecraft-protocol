use super::*;

#[derive(Default)]
pub struct PathfinderMob {
    pub mob: Mob,
}

pub trait PathfinderMobDescendant: MobDescendant {
    fn get_pathfinder_mob(&self) -> &PathfinderMob;
    fn get_pathfinder_mob_mut(&mut self) -> &mut PathfinderMob;
}

impl EntityDescendant for PathfinderMob {
    fn get_entity(&self) -> &Entity { self.mob.get_entity() }
    fn get_entity_mut(&mut self) -> &mut Entity { self.mob.get_entity_mut() }
}

impl LivingEntityDescendant for PathfinderMob {
    fn get_living_entity(&self) -> &LivingEntity { self.mob.get_living_entity() }
    fn get_living_entity_mut(&mut self) -> &mut LivingEntity { self.mob.get_living_entity_mut() }
}

impl MobDescendant for PathfinderMob {
    fn get_mob(&self) -> &Mob { self.mob.get_mob() }
    fn get_mob_mut(&mut self) -> &mut Mob { self.mob.get_mob_mut() }
}

impl PathfinderMobDescendant for PathfinderMob {
    fn get_pathfinder_mob(&self) -> &PathfinderMob { self }
    fn get_pathfinder_mob_mut(&mut self) -> &mut PathfinderMob { self }
}
