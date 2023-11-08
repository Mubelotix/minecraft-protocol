use super::*;

#[inherit(Mob, LivingEntity, Entity)]
pub struct EnderDragon {
    pub mob: Mob,
    pub phase: usize,
}

impl Default for EnderDragon {
    fn default() -> Self {
        Self {
            mob: Mob::default(),
            phase: 10,
        }
    }
}

#[inherit(Entity)]
pub struct EndCrystal {
    pub entity: Entity,
    pub block_position: Option<BlockPosition>,
    pub show_bottom: bool,
}

impl Default for EndCrystal {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            block_position: None,
            show_bottom: true,
        }
    }
    
}
