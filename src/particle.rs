use crate::*;

#[derive(Debug, Clone, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum Particle {
    AmbiantEntityEffect,
    AngryVillager,
    Barrier,
    Block {
        /// Use [Block::from_state_id](crate::ids::blocks::Block::from_state_id) to get the block.
        block_state_id: VarInt,
    },
    Bubble,
    Cloud,
    Crit,
    DamageIndicator,
    DragonBreath,
    DrippingLava,
    FallingLava,
    LandingLava,
    DrippingWater,
    FallingWater,
    Dust {
        /// Red value, between 0.0 and 1.0
        red: f32,
        /// Green value, between 0.0 and 1.0
        green: f32,
        /// Blue value, between 0.0 and 1.0
        blue: f32,
        /// The scale, will be clamped between 0.01 and 4
        scale: f32,
    },
    Effect,
    ElderGuardian,
    EnchantedHit,
    Enchant,
    EndRod,
    EntityEffect,
    ExplosionEmitter,
    Explosion,
    FallingDust {
        /// Use [Block::from_state_id](crate::ids::blocks::Block::from_state_id) to get the block.
        block_state_id: VarInt,
    },
    Firework,
    Fishing,
    Flame,
    Flash,
    HappyVillager,
    Composter,
    Heart,
    InstantEffect,
    Item {
        /// The item that will be used
        item: crate::slots::Slot,
    },
    ItemSlime,
    IemSnowball,
    LargeSmoke,
    Lava,
    Mycelium,
    Note,
    Poof,
    Portal,
    Rain,
    Smoke,
    Sneeze,
    Spit,
    SquidInk,
    SweepAttack,
    TotemOfUndying,
    Underwater,
    Splash,
    Witch,
    BubblePop,
    CurrentDown,
    BubbleColumnUp,
    Nautilus,
    Dolphin,
    CampfireCosySmoke,
    CampfireSignalSmoke,
    DrippingHoney,
    FallingHoney,
    LandingHoney,
    FallingNectar,
}
