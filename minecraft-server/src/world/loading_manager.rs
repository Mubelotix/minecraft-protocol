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
        if loaded_before.is_empty() {
            self.loaded_chunks.remove(&uuid);
        }
    }

    pub(super) fn get_loaders(&self, position: &ChunkColumnPosition) -> Option<&HashSet<UUID>> {
        self.loader_entities.get(position)
    }

    pub(super) fn get_loaded_chunks(&self) -> HashSet<ChunkColumnPosition> {
        self.loader_entities.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_loading_manager() {
        let mut manager = WorldLoadingManager::default();

        let mut loaded_first = vec![ChunkColumnPosition{cx: 0, cz: 0}, ChunkColumnPosition{cx: 1, cz: 0}, ChunkColumnPosition{cx: 2, cz: 0}];
        manager.update_loaded_chunks(0, loaded_first.clone().into_iter().collect());
        assert!(manager.get_loaded_chunks().len() == 3);
        assert!(manager.get_loaders(&ChunkColumnPosition{cx: 2, cz: 0}).unwrap().len() == 1);

        let loaded_second = vec![ChunkColumnPosition{cx: 0, cz: 1}, ChunkColumnPosition{cx: 1, cz: 1}, ChunkColumnPosition{cx: 2, cz: 1}];
        manager.update_loaded_chunks(1, loaded_second.clone().into_iter().collect());
        assert!(manager.get_loaded_chunks().len() == 6);

        loaded_first = vec![ChunkColumnPosition{cx: 0, cz: 0}, ChunkColumnPosition{cx: 1, cz: 1}];
        manager.update_loaded_chunks(0, loaded_first.clone().into_iter().collect());
        assert!(manager.get_loaded_chunks().len() == 4);
        assert!(manager.get_loaders(&ChunkColumnPosition{cx: 1, cz: 1}).unwrap().len() == 2);
        assert!(manager.get_loaders(&ChunkColumnPosition{cx: 2, cz: 0}).is_none());
    }
}
