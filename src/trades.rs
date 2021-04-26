use crate::slots::Slot;
use crate::*;

/// Represents a single trade offer
///
/// Notes: Modifiers can increase or decrease the number of items for the first input slot. The second input slot and the output slot never change the nubmer of items. The number of items may never be less than 1, and never more than the stack size. If special price and demand are both zero, only the default price is displayed. If either is non-zero, then the adjusted price is displayed next to the crossed-out default price. The adjusted prices is calculated as follows:  
/// `Adjusted price = default price + floor(default price x multiplier x demand) + special price`
#[derive(Debug, MinecraftPacketPart)]
pub struct Trade {
    /// The first item the player has to supply for this villager trade.
    /// The count of the item stack is the default "price" of this trade.
    pub input_item1: Slot,
    /// The item the player will receive from this villager trade
    pub output_item: Slot,
    /// The second item the player has to supply for this villager trade
    pub input_item2: Option<Slot>,
    /// True if the trade is disabled; false if the trade is enabled
    pub disabled: bool,
    /// Number of times the trade has been used so far.
    /// If equal to the maximum number of trades, the client will display a red X.
    pub use_count: i32,
    /// Number of times this trade can be used before it's exhausted
    pub max_use_count: i32,
    /// Amount of XP both the player and the villager will earn each time the trade is used
    pub xp: i32,
    /// Can be zero or negative.
    /// The number is added to the price when an item is discounted due to player reputation or other effects.
    pub special_price: i32,
    /// Can be low (0.05) or high (0.2).
    /// Determines how much demand, player reputation, and temporary effects will adjust the price.
    pub price_multiplier: f32,
    /// Can be zero or a positive number.
    /// Causes the price to increase when a trade is used repeatedly.
    pub demand: i32,
}

#[minecraft_enum(VarInt)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VillagerType {
    Desert,
    Jungle,
    Plains,
    Savanna,
    Snow,
    Swamp,
    Taiga,
}

#[minecraft_enum(VarInt)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VillagerProfession {
    None,
    Armorer,
    Butcher,
    Cartographer,
    Cleric,
    Farmer,
    Fisherman,
    Fletcher,
    Leatherworker,
    Librarian,
    Mason,
    Nitwit,
    Shepherd,
    Toolsmith,
    Weaponsmith,
}

/// Appears on the trade GUI; meaning comes from the translation key `merchant.level.` + level.
#[minecraft_enum(VarInt)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VillagerLevel {
    Novice = 1,
    Apprentice,
    Journeyman,
    Expert,
    Master,
}
