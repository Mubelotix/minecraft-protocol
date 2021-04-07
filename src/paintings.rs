use crate::packets::serializer::MinecraftPacketPart;
use crate::packets::VarInt;

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
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
    DonkeyKong = 25,
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

impl<'a> MinecraftPacketPart<'a> for Painting {
    fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        VarInt(self as i32).append_minecraft_packet_part(output)
    }

    fn build_from_minecraft_packet(
        input: &'a mut [u8],
    ) -> Result<(Self, &'a mut [u8]), &'static str> {
        let (painting_id, input) = VarInt::build_from_minecraft_packet(input)?;
        let painting = match painting_id.0 {
            0 => Painting::Kebab,
            1 => Painting::Aztec,
            2 => Painting::Alban,
            3 => Painting::Aztec2,
            4 => Painting::Bomb,
            5 => Painting::Plant,
            6 => Painting::Wasteland,
            7 => Painting::Pool,
            8 => Painting::Courbet,
            9 => Painting::Sea,
            10 => Painting::Sunset,
            11 => Painting::Creebet,
            12 => Painting::Wanderer,
            13 => Painting::Graham,
            14 => Painting::Match,
            15 => Painting::Bust,
            16 => Painting::Stage,
            17 => Painting::Void,
            18 => Painting::SkullAndRoses,
            19 => Painting::Wither,
            20 => Painting::Fighters,
            21 => Painting::Pointer,
            22 => Painting::Pigscene,
            23 => Painting::BurningSkull,
            24 => Painting::Skeleton,
            25 => Painting::DonkeyKong,
            _ => return Err("The painting ID is outside the definition range"),
        };
        Ok((painting, input))
    }
}
