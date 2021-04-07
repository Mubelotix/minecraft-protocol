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
    pub velocity_z: i16
}

