use crate::prelude::*;

/// Describes an event for something happening the world.
#[derive(Debug, Clone)]
pub enum WorldChange {
    Tick(usize),
    Block(BlockPosition, BlockWithState),
    ColumnLoaded(ChunkColumnPosition),
    ColumnUnloaded(ChunkColumnPosition),
    EntitySpawned {
        eid: Eid,
        uuid: UUID,
        ty: NetworkEntity,
        position: Position,
        pitch: f32,
        yaw: f32,
        head_yaw: f32,
        data: u32,
        velocity: Translation,
    },
    EntityDispawned {
        eid: Eid,
    },
    EntityMetadata {
        eid: Eid,
        metadata: (),
    },
    EntityPosition {
        eid: Eid,
        from: Position,
        to: Position,
    },
    EntityVelocity {
        eid: Eid,
        velocity: Translation,
    },
    EntityPitch {
        eid: Eid,
        pitch: f32,
        yaw: f32,
        head_yaw: f32,
    },
}

/// Describes what has been modified in an entity by a mutator closure.
pub struct EntityChanges(u8);

impl EntityChanges {
    pub const fn other() -> EntityChanges {
        EntityChanges(0)
    }

    pub const fn nothing() -> EntityChanges {
        EntityChanges(0)
    }

    pub const fn position() -> EntityChanges {
        EntityChanges(1)
    }

    pub const fn velocity() -> EntityChanges {
        EntityChanges(1 << 1)
    }

    pub const fn pitch() -> EntityChanges {
        EntityChanges(1 << 2)
    }

    pub const fn metadata() -> EntityChanges {
        EntityChanges(1 << 3)
    }

    pub const fn nothing_changed(&self) -> bool {
        self.0 == 0
    }

    pub const fn position_changed(&self) -> bool {
        self.0 & 1 != 0
    }

    pub const fn velocity_changed(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    pub const fn pitch_changed(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    pub const fn metadata_changed(&self) -> bool {
        self.0 & (1 << 3) != 0
    }
}

impl std::ops::Add<EntityChanges> for EntityChanges {
    type Output = EntityChanges;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: EntityChanges) -> EntityChanges {
        EntityChanges(self.0 | rhs.0)
    }
}

impl std::ops::AddAssign<EntityChanges> for EntityChanges {
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add_assign(&mut self, rhs: EntityChanges) {
        self.0 |= rhs.0;
    }
}

/// An observer for changes in the world.
/// It listens for changes.
/// It will not listen for all changes, but only the kind of changes it subscribed to.
pub struct WorldObserver {
    receiver: MpscReceiver<WorldChange>,
    eid: Eid,
    observer_manager: &'static WorldObserverManager,
}

impl WorldObserver {
    pub async fn recv(&mut self) -> Option<WorldChange> {
        self.receiver.recv().await
    }

    pub fn try_recv(&mut self) -> Result<WorldChange, tokio::sync::mpsc::error::TryRecvError> {
        self.receiver.try_recv()
    }

    /// Enables receiving ticks.
    pub async fn enable_ticks(&self) {
        let mut trackers = self.observer_manager.trackers.write().await;
        let Some(tracker) = trackers.get_mut(&self.eid) else { return };
        if !tracker.ticks {
            tracker.ticks = true;
            self.observer_manager.ticks.write().await.insert(self.eid);
        }
    }

    /// Disables receiving ticks.
    pub async fn disable_ticks(&self) {
        let mut trackers = self.observer_manager.trackers.write().await;
        let Some(tracker) = trackers.get_mut(&self.eid) else { return };
        if tracker.ticks {
            tracker.ticks = false;
            self.observer_manager.ticks.write().await.remove(&self.eid);
        }
    }
}

/// A builder for [WorldObserver].
#[must_use = "The observer builder must be built to be used"]
pub struct WorldObserverBuilder {
    eid: Eid,
    observer_manager: &'static WorldObserverManager,
    ticks: bool,
    blocks: Vec<ChunkColumnPosition>,
    entities: Vec<ChunkColumnPosition>,
    nearby_blocks: Vec<NearbyBlockSubscription>,
    specific_entities: Vec<Eid>,
}

impl WorldObserverBuilder {
    pub fn new(eid: Eid, observer_manager: &'static WorldObserverManager) -> WorldObserverBuilder {
        WorldObserverBuilder {
            eid,
            observer_manager,
            ticks: false,
            blocks: Vec::new(),
            nearby_blocks: Vec::new(),
            entities: Vec::new(),
            specific_entities: Vec::new(),
        }
    }

    pub fn with_ticks(mut self) -> WorldObserverBuilder {
        self.ticks = true;
        self
    }

    pub fn with_blocks_in_chunk(mut self, position: ChunkColumnPosition) -> WorldObserverBuilder {
        self.blocks.push(position);
        self
    }

    pub fn with_entities_in_chunk(mut self, position: ChunkColumnPosition) -> WorldObserverBuilder {
        self.entities.push(position);
        self
    }

    pub fn with_nearby_blocks(mut self, position: BlockPosition, radius: u8) -> WorldObserverBuilder {
        self.nearby_blocks.push(NearbyBlockSubscription {
            position,
            radius,
        });
        self
    }

    pub fn with_entity(mut self, eid: Eid) -> WorldObserverBuilder {
        self.specific_entities.push(eid);
        self
    }

    pub async fn build(self) -> WorldObserver {
        let (sender, receiver) = mpsc_channel(200); // TODO: Review this value
        let eid = self.eid;
        let observer_manager = self.observer_manager;
        observer_manager.add_subscriber(eid, self, sender).await;
        WorldObserver {
            receiver,
            eid,
            observer_manager,
        }
    }
}

/// Used by [WorldObserverManager] to keep track of what an observer is subscribed to.
/// Will be used to clear subscriptions when the observer is removed.
struct WorldObserverTracker {
    sender: MpscSender<WorldChange>,
    ticks: bool,
    blocks: HashSet<ChunkColumnPosition>,
    entities: HashSet<ChunkColumnPosition>,
    nearby_blocks: HashSet<ChunkColumnPosition>,
    specific_entities: HashSet<Eid>,
}

#[derive(Debug, Clone)]
struct NearbyBlockSubscription {
    position: BlockPosition,
    radius: u8,
}

// TODO: remove sets and maps when they are empty

// TODO: allow different observers for same entity
pub struct WorldObserverManager {
    trackers: RwLock<HashMap<Eid, WorldObserverTracker>>,
    ticks: RwLock<HashSet<Eid>>,
    blocks: RwLock<HashMap<ChunkColumnPosition, HashMap<Eid, MpscSender<WorldChange>>>>,
    entities: RwLock<HashMap<ChunkColumnPosition, HashMap<Eid, MpscSender<WorldChange>>>>,
    nearby_blocks: RwLock<HashMap<ChunkColumnPosition, HashMap<Eid, (NearbyBlockSubscription, MpscSender<WorldChange>)>>>,
    specific_entities: RwLock<HashMap<Eid, HashMap<Eid, MpscSender<WorldChange>>>>,
}

impl WorldObserverManager {
    pub fn new() -> WorldObserverManager {
        WorldObserverManager {
            trackers: RwLock::new(HashMap::new()),
            ticks: RwLock::new(HashSet::new()),
            blocks: RwLock::new(HashMap::new()),
            entities: RwLock::new(HashMap::new()),
            nearby_blocks: RwLock::new(HashMap::new()),
            specific_entities: RwLock::new(HashMap::new()),
        }
    }

    async fn add_subscriber(&self, eid: Eid, observer_builder: WorldObserverBuilder, sender: MpscSender<WorldChange>) {
        let mut entities = self.trackers.write().await;
        if !observer_builder.blocks.is_empty() {
            let mut blocks = self.blocks.write().await;
            for column in &observer_builder.blocks {
                blocks.entry(column.clone()).or_default().insert(eid, sender.clone());
            }
        }
        if !observer_builder.entities.is_empty() {
            let mut entities = self.blocks.write().await;
            for column in &observer_builder.entities {
                entities.entry(column.clone()).or_default().insert(eid, sender.clone());
            }
        }
        let mut observer_nearby_blocks = HashSet::new();
        if !observer_builder.nearby_blocks.is_empty() {
            let mut nearby_blocks = self.nearby_blocks.write().await;
            for nearby_block in &observer_builder.nearby_blocks {
                let min_column = BlockPosition {
                    x: nearby_block.position.x.saturating_sub(nearby_block.radius as i32),
                    z: nearby_block.position.z.saturating_sub(nearby_block.radius as i32),
                    y: 0,
                }.chunk_column();
                let max_column = BlockPosition {
                    x: nearby_block.position.x.saturating_add(nearby_block.radius as i32),
                    z: nearby_block.position.z.saturating_add(nearby_block.radius as i32),
                    y: 0,
                }.chunk_column();
                for cx in min_column.cx..=max_column.cx {
                    for cz in min_column.cz..=max_column.cz {
                        nearby_blocks.entry(ChunkColumnPosition {cx, cz}).or_default().insert(eid, (nearby_block.clone(), sender.clone()));
                        observer_nearby_blocks.insert(ChunkColumnPosition {cx, cz});
                    }
                }
            }
        }
        if !observer_builder.specific_entities.is_empty() {
            let mut specific_entities = self.specific_entities.write().await;
            for entity in &observer_builder.specific_entities {
                specific_entities.entry(*entity).or_default().insert(eid, sender.clone());
            }
        }
        entities.insert(eid, WorldObserverTracker {
            sender,
            ticks: observer_builder.ticks,
            blocks: observer_builder.blocks.into_iter().collect(),
            entities: observer_builder.entities.into_iter().collect(),
            nearby_blocks: observer_nearby_blocks,
            specific_entities: observer_builder.specific_entities.into_iter().collect(),
        });
    }

    pub async fn notify_block_change(&self, position: BlockPosition, block: BlockWithState) {
        let column = position.chunk_column();
        let blocks = self.blocks.read().await;
        if let Some(subscribers) = blocks.get(&column) {
            let event = WorldChange::Block(position.clone(), block.clone());
            for sender in subscribers.values() {
                let _ = sender.try_send(event.clone());
            }
        }
    }

    pub async fn notify_column_loaded(&self, column_pos: ChunkColumnPosition) {
        // TODO: Notify nearby block listeners of loaded/unloaded chunks
        let blocks = self.blocks.read().await;
        if let Some(subscribers) = blocks.get(&column_pos) {
            let event = WorldChange::ColumnLoaded(column_pos.clone());
            for sender in subscribers.values() {
                if let Err(e) = sender.try_send(event.clone()) {
                    warn!("Failed to notify loaded column: {}", e)
                }
            }
        }
    }

    pub async fn notify_column_unloaded(&self, column_pos: ChunkColumnPosition) {
        let blocks = self.blocks.read().await;
        if let Some(subscribers) = blocks.get(&column_pos) {
            let event = WorldChange::ColumnUnloaded(column_pos.clone());
            for sender in subscribers.values() {
                let _ = sender.try_send(event.clone());
            }
        }
    }

    pub async fn notify_tick(&self, tick_id: usize) {
        let ticks = self.ticks.read().await;
        for eid in ticks.iter() {
            let trackers = self.trackers.read().await;
            if let Some(tracker) = trackers.get(eid) {
                let _ = tracker.sender.try_send(WorldChange::Tick(tick_id));
            }
        }
    }

    #[inline]
    async fn notify_entity_change(&self, eid: Eid, position: Position, from: Option<Position>, change: WorldChange) {
        let specific_entities = self.specific_entities.read().await;

        // Notify for specific entities
        let specific_entities_subscribers = specific_entities.get(&eid);
        if let Some(subscribers) = specific_entities_subscribers {
            for sender in subscribers.values() {
                let _ = sender.try_send(change.clone());
            }
        }

        // Notify for current column those who were not already notified
        let entities = self.entities.read().await;
        let current_column = entities.get(&position.chunk_column());
        if let Some(subscribers) = current_column {
            for (subscriber, sender) in subscribers {
                if specific_entities_subscribers.map(|c| c.contains_key(subscriber)).unwrap_or(true) {
                    let _ = sender.try_send(change.clone());
                }
            }
        }

        // Notify for previous column those who were not already notified
        if let Some(from) = from {
            if from.chunk_column() != position.chunk_column() {
                let previous_column = entities.get(&from.chunk_column());
                if let Some(subscribers) = previous_column {
                    for (subscriber, sender) in subscribers {
                        if current_column.map(|c| c.contains_key(subscriber)).unwrap_or(true) &&
                            specific_entities_subscribers.map(|c| c.contains_key(subscriber)).unwrap_or(true)
                        {
                            let _ = sender.try_send(change.clone());
                        }
                    }
                }
            }
        }
    }

    pub async fn notify_entity_spawned(&self, eid: Eid, uuid: UUID, ty: NetworkEntity, position: Position, pitch: f32, yaw: f32, head_yaw: f32, data: u32, velocity: Translation) {
        let change = WorldChange::EntitySpawned {
            eid,
            uuid,
            ty,
            position: position.clone(),
            pitch,
            yaw,
            head_yaw,
            data,
            velocity,
        };
        self.notify_entity_change(eid, position, None, change).await;
    }

    pub async fn notify_entity_dispawned(&self, eid: Eid, position: Position) {
        let change = WorldChange::EntityDispawned {
            eid,
        };
        self.notify_entity_change(eid, position, None, change).await;
        self.specific_entities.write().await.remove(&eid);
    }

    pub async fn notify_metadata_change(&self, eid: Eid, position: Position, metadata: ()) {
        let change = WorldChange::EntityMetadata {
            eid,
            metadata,
        };
        self.notify_entity_change(eid, position, None, change).await;
    }

    pub async fn notify_entity_moved(&self, eid: Eid, from: Position, to: Position) {
        let change = WorldChange::EntityPosition {
            eid,
            from: from.clone(),
            to: to.clone(),
        };
        self.notify_entity_change(eid, to, Some(from), change).await;
    }

    pub async fn notify_entity_velocity(&self, eid: Eid, position: Position, velocity: Translation) {
        let change = WorldChange::EntityVelocity {
            eid,
            velocity,
        };
        self.notify_entity_change(eid, position, None, change).await;
    }

    pub async fn notify_entity_pitch(&self, eid: Eid, position: Position, pitch: f32, yaw: f32, head_yaw: f32) {
        let change = WorldChange::EntityPitch {
            eid,
            pitch,
            yaw,
            head_yaw,
        };
        self.notify_entity_change(eid, position, None, change).await;
    }

    pub async fn update_loaded_columns(&self, eid: Eid, loaded_chunks: HashSet<ChunkColumnPosition>) {
        let mut entities = self.trackers.write().await;
        let Some(observer) = entities.get_mut(&eid) else {return};
        let unloaded_chunks = observer.blocks.difference(&loaded_chunks);
        let newly_loaded_chunks = loaded_chunks.difference(&observer.blocks);

        let mut blocks = self.blocks.write().await;
        let mut entities = self.entities.write().await;
        for column in unloaded_chunks {
            blocks.get_mut(column).map(|map| map.remove(&eid));
            entities.get_mut(column).map(|map| map.remove(&eid));
        }
        for column in newly_loaded_chunks {
            blocks.entry(column.clone()).or_default().insert(eid, observer.sender.clone());
            entities.entry(column.clone()).or_default().insert(eid, observer.sender.clone());
        }
        observer.blocks = loaded_chunks.clone();
    }

    pub async fn get_all_needed_columns(&self) -> HashSet<ChunkColumnPosition> {
        let blocks = self.blocks.read().await;
        let mut all_needed_chunks = HashSet::new();
        all_needed_chunks.extend(blocks.keys().cloned());
        all_needed_chunks
    }

    pub async fn remove_subscriber(&self, eid: Eid) {
        let mut entities = self.trackers.write().await;
        let Some(observer) = entities.remove(&eid) else {return};
        if observer.ticks {
            self.ticks.write().await.remove(&eid);
        }
        if !observer.blocks.is_empty() {
            let mut block_subscriptions = self.blocks.write().await;
            for column in observer.blocks {
                block_subscriptions.get_mut(&column).map(|set| set.remove(&eid));
            }
        }
        if !observer.nearby_blocks.is_empty() {
            let mut precise_block_subscriptions = self.nearby_blocks.write().await;
            for column in observer.nearby_blocks {
                precise_block_subscriptions.get_mut(&column).map(|map| map.remove(&eid));
            }
        }
        if !observer.entities.is_empty() {
            let mut entity_subscriptions = self.entities.write().await;
            for column in observer.entities {
                entity_subscriptions.get_mut(&column).map(|set| set.remove(&eid));
            }
        }
        if !observer.specific_entities.is_empty() {
            let mut specific_entity_subscriptions = self.specific_entities.write().await;
            for entity in observer.specific_entities {
                specific_entity_subscriptions.get_mut(&entity).map(|set| set.remove(&eid));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ticks() {

    }
}
