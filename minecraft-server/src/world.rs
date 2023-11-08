
use crate::prelude::*;

enum WorldChange {

}

struct WorldLoadingManager {
    loaded_chunks: HashMap<UUID, HashSet<ChunkPosition>>,
    loader_players: HashMap<ChunkPosition, HashSet<UUID>>,
}

impl WorldLoadingManager {
    fn update_loaded_chunks(&mut self, uuid: UUID, loaded_chunks: HashSet<ChunkPosition>) {
        self.loaded_chunks.entry(uuid).and_modify(|f| {
            for just_unloaded in f.difference(&loaded_chunks) {
                let mut can_be_removed = false;
                self.loader_players.entry(just_unloaded.clone()).and_modify(|f| {
                    f.remove(&uuid);
                    if f.is_empty() { can_be_removed = true;}
                });
                if can_be_removed {
                    self.loader_players.remove(just_unloaded);
                }
            }
            for newly_loaded in loaded_chunks.difference(f).cloned() {
                self.loader_players.entry(newly_loaded).or_default().insert(uuid);
            }
        }).or_insert(loaded_chunks);
    }
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
