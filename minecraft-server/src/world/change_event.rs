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
        velocity: (),
        metadata: (),
    },
    EntityDispawned {
        eid: Eid,
    },
    EntityMetadata {
        eid: Eid,
        metadata: (),
    },
    // TODO packet without this
    EntityMoved {
        eid: Eid,
        position: Position,
        pitch: f32,
        yaw: f32,
        head_yaw: f32,
    },
}
