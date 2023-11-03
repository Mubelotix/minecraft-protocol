use super::*;

#[inherit(Entity)]
#[inheritable]
pub struct Display {
    pub entity: Entity,
    interpolation_delay: u32,
    transformation_interpolation_duration: u32,
    position_interpolation_duration: u32,
    translation: (f64, f64, f64),
    scale: (f64, f64, f64),
    rotation_left: (f64, f64, f64, f64),
    rotation_right: (f64, f64, f64, f64),
    fixed_constraint: bool,
    vertical_constraint: bool,
    horizontal_constraint: bool,
    center_constraint: bool,
    brightness: isize,
    view_range: f32,
    shadow_radius: f32,
    shadow_strenght: f32,
    width: f32,
    height: f32,
    glow_color: isize,
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

#[inherit(Display, Entity)]
pub struct BlockDisplay {
    display: Display,
    block: BlockWithState,
}

impl Default for BlockDisplay {
    fn default() -> Self {
        Self {
            display: Display::default(),
            block: BlockWithState::Air,
        }
    }
}

#[inherit(Display, Entity)]
pub struct ItemDisplay {
    display: Display,
    item: Slot,
    display_type: u8,
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

#[inherit(Display, Entity)]
pub struct TextDisplay {
    display: Display,
    text: String,
    line_width: usize,
    background_color: isize,
    text_opacity: i8,
    has_shadow: bool,
    is_seethrough: bool,
    use_default_background: bool,
    alignement: u8,
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
