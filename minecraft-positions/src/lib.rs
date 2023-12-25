mod shards;

use std::ops::AddAssign;

pub use minecraft_protocol::packets::Position as NetworkPosition;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
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

    pub fn chunk_column(&self) -> ChunkColumnPosition {
        ChunkColumnPosition {
            cx: (self.x.floor() as i32).div_euclid(16),
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

#[derive(Clone, Default)]
pub struct Rotation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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

    pub fn get_circle_from_center(&self, radius: i32) -> Vec<ChunkColumnPosition> {
        let mut chunks = Vec::new();
        let r2 = radius * radius;
        for x in 0..=radius {
            let x2 = x * x;
            for y in 0..x {
                if x2 + y * y > r2 {
                    break;
                }
                chunks.push(ChunkColumnPosition {
                    cx: self.cx + x,
                    cz: self.cz + y,
                });
                chunks.push(ChunkColumnPosition {
                    cx: self.cx + x,
                    cz: self.cz - y,
                });
                chunks.push(ChunkColumnPosition {
                    cx: self.cx - x,
                    cz: self.cz + y,
                });
                chunks.push(ChunkColumnPosition {
                    cx: self.cx - x,
                    cz: self.cz - y,
                });
                chunks.push(ChunkColumnPosition {
                    cx: self.cx + y,
                    cz: self.cz + x,
                });
                chunks.push(ChunkColumnPosition {
                    cx: self.cx + y,
                    cz: self.cz - x,
                });
                chunks.push(ChunkColumnPosition {
                    cx: self.cx - y,
                    cz: self.cz + x,
                });
                chunks.push(ChunkColumnPosition {
                    cx: self.cx - y,
                    cz: self.cz - x,
                });
            }
        }
        chunks
    }
}

#[derive(Debug, Clone)]
pub struct LightPositionInChunkColumn {
    pub bx: u8,
    pub y: usize,
    pub bz: u8,
}

impl LightPositionInChunkColumn {
    pub fn in_chunk(&self) -> BlockPositionInChunk {
        BlockPositionInChunk {
            bx: self.bx,
            by: self.y.rem_euclid(16) as u8,
            bz: self.bz,
        }
    }
}


impl From<BlockPositionInChunkColumn> for LightPositionInChunkColumn {
    fn from(val: BlockPositionInChunkColumn) -> Self {
        Self {
            bx: val.bx,
            y: (val.y + 64 + 16) as usize, // TODO: Use the world config
            bz: val.bz,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LightPosition {
    pub x: i32,
    pub y: usize,
    pub z: i32,
}

impl From<LightPosition> for LightPositionInChunkColumn {
    fn from(val: LightPosition) -> Self {
        LightPositionInChunkColumn {
            bx: val.x.rem_euclid(16) as u8,
            y: val.y,
            bz: val.z.rem_euclid(16) as u8,
        }
    }
}

impl From<LightPosition> for ChunkColumnPosition {
    fn from(val: LightPosition) -> Self {
        ChunkColumnPosition {
            cx: val.x.div_euclid(16),
            cz: val.z.div_euclid(16),
        }
    }
}

impl From<BlockPosition> for LightPosition {
    fn from(val: BlockPosition) -> Self {
        Self {
            x: val.x,
            y: (val.y + 64 + 16) as usize,
            z: val.z,
        }
    }
}

impl From<LightPosition> for BlockPosition {
    fn from(val: LightPosition) -> Self {
        Self {
            x: val.x,
            y: val.y as i32 - 64 - 16,
            z: val.z
        }
    }
}

impl LightPosition {
    pub fn in_chunk(&self) -> BlockPositionInChunk {
        BlockPositionInChunk {
            bx: self.x.rem_euclid(16) as u8,
            by: self.y.rem_euclid(16) as u8,
            bz: self.z.rem_euclid(16) as u8,
        }
    }

    pub fn get_neighbors(&self, n_chunk: usize) -> Vec<Self> {
        let mut neighbors = Vec::new();
        if self.y < ((n_chunk - 1) * 16) + 1 { // No block can be higher so no block can affect the light level 
            neighbors.push(LightPosition { x: self.x, y: self.y + 1, z: self.z });
        }
        neighbors.push(LightPosition { x: self.x - 1, y: self.y, z: self.z });
        neighbors.push(LightPosition { x: self.x + 1, y: self.y, z: self.z });
        neighbors.push(LightPosition { x: self.x, y: self.y, z: self.z - 1 });
        neighbors.push(LightPosition { x: self.x, y: self.y, z: self.z + 1 });
        if self.y > 0 {
            neighbors.push(LightPosition { x: self.x, y: self.y - 1, z: self.z });
        }
        neighbors
    }
}

impl PartialEq for LightPosition {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y
    }
}

impl From<LightPosition> for BlockPositionInChunkColumn {
    fn from(val: LightPosition) -> Self {
        BlockPositionInChunkColumn {
            bx: val.x.rem_euclid(16) as u8,
            y: val.y as i32 - 64 - 16, // TODO: Use the world config
            bz: val.x.rem_euclid(16) as u8,
        }
    }
}

impl AddAssign<usize> for LightPosition {
    fn add_assign(&mut self, rhs: usize) {
        self.y += rhs;
    }
}

impl std::cmp::Eq for LightPosition {}

impl std::cmp::PartialOrd for LightPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.y.cmp(&other.y))
    }
}

impl std::cmp::Ord for LightPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y)
    }
}

