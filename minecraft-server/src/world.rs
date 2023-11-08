
use crate::prelude::*;

enum WorldChange {

}

/// World is the union of the map and entities.
/// World handles loaded chunks and entities.
/// It is responsible for notifying players of changes in the world.
struct World {
    map: WorldMap,
    entities: Entities,
}

impl World {
}
