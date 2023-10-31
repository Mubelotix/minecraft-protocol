use crate::*;

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum Animation {
    SwingMainArm = 0,
    TakeDamage, /// Not in the wiki maybe disapeared?
    LeaveBed,
    SwingOffhand,
    CriticalEffect,
    MagicCriticalEffect = 5,
}

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(i32)]
#[derive(Debug)]
pub enum Effect {
    DispenserDispenses = 1000,
    DispenserFailToDispense,
    DispenserShoots,
    EnderEyeLaunched,
    FireworkShot,
    IronDoorOpened,
    WoodenDoorOpened,
    WoodenTrapdoorOpened,
    FenceGateOpened,
    FireExtinguished,
    /// Play record: This is actually a special case within the packet [packets::play_clientbound::ClientboundPacket::Effect].
    /// You can start/stop a record at a specific location.
    /// Use a valid Record ID to start a record (or overwrite a currently playing one), any other value will stop the record.
    /// See [item IDs](crate::ids::items::Item).
    PlayRecord,
    IronDoorClosed,
    WoodenDoorClosed,
    WoodenTrapdoorClosed,
    FenceGateClosed,
    GhastWarns,
    GhastShoots,
    EnderdragonShoots,
    BlazeShoots,
    ZombieAttacksWoodDoor,
    ZombieAttacksIronDoor,
    ZombieBreackWoodDoor,
    WitherBreacksBlock,
    WitherSpawn,
    WitherShoots,
    BatTakesOff,
    ZoombieInfects,
    ZoombieVillagerConverted,
    EnderdragonDeath,
    AnvilDestroyed,
    AnvilUsed,
    AnvilLanded,
    PortalTravel,
    ChorusFlowerGrown,
    ChorusFlowerDied,
    BrewingStandBrewed,
    IronTrapdoorOpened,
    IronTrapdoorClosed,
    EndPortalOpening,
    PhantomBites,
    ZombieConvertsToDrowned,
    HuskConvertsToZombieByDrowning,
    GrindStoneUsed,
    BookPageTurned,

    ComposerComposts = 1500,
    /// either water to stone, or removes existing blocks such as torches
    LavaConvertsBlock,
    RedstoneTorchBurnsOut,
    EnderEyePlaced,

    /// Spawns 10 smoke particles, e.g. from a fire.
    SmokeParticules = 2000,
    /// Block state, as an index into the global palette.
    BlockBreack,
    /// Particle effect + glass break sound.
    /// RGB color as an integer (e.g. 8364543 for #7FA1FF).
    SplashPotion,
    /// particles and sound
    EyeOfEnderBreak,
    /// particle effect: smoke + flames
    MobSpawn,
    /// How many particles to spawn (if set to 0, 15 are spawned).
    BonemealParticules,
    DragonBreath,
    /// Particle effect + glass break sound.
    InstantSplashPotion,
    EnderdragonDestroysBlock,
    WetSpongeVaporizesInNether,

    EndGatewaySpawn = 3000,
    EnderdragonGrowl,
    ElectricSpark,
    CopperRemoveWax,
    CopperScrapeOxidation,
}
