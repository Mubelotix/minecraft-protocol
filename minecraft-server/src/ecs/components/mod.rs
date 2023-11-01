pub use super::*;

pub mod health;
pub mod positions;

pub use health::HealthComponent;
pub use positions::PositionComponent;

use ecs_macros::generate_component_enum;

generate_component_enum!();
