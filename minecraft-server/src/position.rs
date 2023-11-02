#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPosition {
    pub fn chunk(&self) -> ChunkPosition {
        ChunkPosition {
            cx: self.x.div_euclid(16),
            cy: self.y.div_euclid(16),
            cz: self.z.div_euclid(16),
        }
    }

    pub fn in_chunk(&self) -> BlockPositionInChunk {
        BlockPositionInChunk {
            bx: self.x.rem_euclid(16) as u8,
            by: self.y.rem_euclid(16) as u8,
            bz: self.z.rem_euclid(16) as u8,
        }
    }

    pub fn in_chunk_column(&self) -> BlockPositionInChunkColumn {
        BlockPositionInChunkColumn {
            bx: self.x.rem_euclid(16) as u8,
            y: self.y,
            bz: self.z.rem_euclid(16) as u8,
        }
    }

    pub fn chunk_column(&self) -> ChunkColumnPosition {
        ChunkColumnPosition {
            cx: self.x.div_euclid(16),
            cz: self.z.div_euclid(16),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockPositionInChunk {
    pub bx: u8,
    pub by: u8,
    pub bz: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockPositionInChunkColumn {
    pub bx: u8,
    pub y: i32,
    pub bz: u8,
}

impl BlockPositionInChunkColumn {
    pub fn in_chunk(&self) -> BlockPositionInChunk {
        BlockPositionInChunk {
            bx: self.bx,
            by: self.y.rem_euclid(16) as u8,
            bz: self.bz,
        }
    }

    pub fn cy(&self) -> i32 {
        self.y.div_euclid(16)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChunkPosition {
    pub cx: i32,
    pub cy: i32,
    pub cz: i32,
}

impl ChunkPosition {
    pub fn chunk_column(&self) -> ChunkColumnPosition {
        ChunkColumnPosition {
            cx: self.cx,
            cz: self.cz,
        }
    }
}

impl std::ops::Add<BlockPositionInChunk> for ChunkPosition {
    type Output = BlockPosition;

    fn add(self, rhs: BlockPositionInChunk) -> Self::Output {
        BlockPosition {
            x: self.cx * 16 + rhs.bx as i32,
            y: self.cy * 16 + rhs.by as i32,
            z: self.cz * 16 + rhs.bz as i32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChunkColumnPosition {
    pub cx: i32,
    pub cz: i32,
}
