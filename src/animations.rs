use minecraft_packet_derive::minecraft_enum;
use crate::packets::serializer::MinecraftPacketPart;

#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum Animation {
    SwingMainArm = 0,
    TakeDamage,
    LeaveBed,
    SwingOffhand,
    CriticalEffect,
    MagicCriticalEffect = 5,
}
