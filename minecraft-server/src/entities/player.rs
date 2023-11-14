use super::*;

#[MinecraftEntity(
    ancestors { LivingEntity, Entity },
    defines {
        Entity.init(self, server_msg_rcvr: BroadcastReceiver<ServerMessage>);
    }
)]
pub struct Player {
    pub living_entity: LivingEntity,
    pub additional_hearts: f32,
    pub score: usize,
    pub is_cape_enabled: bool,
    pub is_jacket_enabled: bool,
    pub is_left_sleeve_enabled: bool,
    pub is_right_sleeve_enabled: bool,
    pub is_left_pants_leg_enabled: bool,
    pub is_right_pants_leg_enabled: bool,
    pub is_hat_enabled: bool,
    pub main_hand: Hand,
    pub left_shoulder_entity: NbtTag,
    pub right_shoulder_entity: NbtTag,
}

impl Handler<Player> {
    pub async fn init(self, server_msg_rcvr: BroadcastReceiver<ServerMessage>) {
        //self.insert_task("newton", tokio::spawn(newton_task(self.clone(), server_msg_rcvr))).await;
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            living_entity: LivingEntity::default(),
            additional_hearts: 0.0,
            score: 0,
            is_cape_enabled: false,
            is_jacket_enabled: false,
            is_left_sleeve_enabled: false,
            is_right_sleeve_enabled: false,
            is_left_pants_leg_enabled: false,
            is_right_pants_leg_enabled: false,
            is_hat_enabled: false,
            main_hand: Hand::MainHand,
            left_shoulder_entity: NbtTag::Null,
            right_shoulder_entity: NbtTag::Null,
        }
    }
}
