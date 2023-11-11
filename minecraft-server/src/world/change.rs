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
        velocity: Position,
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
        velocity: Position,
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

    pub fn has_position_changed(&self) -> bool {
        self.0 & 1 != 0
    }

    pub fn has_velocity_changed(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    pub fn has_pitch_changed(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    pub fn has_metadata_changed(&self) -> bool {
        self.0 & (1 << 3) != 0
    }
}

impl std::ops::Add<EntityChanges> for EntityChanges {
    type Output = EntityChanges;

    fn add(self, rhs: EntityChanges) -> EntityChanges {
        EntityChanges(self.0 | rhs.0)
    }
}
