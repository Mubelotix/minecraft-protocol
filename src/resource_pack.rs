use crate::*;

#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum ResourcePackStatus {
    Loaded,
    Declined,
    FailedDownload,
    Accepted,
}
