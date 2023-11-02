use super::*;

#[derive(Default)]
pub struct ChestBoat {
    pub entity: Entity,
    pub boat: Boat,
}

impl EntityDescendant for ChestBoat {
    fn get_entity(&self) -> &Entity { &self.entity }
    fn get_entity_mut(&mut self) -> &mut Entity { &mut self.entity }
}

impl BoatDescendant for ChestBoat {
    fn get_boat(&self) -> &Boat { &self.boat }
    fn get_boat_mut(&mut self) -> &mut Boat { &mut self.boat }
}
