use crate::prelude::*;

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

impl From<BlockPosition> for NetworkPosition {
    fn from(value: BlockPosition) -> Self {
        NetworkPosition {
            x: value.x,
            y: value.y as i16,
            z: value.z,
        }
    }
}

impl From<NetworkPosition> for BlockPosition {
    fn from(value: NetworkPosition) -> Self {
        BlockPosition {
            x: value.x,
            y: value.y as i32,
            z: value.z,
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
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn chunk(&self) -> ChunkPosition {
        ChunkPosition {
            cx: (self.x.floor() as i32).div_euclid(16),
            cy: (self.y.floor() as i32).div_euclid(16),
            cz: (self.z.floor() as i32).div_euclid(16),
        }
    }
}

impl std::ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
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

impl ChunkColumnPosition {
    pub fn chunk(&self, cy: i32) -> ChunkPosition {
        ChunkPosition {
            cx: self.cx,
            cy,
            cz: self.cz,
        }
    }
}
