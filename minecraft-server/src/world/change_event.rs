use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum WorldChange {
    BlockChange(BlockPosition, BlockWithState),
}
