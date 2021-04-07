use crate::*;

#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum PartialDiggingState {
    Started,
    Cancelled,
    Finished
}

#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum DiggingState {
    Started,
    Cancelled,
    Finished,
    DropItemStack,
    DropItem,
    ShootArrowOrFinishEating,
    SwapItemInHand,
}

/// The type of update to perform used in [crate::packets::play_clientbound::BlockEntityData] packets.
#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum BlockEntityDataAction {
    /// Set data of a mob spawner (everything except for SpawnPotentials: current delay, min/max delay, mob to be spawned, spawn count, spawn range, etc.)
    MobSpawner = 1,
    /// Set command block text (command and last execution status)
    CommandBlock,
    /// Set the level, primary, and secondary powers of a beacon
    Beacon,
    /// Set rotation and skin of mob head
    MobHead,
    /// Declare a conduit
    Conduit,
    /// Set base color and patterns on a banner
    Banner,
    /// Set the data for a Structure tile entity
    Structure,
    /// Set the destination for a end gateway
    EndGateway,
    /// Set the text on a sign
    Sign,

    /// Declare a bed
    Bed = 11,
    /// Set data of a jigsaw block
    Jigsaw,
    /// Set items in a campfire
    Campfire,
    /// Beehive information
    Beehive,
}