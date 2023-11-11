use super::*;
use minecraft_protocol::components::paintings::Painting as PaintingType;

#[MinecraftEntity(
    inheritable, ancestors { Entity },
)]
pub struct Display {
    pub entity: Entity,
    pub interpolation_delay: u32,
    pub transformation_interpolation_duration: u32,
    pub position_interpolation_duration: u32,
    pub translation: (f64, f64, f64),
    pub scale: (f64, f64, f64),
    pub rotation_left: (f64, f64, f64, f64),
    pub rotation_right: (f64, f64, f64, f64),
    pub fixed_constraint: bool,
    pub vertical_constraint: bool,
    pub horizontal_constraint: bool,
    pub center_constraint: bool,
    pub brightness: isize,
    pub view_range: f32,
    pub shadow_radius: f32,
    pub shadow_strenght: f32,
    pub width: f32,
    pub height: f32,
    pub glow_color: isize,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            interpolation_delay: 0,
            transformation_interpolation_duration: 0,
            position_interpolation_duration: 0,
            translation: (0., 0., 0.),
            scale: (0., 0., 0.),
            rotation_left: (0., 0., 0., 1.),
            rotation_right: (0., 0., 0., 1.),
            fixed_constraint: false,
            vertical_constraint: false,
            horizontal_constraint: false,
            center_constraint: false,
            brightness: -1,
            view_range: 1.,
            shadow_radius: 0.,
            shadow_strenght: 1.,
            width: 0.,
            height: 0.,
            glow_color: -1,
        }
    }
}

impl TryAsEntityRef<Display> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&Display> {
        match self {
            AnyEntity::Display(display) => Some(display),
            AnyEntity::BlockDisplay(block_display) => Some(&block_display.display),
            AnyEntity::ItemDisplay(item_display) => Some(&item_display.display),
            AnyEntity::TextDisplay(text_display) => Some(&text_display.display),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut Display> {
        match self {
            AnyEntity::Display(display) => Some(display),
            AnyEntity::BlockDisplay(block_display) => Some(&mut block_display.display),
            AnyEntity::ItemDisplay(item_display) => Some(&mut item_display.display),
            AnyEntity::TextDisplay(text_display) => Some(&mut text_display.display),
            _ => None,
        }
    }
}

#[MinecraftEntity(
    ancestors { Display, Entity },
)]
pub struct BlockDisplay {
    pub display: Display,
    pub block: BlockWithState,
}

impl Default for BlockDisplay {
    fn default() -> Self {
        Self {
            display: Display::default(),
            block: BlockWithState::Air,
        }
    }
}

#[MinecraftEntity(
    ancestors { Display, Entity },
)]
pub struct ItemDisplay {
    pub display: Display,
    pub item: Slot,
    pub display_type: u8,
}

impl Default for ItemDisplay {
    fn default() -> Self {
        Self {
            display: Display::default(),
            item: Slot { item: None },
            display_type: 0,
        }
    }
}

#[MinecraftEntity(
    ancestors { Display, Entity },
)]
pub struct TextDisplay {
    pub display: Display,
    pub text: String,
    pub line_width: usize,
    pub background_color: isize,
    pub text_opacity: i8,
    pub has_shadow: bool,
    pub is_seethrough: bool,
    pub use_default_background: bool,
    pub alignement: u8,
}

impl Default for TextDisplay {
    fn default() -> Self {
        Self {
            display: Display::default(),
            text: String::new(),
            line_width: 0,
            background_color: 1073741824,
            text_opacity: -1,
            has_shadow: false,
            is_seethrough: false,
            use_default_background: false,
            alignement: 0,
        }
    }
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Entity },
)]
pub struct Painting {
    pub entity: Entity,
    pub painting_type: PaintingType,
}
