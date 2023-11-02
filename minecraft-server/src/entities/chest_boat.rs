use super::*;

#[derive(Default)]
#[inherit(Boat, Entity)]
pub struct ChestBoat {
    pub boat: Boat,
}
