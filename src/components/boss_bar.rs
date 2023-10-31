use crate::*;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum BossBarAction<'a> {
    Add {
        title: Chat<'a>,
        /// From 0 to 1. Values greater than 1 do not crash a Notchian client, and start [rendering part of a second health bar](https://i.johni0702.de/nA.png) at around 1.5.
        health: f32,
        color: Color,
        division: Division,
        /// Bit mask. 0x1: should darken sky, 0x2: is dragon bar (used to play end music), 0x04: create fog (previously was also controlled by 0x02).
        flags: u8,
    },
    Remove,
    UpdateHealth {
        /// From 0 to 1. Values greater than 1 do not crash a Notchian client, and start [rendering part of a second health bar](https://i.johni0702.de/nA.png) at around 1.5.
        health: f32,
    },
    UpdateTitle {
        title: Chat<'a>,
    },
    UpdateStyle {
        color: Color,
        division: Division,
    },
    UpdateFlages {
        /// Bit mask. 0x1: should darken sky, 0x2: is dragon bar (used to play end music), 0x04: create fog (previously was also controlled by 0x02).
        flags: u8,
    },
}

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum Color {
    Pink,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum Division {
    NoDivision,
    SixNotches,
    TenNotches,
    TwelveNotches,
    TwentyNotches,
}
