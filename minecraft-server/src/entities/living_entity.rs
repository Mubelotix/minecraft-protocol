use super::*;

#[derive(Clone)]
#[MinecraftEntity(
    inheritable,
    ancestors { Entity },
    descendants { Player, ArmorStand, Mob... },
)]
pub struct LivingEntity {
    pub entity: Entity,
    pub head_yaw: f32,
    pub is_hand_active: bool,
    pub active_hand: Hand,
    pub is_riptide_spinning: bool,
    pub health: f32,
    pub potion_effect_color: usize,
    pub is_potion_effect_ambient: bool,
    pub arrows_count: usize,
    pub bee_stinger_count: usize,
    pub bed: Option<BlockPosition>,
}

impl Default for LivingEntity {
    fn default() -> Self {
        LivingEntity {
            entity: Entity::default(),
            head_yaw: 0.0,
            is_hand_active: false,
            active_hand: Hand::MainHand,
            is_riptide_spinning: false,
            health: 1.0,
            potion_effect_color: 0,
            is_potion_effect_ambient: false,
            arrows_count: 0,
            bee_stinger_count: 0,
            bed: None,
        }
    }
}

#[MinecraftEntity(
    ancestors { LivingEntity, Entity },
)]
pub struct ArmorStand {
    pub living_entity: LivingEntity,
    pub apparence_mask: u8,
    pub head_rotation: Rotation,
    pub body_rotation: Rotation,
    pub left_arm_rotation: Rotation,
    pub right_arm_rotation: Rotation,
    pub left_leg_rotation: Rotation,
    pub right_leg_rotation: Rotation,
}

impl Default for ArmorStand {
    fn default() -> Self {
        Self {
            living_entity: LivingEntity::default(),
            apparence_mask: 0,
            head_rotation: Rotation::default(),
            body_rotation: Rotation::default(),
            left_arm_rotation: Rotation { 
                x: -10.0, 
                y: 0.0, 
                z: -10.0,
            },
            right_arm_rotation: Rotation { 
                x: -15.0, 
                y: 0.0, 
                z: 10.0,
            },
            left_leg_rotation: Rotation { 
                x: -1.0, 
                y: 0.0, 
                z: -1.0,
            },
            right_leg_rotation: Rotation { 
                x: 1.0, 
                y: 0.0, 
                z: 1.0,
            },
        }
    }
}
