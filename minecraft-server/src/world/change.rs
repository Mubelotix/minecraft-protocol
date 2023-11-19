use futures::channel::mpsc::Sender;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum WorldChange {
    Block(BlockPosition, BlockWithState),
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
        metadata: (),
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
        position: Position,
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

    fn add(self, rhs: EntityChanges) -> EntityChanges {
        EntityChanges(self.0 | rhs.0)
    }
}

impl std::ops::AddAssign<EntityChanges> for EntityChanges {
    fn add_assign(&mut self, rhs: EntityChanges) {
        self.0 |= rhs.0;
    }
}

struct WorldObserver {
    sender: MpscSender<EntityChanges>,
    ticks: bool,
    blocks: HashSet<ChunkColumnPosition>,
    entities: HashSet<ChunkColumnPosition>,
    nearby_blocks: HashSet<ChunkColumnPosition>,
    specific_entities: HashSet<Eid>,
}

#[must_use = "The observer must be added to the manager to be used"]
pub struct WorldSubscriberBuilder {
    ticks: bool,
    blocks: Vec<ChunkColumnPosition>,
    entities: Vec<ChunkColumnPosition>,
    nearby_blocks: Vec<NearbyBlockSubscription>,
    specific_entities: Vec<Eid>,
}

impl WorldSubscriberBuilder {
    pub fn new() -> WorldSubscriberBuilder {
        WorldSubscriberBuilder {
            ticks: false,
            blocks: Vec::new(),
            nearby_blocks: Vec::new(),
            entities: Vec::new(),
            specific_entities: Vec::new(),
        }
    }

    pub fn with_ticks(mut self) -> WorldSubscriberBuilder {
        self.ticks = true;
        self
    }

    pub fn with_blocks_in_chunk(mut self, position: ChunkColumnPosition) -> WorldSubscriberBuilder {
        self.blocks.push(position);
        self
    }

    pub fn with_entities_in_chunk(mut self, position: ChunkColumnPosition) -> WorldSubscriberBuilder {
        self.entities.push(position);
        self
    }

    pub fn with_nearby_blocks(mut self, position: BlockPosition, radius: u8) -> WorldSubscriberBuilder {
        self.nearby_blocks.push(NearbyBlockSubscription {
            position,
            radius,
        });
        self
    }

    pub fn with_entity(mut self, eid: Eid) -> WorldSubscriberBuilder {
        self.specific_entities.push(eid);
        self
    }

    pub async fn finish(self, eid: Eid, observer_manager: &WorldObserverManager) -> MpscReceiver<EntityChanges> {
        let (sender, receiver) = mpsc_channel(30);
        observer_manager.add_subscriber(eid, self, sender).await;
        receiver
    }
}

#[derive(Debug, Clone)]
struct NearbyBlockSubscription {
    position: BlockPosition,
    radius: u8,
}

pub struct WorldObserverManager {
    observers: RwLock<HashMap<Eid, WorldObserver>>,
    ticks: RwLock<HashSet<Eid>>,
    blocks: RwLock<HashMap<ChunkColumnPosition, HashSet<Eid>>>,
    entities: RwLock<HashMap<ChunkColumnPosition, HashSet<Eid>>>,
    nearby_blocks: RwLock<HashMap<ChunkColumnPosition, HashMap<Eid, NearbyBlockSubscription>>>,
    specific_entities: RwLock<HashMap<Eid, HashSet<Eid>>>,
}

impl WorldObserverManager {
    async fn add_subscriber(&self, eid: Eid, observer_builder: WorldSubscriberBuilder, sender: MpscSender<EntityChanges>) {
        let mut entities = self.observers.write().await;
        if !observer_builder.blocks.is_empty() {
            let mut blocks = self.blocks.write().await;
            for column in &observer_builder.blocks {
                blocks.entry(column.clone()).or_default().insert(eid);
            }
        }
        if !observer_builder.entities.is_empty() {
            let mut entities = self.blocks.write().await;
            for column in &observer_builder.entities {
                entities.entry(column.clone()).or_default().insert(eid);
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
                        nearby_blocks.entry(ChunkColumnPosition {cx: cx, cz: cz}).or_default().insert(eid, nearby_block.clone());
                        observer_nearby_blocks.insert(ChunkColumnPosition {cx: cx, cz: cz});
                    }
                }
            }
        }
        if !observer_builder.specific_entities.is_empty() {
            let mut specific_entities = self.specific_entities.write().await;
            for entity in &observer_builder.specific_entities {
                specific_entities.entry(entity.clone()).or_default().insert(eid);
            }
        }
        entities.insert(eid, WorldObserver {
            sender,
            ticks: observer_builder.ticks,
            blocks: observer_builder.blocks.into_iter().collect(),
            entities: observer_builder.entities.into_iter().collect(),
            nearby_blocks: observer_nearby_blocks,
            specific_entities: observer_builder.specific_entities.into_iter().collect(),
        });
    }

    pub async fn remove_subscriber(&self, eid: Eid) {
        let mut entities = self.observers.write().await;
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
