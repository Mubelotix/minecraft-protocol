use minecraft_packet_derive::minecraft_enum;
use crate::packets::serializer::MinecraftPacketPart;
use crate::packets::VarInt;

#[minecraft_enum(VarInt)]
#[derive(Debug, Clone, Copy)]
pub enum Painting {
    Kebab = 0,
    Aztec,
    Alban,
    Aztec2,
    Bomb,
    Plant,
    Wasteland,
    Pool,
    Courbet,
    Sea,
    Sunset,
    Creebet,
    Wanderer,
    Graham,
    Match,
    Bust,
    Stage,
    Void,
    SkullAndRoses,
    Wither,
    Fighters,
    Pointer,
    Pigscene,
    BurningSkull,
    Skeleton,
    DonkeyKong,
}

impl Painting {
    pub fn get_id(&self) -> i32 {
        *self as i32
    }

    /// Get the rect position on `paintings_kristoffer_zetterstrand.png`.
    /// Return (x, y, width, height).
    /// See [the wiki](https://wiki.vg/Protocol#Spawn_Painting).
    pub fn get_position(&self) -> (u8, u8, u8, u8) {
        match self {
            Painting::Kebab => (0, 0, 16, 16),
            Painting::Aztec => (16, 0, 16, 16),
            Painting::Alban => (32, 0, 16, 16),
            Painting::Aztec2 => (48, 0, 16, 16),
            Painting::Bomb => (64, 0, 16, 16),
            Painting::Plant => (80, 0, 16, 16),
            Painting::Wasteland => (96, 0, 16, 16),
            Painting::Pool => (0, 32, 32, 16),
            Painting::Courbet => (32, 32, 32, 16),
            Painting::Sea => (64, 32, 32, 16),
            Painting::Sunset => (96, 32, 32, 16),
            Painting::Creebet => (128, 32, 32, 16),
            Painting::Wanderer => (0, 64, 16, 32),
            Painting::Graham => (16, 64, 16, 32),
            Painting::Match => (0, 128, 32, 32),
            Painting::Bust => (32, 128, 32, 32),
            Painting::Stage => (64, 128, 32, 32),
            Painting::Void => (96, 128, 32, 32),
            Painting::SkullAndRoses => (128, 128, 32, 32),
            Painting::Wither => (160, 128, 32, 32),
            Painting::Fighters => (0, 96, 64, 32),
            Painting::Pointer => (0, 192, 64, 64),
            Painting::Pigscene => (64, 192, 64, 64),
            Painting::BurningSkull => (128, 192, 64, 64),
            Painting::Skeleton => (192, 64, 64, 48),
            Painting::DonkeyKong => (192, 112, 64, 48),
        }
    }
}
