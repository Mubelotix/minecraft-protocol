use super::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Entity {
    id: u32,
    #[serde(rename = "name")]
    text_id: String,
    display_name: String,
    width: f32,
    height: f32,
    #[serde(rename = "type")]
    category: String,
}

pub fn generate_entity_enum(data: serde_json::Value) {
    let mut entities: Vec<Entity> = serde_json::from_value(data).expect("Invalid entity data");
    entities.sort_by_key(|entity| entity.id);

    // Look for missing items in the array
    let mut expected = 0;
    for entity in &entities {
        if entity.id != expected {
            panic!("The entity with id {} is missing.", expected)
        }
        expected += 1;
    }

    // Generate the categories array
    let mut categories = String::new();
    categories.push('[');
    for entity in &entities {
        let variant_name = match entity.category.as_str() {
            "other" => "Other",
            "living" => "Living",
            "projectile" => "Projectile",
            "animal" => "Animal",
            "ambient" => "Ambient",
            "hostile" => "Hostile",
            "water_creature" => "WaterCreature",
            "mob" => "Mob",
            "passive" => "Passive",
            "player" => "Player",
            unknown_category => panic!("Unknown entity category {}", unknown_category),
        };
        categories.push_str("EntityCategory::");
        categories.push_str(variant_name);
        categories.push_str(", ");
    }
    categories.push(']');

    // Generate the variants of the Item enum
    let mut variants = String::new();
    for entity in &entities {
        let name = entity
            .text_id
            .from_case(Case::Snake)
            .to_case(Case::UpperCamel);
        variants.push_str(&format!("\t{} = {},\n", name, entity.id));
    }

    // Generate the code
    let code = format!(
        r#"use crate::*;

/// See [implementations](#implementations) for useful methods.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Entity {{
{variants}
}}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityCategory {{
    Other,
    Living,
    Projectile,
    Animal,
    Ambient,
    Hostile,
    WaterCreature,
    Mob,
    Passive,
    Player,
}}

impl Entity {{
    #[inline]
    pub fn from_id(id: u32) -> Option<Entity> {{
        if id < {max_value} {{
            Some(unsafe{{std::mem::transmute(id)}})
        }} else {{
            None
        }}
    }}

    #[inline]
    pub fn text_id(self) -> &'static str {{
        unsafe {{*TEXT_IDS.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn display_name(self) -> &'static str {{
        unsafe {{*DISPLAY_NAMES.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn category(self) -> EntityCategory {{
        unsafe {{*CATEGORIES.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn height(self) -> f32 {{
        unsafe {{*HEIGHTS.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn width(self) -> f32 {{
        unsafe {{*WIDTHS.get_unchecked((self as u32) as usize)}}
    }}
}}

impl<'a> MinecraftPacketPart<'a> for Entity {{
    #[inline]
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {{
        VarInt((self as u32) as i32).serialize_minecraft_packet_part(output)
    }}

    #[inline]
    fn deserialize_minecraft_packet_part(input: &'a[u8]) -> Result<(Self, &'a[u8]), &'static str> {{
        let (id, input) = VarInt::deserialize_minecraft_packet_part(input)?;
        let id = std::cmp::max(id.0, 0) as u32;
        let entity = Entity::from_id(id).ok_or("No entity corresponding to the specified numeric ID.")?;
        Ok((entity, input))
    }}
}}

const HEIGHTS: [f32; {max_value}] = {heights:?};

const WIDTHS: [f32; {max_value}] = {widths:?};

const DISPLAY_NAMES: [&str; {max_value}] = {display_names:?};

const TEXT_IDS: [&str; {max_value}] = {text_ids:?};

const CATEGORIES: [EntityCategory; {max_value}] = {categories};
"#,
        variants = variants,
        max_value = expected,
        heights = entities.iter().map(|e| e.height).collect::<Vec<_>>(),
        widths = entities.iter().map(|e| e.width).collect::<Vec<_>>(),
        display_names = entities.iter().map(|e| &e.display_name).collect::<Vec<_>>(),
        text_ids = entities.iter().map(|e| &e.text_id).collect::<Vec<_>>(),
        categories = categories,
    );

    File::create("src/ids/entities.rs")
        .unwrap()
        .write_all(code.as_bytes())
        .unwrap()
}
