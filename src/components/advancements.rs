use crate::*;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, MinecraftPacketPart)]
pub struct Advancement<'a> {
    /// The identifier of the parent advancement
    pub parent_id: Option<Identifier<'a>>,
    pub display_data: Option<AdvancementDisplay<'a>>,
    /// Array of arrays of required criteria
    pub requirements: Array<'a, Array<'a, &'a str, VarInt>, VarInt>,
    /// Whether the client should include this achievement in the telemetry data when it's completed.
    /// The Notchian client only sends data for advancements on the `minecraft` namespace.
    pub sends_telemetry_data: bool,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct AdvancementDisplay<'a> {
    pub title: Chat<'a>,
    pub description: Chat<'a>,
    pub icon: super::slots::Slot,
    pub frame_type: AdvancementFrameType,
    pub show_toast: bool,
    pub hidden: bool,
    pub background_texture: Option<Identifier<'a>>,
    pub x: f32,
    pub y: f32,
}

/// A map linking criterion identifiers to their progress.
pub type AdvancementProgress<'a> = Map<'a, Identifier<'a>, CriterionProgress, VarInt>;

/// Contains the date of achieving or `None` if it has not been achieved.
pub type CriterionProgress = Option<i64>;

impl<'a> MinecraftPacketPart<'a> for AdvancementDisplay<'a> {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        self.title.serialize_minecraft_packet_part(output)?;
        self.description.serialize_minecraft_packet_part(output)?;
        self.icon.serialize_minecraft_packet_part(output)?;
        self.frame_type.serialize_minecraft_packet_part(output)?;
        let flags = (self.background_texture.is_some() as u8)
            + ((self.show_toast as u8) << 1)
            + ((self.hidden as u8) << 2);
        flags.serialize_minecraft_packet_part(output)?;
        if let Some(background_texture) = self.background_texture {
            background_texture.serialize_minecraft_packet_part(output)?;
        }
        self.x.serialize_minecraft_packet_part(output)?;
        self.y.serialize_minecraft_packet_part(output)?;
        Ok(())
    }
    fn deserialize_minecraft_packet_part(
        input: &'a [u8],
    ) -> Result<(Self, &'a [u8]), &'static str> {
        let (title, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (description, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (icon, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (frame_type, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (flags, input) = i32::deserialize_minecraft_packet_part(input)?;
        let has_background_texture = flags & 0b0000_0001 != 0;
        let show_toast = flags & 0b0000_0010 != 0;
        let hidden = flags & 0b0000_0100 != 0;
        let (background_texture, input) = match has_background_texture {
            true => {
                let (background_texture, input) =
                    MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
                (Some(background_texture), input)
            }
            false => (None, input),
        };
        let (x, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (y, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        Ok((
            AdvancementDisplay {
                title,
                description,
                icon,
                frame_type,
                show_toast,
                hidden,
                background_texture,
                x,
                y,
            },
            input,
        ))
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum AdvancementFrameType {
    Task,
    Challenge,
    Goal,
}

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum StatisticCategory {
    Mined,
    Crafted,
    Used,
    Broken,
    PickedUp,
    Dropped,
    Killed,
    KilledBy,
    Custom,
}

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum StatisticId {
    LeaveGame = 0,
    PlayOneMinute,
    TimeSinceDeath,
    TimeSinceRest,
    SneakTime,
    WalkOneCm,
    CrouchOneCm,
    SprintOneCm,
    WalkOnWaterOneCm,
    FallOneCm,
    ClimbOneCm,
    FlyOneCm,
    WalkUnderWaterOneCm,
    MinecartOneCm,
    BoatOneCm,
    PigOneCm,
    HorseOneCm,
    AviateOneCm,
    SwimOneCm,
    StriderOneCm,
    Jump,
    Drop,
    DamageDealt,
    DamageDealtAbsorbed,
    DamageDealtResisted,
    DamageTaken,
    DamageBlockedByShield,
    DamageAbsorbed,
    DamageResisted,
    Deaths,
    MobKills,
    AnimalsBred,
    PlayerKills,
    FishCaught,
    TalkedToVillager,
    TradedWithVillager,
    EatCakeSlice,
    FillCauldron,
    CleanArmor,
    CleanBanner,
    CleanShulkerBox,
    InteractWithBrewingstand,
    InteractWithBeacon,
    InspectDropper,
    InspectHopper,
    InspectDispenser,
    PlayNoteblock,
    TuneNoteblock,
    PotFlower,
    TriggerTrappedChest,
    OpenEnderchest,
    EnchantItem,
    PlayRecord,
    InteractWithFurnace,
    InteractWithCraftingTable,
    OpenChest,
    SleepInBed,
    OpenShulkerBox,
    OpenBarrel,
    InteractWithBlastFurnace,
    InteractWithSmoker,
    InteractWithLectern,
    InteractWithCampfire,
    InteractWithCartographyTable,
    InteractWithLoom,
    InteractWithStonecutter,
    BellRing,
    RaidTrigger,
    RaidWin,
    InteractWithAnvil,
    InteractWithGrindstone,
    TargetHit,
    InteractWithSmithingTable,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, MinecraftPacketPart)]
pub struct Statistic {
    pub category: StatisticCategory,
    /// Used when `category` is [StatisticCategory::Custom].
    /// See [the wiki](https://wiki.vg/Protocol#Statistics) for meaning
    pub statistic_id: StatisticId,
    /// Units depends on previous fields.
    pub value: VarInt,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct AdvancementTabPacket<'a> {
    tab_id: Option<Identifier<'a>>,
}

impl<'a> MinecraftPacketPart<'a> for AdvancementTabPacket<'a> {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        self.tab_id
            .is_none()
            .serialize_minecraft_packet_part(output)?;
        if let Some(tab_id) = self.tab_id {
            tab_id.serialize_minecraft_packet_part(output)?;
        }
        Ok(())
    }
    fn deserialize_minecraft_packet_part(
        input: &'a [u8],
    ) -> Result<(Self, &'a [u8]), &'static str> {
        let (present, input) = VarInt::deserialize_minecraft_packet_part(input)?;
        let (tab_id, input) = if present.0 == 0 {
            let (tab_id, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
            (Some(tab_id), input)
        } else {
            (None, input)
        };

        Ok((AdvancementTabPacket { tab_id }, input))
    }
}
