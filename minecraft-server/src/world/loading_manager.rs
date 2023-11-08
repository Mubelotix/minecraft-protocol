use crate::prelude::*;

#[derive(Default)]
pub(super) struct WorldLoadingManager {
    loaded_chunks: HashMap<UUID, HashSet<ChunkColumnPosition>>,
    loader_entities: HashMap<ChunkColumnPosition, HashSet<UUID>>,
}

impl WorldLoadingManager {
    pub(super) fn update_loaded_chunks(&mut self, uuid: UUID, loaded_chunks: HashSet<ChunkColumnPosition>) {
        let loaded_before = self.loaded_chunks.entry(uuid).or_default();
        for just_unloaded in loaded_before.difference(&loaded_chunks) {
            let mut can_be_removed = false;
            self.loader_entities.entry(just_unloaded.clone()).and_modify(|f| {
                f.remove(&uuid);
                if f.is_empty() { can_be_removed = true;}
            });
            if can_be_removed {
                self.loader_entities.remove(just_unloaded);
            }
        }
        for newly_loaded in loaded_chunks.difference(loaded_before).cloned() {
            self.loader_entities.entry(newly_loaded).or_default().insert(uuid);
        }
        *loaded_before = loaded_chunks;
    }

    pub(super) fn get_loaders(&self, position: &ChunkColumnPosition) -> Option<&HashSet<UUID>> {
        self.loader_entities.get(position)
    }

    pub(super) fn get_loaded_chunks(&self) -> HashSet<ChunkColumnPosition> {
        self.loader_entities.keys().cloned().collect()
    }
}
