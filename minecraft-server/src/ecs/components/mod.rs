pub use super::*;

pub mod health;
pub mod positions;

pub use health::HealthComponent;
pub use positions::PositionComponent;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Component {
    Health,
    Position,
}