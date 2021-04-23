use crate::*;

/// [Read about effects](https://minecraft.fandom.com/wiki/Effect)
#[minecraft_enum(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Effect {
    Speed = 1,
    Slowness,
    Haste,
    MiningFatigue,
    Strength,
    InstantHealth,
    InstantDamage,
    JumpBoost,
    Nausea,
    Regeneration,
    Resistance,
    FireResistance,
    WaterBreathing,
    Invisibility,
    Blindness,
    NightVision,
    Hunger,
    Weakness,
    Poison,
    Wither,
    HealthBoost,
    Absorption,
    Saturation,
    Glowing,
    Levitation,
    Luck,
    BadLuck,
    SlowFalling,
    ConduitPower,
    DolphinsGrace,
    BadOmen,
    HeroOfTheVillage,
}

impl Effect {
    pub fn get_name(&self) -> &'static str {
        match self {
            Effect::Speed => "speed",
            Effect::Slowness => "slowness",
            Effect::Haste => "haste",
            Effect::MiningFatigue => "mining_fatigue",
            Effect::Strength => "strength",
            Effect::InstantHealth => "instant_health",
            Effect::InstantDamage => "instant_damage",
            Effect::JumpBoost => "jump_boost",
            Effect::Nausea => "nausea",
            Effect::Regeneration => "regeneration",
            Effect::Resistance => "resistance",
            Effect::FireResistance => "fire_resistance",
            Effect::WaterBreathing => "water_breathing",
            Effect::Invisibility => "invisibility",
            Effect::Blindness => "blindness",
            Effect::NightVision => "night_vision",
            Effect::Hunger => "hunger",
            Effect::Weakness => "weakness",
            Effect::Poison => "poison",
            Effect::Wither => "wither",
            Effect::HealthBoost => "health_boost",
            Effect::Absorption => "absorption",
            Effect::Saturation => "saturation",
            Effect::Glowing => "glowing",
            Effect::Levitation => "levitation",
            Effect::Luck => "luck",
            Effect::BadLuck => "unluck",
            Effect::SlowFalling => "slow_falling",
            Effect::ConduitPower => "conduit_power",
            Effect::DolphinsGrace => "dolphins_grace",
            Effect::BadOmen => "bad_omen",
            Effect::HeroOfTheVillage => "hero_of_the_village",
        }
    }
}
