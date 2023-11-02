use super::*;

#[derive(Default)]
pub struct ChestBoat {
    pub boat: Boat,
}

impl EntityDescendant for ChestBoat {
    fn get_entity(&self) -> &Entity { self.boat.get_entity() }
    fn get_entity_mut(&mut self) -> &mut Entity { self.boat.get_entity_mut() }
}

impl BoatDescendant for ChestBoat {
    fn get_boat(&self) -> &Boat { &self.boat }
    fn get_boat_mut(&mut self) -> &mut Boat { &mut self.boat }
}
