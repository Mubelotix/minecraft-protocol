use minecraft_protocol::components::{entity::Pose, slots::Slot};

use crate::prelude::*;

struct Entity {
    is_on_fire: bool,
    is_crouching: bool,
    is_sprinting: bool,
    is_swimming: bool,
    is_invisible: bool,
    is_glowing: bool,
    is_fying_with_elytra: bool,
    air_ticks: u32,
    name: Option<String>,
    is_name_visible: bool,
    is_silent: bool,
    has_no_gravity: bool,
    pose: Pose,
    ticks_frozen: u32,
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            is_on_fire: false,
            is_crouching: false,
            is_sprinting: false,
            is_swimming: false,
            is_invisible: false,
            is_glowing: false,
            is_fying_with_elytra: false,
            air_ticks: 300,
            name: None,
            is_name_visible: false,
            is_silent: false,
            has_no_gravity: false,
            pose: Pose::Standing,
            ticks_frozen: 0,
        }
    }
}

trait EntityDescendant {
    fn get_entity(&self) -> &Entity;
    fn get_entity_mut(&mut self) -> &mut Entity;
}

pub struct ThrownItemProjectile {
    entity: Entity,
    item: Slot,
}

impl Default for ThrownItemProjectile {
    fn default() -> Self {
        ThrownItemProjectile {
            entity: Entity::default(),
            item: Slot {item: None},
        }
    }
}

impl EntityDescendant for ThrownItemProjectile {
    fn get_entity(&self) -> &Entity {
        &self.entity
    }

    fn get_entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
}
