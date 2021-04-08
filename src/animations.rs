use crate::*;

#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum Animation {
    SwingMainArm = 0,
    TakeDamage,
    LeaveBed,
    SwingOffhand,
    CriticalEffect,
    MagicCriticalEffect = 5,
}

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
    /// Play record: This is actually a special case within the packet [packets::play_clientbound::ClientBoundPacket::Effect].
    /// You can start/stop a record at a specific location.
    /// Use a valid Record ID to start a record (or overwrite a currently playing one), any other value will stop the record.
    /// See [Data Generators](https://wiki.vg/Data_Generators) for information on item IDs.
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
    BlockBreack,
    /// Particle effect + glass break sound.
    SplashPotion,
    /// particles and sound
    EyeOfEnderBreak,
    /// particle effect: smoke + flames
    MobSpawn,
    BonemealParticules,
    DragonBreath,
    /// Particle effect + glass break sound.
    InstantSplashPotion,
    EnderdragonDestroysBlock,
    WetSpongeVaporizesInNether,

    EndGatewaySpawn = 3000,
    EnderdragonGrowl,
}
