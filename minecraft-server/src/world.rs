
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
    pub async fn get_block(&self, position: BlockPosition) -> Option<BlockWithState> {
        Some(self.map.get_block(position).await)
    }

    pub async fn set_block(&self, position: BlockPosition, block: BlockWithState) {
        self.map.set_block(position, block).await;
    }
}
