use super::*;

#[MinecraftEntity(
    inheritable,
    descendants { AbstractArrow..., Boat..., Display, FallingBlock, LlamaSpit, Painting, DragonFireball, Fireball..., FireworkRocket, SmallFireball, Interaction..., ItemEntity, ItemFrame..., LivingEntity... EndCrystal, EvokerFangs, WitherSkull, AreaEffectCloud, FishingHook, EyeOfEnder, ThrownItemProjectile... },
    defines {
        init(self, server_msg_rcvr: BroadcastReceiver<ServerMessage>);
    }
)]

#[derive(Debug)]
pub struct Entity {
    pub position: Position,
    pub velocity: Translation,
    pub pitch: f32,
    pub yaw: f32,
    pub is_on_fire: bool,
    pub is_crouching: bool,
    pub is_sprinting: bool,
    pub is_swimming: bool,
    pub is_invisible: bool,
    pub is_glowing: bool,
    pub is_fying_with_elytra: bool,
    pub air_ticks: u32,
    pub name: Option<String>,
    pub is_name_visible: bool,
    pub is_silent: bool,
    pub has_no_gravity: bool,
    pub pose: Pose,
    pub ticks_frozen: u32,
}

impl Handler<Entity> {
    #[instrument(skip_all)]
    pub async fn init(self, server_msg_rcvr: BroadcastReceiver<ServerMessage>) {
        self.insert_task("newton", tokio::spawn(newton_task(self.clone(), server_msg_rcvr))).await;
    }
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            position: Position { x: 0.0, y: 0.0, z: 0.0 },
            velocity: Translation { x: 0.0, y: 0.0, z: 0.0 },
            pitch: 0.0,
            yaw: 0.0,
            is_on_fire: false,
            is_crouching: false,
            is_sprinting: false,
            is_swimming: false,
            is_invisible: false,
            is_glowing: false,
            is_fying_with_elytra: false,
            air_ticks: 300,
            name: None,
            is_name_visible: false,
            is_silent: false,
            has_no_gravity: false,
            pose: Pose::Standing,
            ticks_frozen: 0,
        }
    }
}
