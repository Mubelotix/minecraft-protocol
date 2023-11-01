pub mod health;
pub mod positions;

pub use health::HealthComponent;
pub use positions::PositionComponent;

#[derive(Clone)]
pub enum Components {
    Health,
    Position,
}