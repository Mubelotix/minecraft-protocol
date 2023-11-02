pub use super::*;

pub mod health;
pub mod positions;

pub use health::HealthComponent;
pub use positions::PositionComponent;

use minecraft_ecs_macros::generate_component_enum;

pub trait ComponentTrait: Sized + Clone {
    fn get_component_enum(&self) -> Component;
}


generate_component_enum!();
