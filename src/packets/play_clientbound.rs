use super::*;

#[derive(Debug, MinecraftPacket)]
pub struct SpawnEntity {
    pub id: VarInt,
    pub uuid: UUID,
    pub entity_type: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: Angle,
    pub yaw: Angle,
    pub data: i32,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

#[derive(Debug, MinecraftPacket)]
pub struct SpawnExperienceOrb {
    pub id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub count: i16,
}

#[derive(Debug, MinecraftPacket)]
pub struct SpawnLivingEntity {
    pub id: VarInt,
    pub uuid: UUID,
    pub entity_type: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: Angle,
    pub pitch: Angle,
    pub head_pitch: Angle,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

#[derive(Debug, MinecraftPacket)]
pub struct SpawnPainting {
    pub id: VarInt,
    pub uuid: UUID,
    pub motive: crate::paintings::Painting,
    pub location: Position,
    pub direction: Direction,
}

#[derive(Debug, MinecraftPacket)]
pub struct SpawnPlayer {
    pub id: VarInt,
    pub uuid: UUID,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: Angle,
    pub pitch: Angle,
}

#[derive(Debug, MinecraftPacket)]
pub struct EntityAnimation {
    pub id: VarInt,
    pub animation: crate::animations::Animation,
}

#[derive(Debug, MinecraftPacket)]
pub struct Statistics<'a> {
    pub count: VarInt,
    pub statistic: RawBytes<'a>,
}

#[derive(Debug, MinecraftPacket)]
pub struct AcknowledgePlayerDigging {
    pub location: Position,
    pub block: VarInt,
    pub status: crate::digging_states::PartialDiggingState,
    pub successful: bool,
}
