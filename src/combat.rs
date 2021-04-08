use crate::*;

#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum CombatEvent<'a> {
    EnterCombat,
    EndCombat {
        /// Length of the combat in ticks
        duration: VarInt,
        /// ID of the primary opponent of the ended combat, or -1 if there is no obvious primary opponent
        opponent_entity_id: VarInt,
    },
    EntityDead {
        /// Entity ID of the player that died (should match the client's entity ID)
        dead_entity_id: VarInt,
        /// The killing entity's ID, or -1 if there is no obvious killer
        killer_entity_id: VarInt,
        death_message: Chat<'a>,
    },
}
