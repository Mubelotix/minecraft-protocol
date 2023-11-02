use super::*;
use minecraft_protocol::components::entity::Pose;

pub struct Entity {
    pub is_on_fire: bool,
    pub is_crouching: bool,
    pub is_sprinting: bool,
    pub is_swimming: bool,
    pub is_invisible: bool,
    pub is_glowing: bool,
    pub is_fying_with_elytra: bool,
    pub air_ticks: u32,
    pub name: Option<String>,
    pub is_name_visible: bool,
    pub is_silent: bool,
    pub has_no_gravity: bool,
    pub pose: Pose,
    pub ticks_frozen: u32,
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

pub trait EntityDescendant {
    fn get_entity(&self) -> &Entity;
    fn get_entity_mut(&mut self) -> &mut Entity;
}
