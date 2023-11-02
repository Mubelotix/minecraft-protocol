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
            bx: self.x.rem_euclid(16),
            by: self.y.rem_euclid(16),
            bz: self.z.rem_euclid(16),
        }
    }

    pub fn chunk_column(&self) -> ChunkColumnPosition {
        ChunkColumnPosition {
            cx: self.x.div_euclid(16),
            cz: self.z.div_euclid(16),
        }
    }
}

pub struct BlockPositionInChunk {
    pub bx: i32,
    pub by: i32,
    pub bz: i32,
}

#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn chunk(&self) -> ChunkPosition {
        ChunkPosition {
            cx: self.x.floor() as i32 / 16,
            cy: self.y.floor() as i32 / 16,
            cz: self.z.floor() as i32 / 16,
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

#[derive(PartialEq, Eq, Hash)]
pub struct ChunkPosition {
    pub cx: i32,
    pub cy: i32,
    pub cz: i32,
}

impl ChunkPosition {
    fn chunk_column(&self) -> ChunkColumnPosition {
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
            x: self.cx * 16 + rhs.bx,
            y: self.cy * 16 + rhs.by,
            z: self.cz * 16 + rhs.bz,
        }
    }
}

pub struct ChunkColumnPosition {
    pub cx: i32,
    pub cz: i32,
}
