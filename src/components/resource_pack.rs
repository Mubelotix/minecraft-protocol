use crate::*;

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum ResourcePackStatus {
    Loaded,
    Declined,
    FailedDownload,
    Accepted,
}
