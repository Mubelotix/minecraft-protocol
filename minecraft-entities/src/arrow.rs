use super::*;

#[MinecraftEntity(
    inheritable, parents { Entity },
)]
pub struct AbstractArrow {
    pub entity: Entity,
    pub is_critical: bool,
    pub is_no_clip: bool,
    pub piercing_level: isize,
}

impl Default for AbstractArrow {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            is_critical: false,
            is_no_clip: false,
            piercing_level: 0,
        }
    }
}

impl TryAsEntityRef<AbstractArrow> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&AbstractArrow> {
        match self {
            AnyEntity::AbstractArrow(abstract_arrow) => Some(abstract_arrow),
            AnyEntity::Arrow(arrow) => Some(&arrow.abstract_arrow),
            AnyEntity::SpectralArrow(spectral_arrow) => Some(&spectral_arrow.abstract_arrow),
            AnyEntity::ThrownTrident(thrown_trident) => Some(&thrown_trident.abstract_arrow),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut AbstractArrow> {
        match self {
            AnyEntity::AbstractArrow(abstract_arrow) => Some(abstract_arrow),
            AnyEntity::Arrow(arrow) => Some(&mut arrow.abstract_arrow),
            AnyEntity::SpectralArrow(spectral_arrow) => Some(&mut spectral_arrow.abstract_arrow),
            AnyEntity::ThrownTrident(thrown_trident) => Some(&mut thrown_trident.abstract_arrow),
            _ => None,
        }
    }
}

#[MinecraftEntity(
    parents { AbstractArrow, Entity },
)]
pub struct Arrow {
    pub abstract_arrow: AbstractArrow,
    pub color: isize,
}

impl Default for Arrow {
    fn default() -> Self {
        Self {
            abstract_arrow: AbstractArrow::default(),
            color: -1,
        }
    }
}

#[MinecraftEntity(
    parents { AbstractArrow, Entity },
)]
pub struct SpectralArrow {
    pub abstract_arrow: AbstractArrow,
    pub loyalty_level: isize,
    pub has_enchantment_glint: bool,
}

impl Default for SpectralArrow {
    fn default() -> Self {
        Self {
            abstract_arrow: AbstractArrow::default(),
            loyalty_level: 0,
            has_enchantment_glint: false,
        }
    }
}

#[MinecraftEntity(
    parents { AbstractArrow, Entity },
)]
pub struct ThrownTrident {
    pub abstract_arrow: AbstractArrow,
    pub loyalty_level: isize,
    pub has_enchantment_glint: bool,
}

impl Default for ThrownTrident {
    fn default() -> Self {
        Self {
            abstract_arrow: AbstractArrow::default(),
            loyalty_level: 0,
            has_enchantment_glint: false,
        }
    }
}
