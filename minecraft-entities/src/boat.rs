use super::*;

#[MinecraftEntity(
    inheritable, parents { Entity },
)]
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

impl TryAsEntityRef<Boat> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&Boat> {
        match self {
            AnyEntity::Boat(boat) => Some(boat),
            AnyEntity::ChestBoat(chest_boat) => Some(&chest_boat.boat),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut Boat> {
        match self {
            AnyEntity::Boat(boat) => Some(boat),
            AnyEntity::ChestBoat(chest_boat) => Some(&mut chest_boat.boat),
            _ => None,
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Boat, Entity },
)]
pub struct ChestBoat {
    pub boat: Boat,
}
