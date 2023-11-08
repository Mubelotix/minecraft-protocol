use crate::*;

impl ChunkColumnPosition {
    pub fn shard(&self, shard_count: usize) -> usize {
        (self.cx + self.cz).unsigned_abs() as usize % shard_count
    }
}
