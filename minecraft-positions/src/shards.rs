use crate::*;

impl ChunkColumnPosition {
    pub fn shard(&self, shard_count: usize) -> usize {
        const REGION_SIZE: i32 = 8;
        let region_x = self.cx.div_euclid(REGION_SIZE);
        let region_z = self.cz.div_euclid(REGION_SIZE);
        (region_x + region_z).unsigned_abs() as usize % shard_count
    }
}
