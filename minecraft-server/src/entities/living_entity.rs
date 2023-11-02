use super::*;

pub struct LivingEntity {
    pub entity: Entity,
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

pub trait LivingEntityDescendant: EntityDescendant {
    fn get_living_entity(&self) -> &LivingEntity;
    fn get_living_entity_mut(&mut self) -> &mut LivingEntity;
}

impl EntityDescendant for LivingEntity {
    fn get_entity(&self) -> &Entity { &self.entity }
    fn get_entity_mut(&mut self) -> &mut Entity { &mut self.entity }
}

impl LivingEntityDescendant for LivingEntity {
    fn get_living_entity(&self) -> &LivingEntity { self }
    fn get_living_entity_mut(&mut self) -> &mut LivingEntity { self }
}
