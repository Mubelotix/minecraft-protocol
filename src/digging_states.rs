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
