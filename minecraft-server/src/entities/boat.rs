use super::*;

pub struct Boat {
    pub entity: Entity,
    pub time_since_last_hit: usize,
    pub forward_direction: usize,
    pub damage_taken: f32,
    /// Type (0=oak, 1=spruce, 2=birch, 3=jungle, 4=acacia, 5=dark oak)	
    pub ty: usize,
    pub is_left_paddle_turning: bool,
    pub is_right_paddle_turning: bool,
    pub splash_timer: usize,
}

impl Default for Boat {
    fn default() -> Self {
        Boat {
            entity: Entity::default(),
            time_since_last_hit: 0,
            forward_direction: 1,
            damage_taken: 0.0,
            ty: 0,
            is_left_paddle_turning: false,
            is_right_paddle_turning: false,
            splash_timer: 0,
        }
    }
}

pub trait BoatDescendant: EntityDescendant {
    fn get_boat(&self) -> &Boat;
    fn get_boat_mut(&mut self) -> &mut Boat;
}

impl EntityDescendant for Boat {
    fn get_entity(&self) -> &Entity { &self.entity }
    fn get_entity_mut(&mut self) -> &mut Entity { &mut self.entity }
}

impl BoatDescendant for Boat {
    fn get_boat(&self) -> &Boat { self }
    fn get_boat_mut(&mut self) -> &mut Boat { self }
}
