use super::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: u32,
    display_name: String,
    #[serde(rename = "name")]
    pub text_id: String,
    stack_size: u8,
    max_durability: Option<u16>,
}

#[allow(clippy::explicit_counter_loop)]
pub fn generate_item_enum(data: serde_json::Value) -> Vec<Item> {
    let mut items: Vec<Item> = serde_json::from_value(data).expect("Invalid block data");
    items.sort_by_key(|item| item.id);

    // Patch the missing Air
    if items.first().map(|i| i.id) != Some(0) {
        items.insert(
            0,
            Item {
                id: 0,
                display_name: String::from("Air"),
                text_id: String::from("air"),
                stack_size: 64,
                max_durability: None,
            },
        );
    }

    // Look for missing items in the array
    let mut expected = 0;
    for item in &items {
        if item.id != expected {
            panic!("The item with id {} is missing.", expected)
        }
        expected += 1;
    }

    // Generate the variants of the Item enum
    let mut variants = String::new();
    for item in &items {
        let name = item
            .text_id
            .from_case(Case::Snake)
            .to_case(Case::UpperCamel);
        variants.push_str(&format!("\t{} = {},\n", name, item.id));
    }

    // Generate the code
    let code = format!(
        r#"use crate::*;

/// See [implementations](#implementations) for useful methods.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Item {{
{variants}
}}

impl Item {{
    #[inline]
    pub fn from_id(id: u32) -> Option<Item> {{
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
    pub fn max_stack_size(self) -> u8 {{
        unsafe {{*STACK_SIZES.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn durability(self) -> Option<u16> {{
        unsafe {{*DURABILITIES.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn crafting_recipes(&self) -> &'static [crate::ids::recipes::Recipe] {{
        crate::ids::recipes::Recipe::get_recipes_for_item(*self)
    }}
}}

impl<'a> MinecraftPacketPart<'a> for Item {{
    #[inline]
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {{
        VarInt(self as i32).serialize_minecraft_packet_part(output)
    }}

    #[inline]
    fn deserialize_minecraft_packet_part(input: &'a[u8]) -> Result<(Self, &'a[u8]), &'static str> {{
        let (id, input) = VarInt::deserialize_minecraft_packet_part(input)?;
        let id = std::cmp::max(id.0, 0) as u32;
        let item = Item::from_id(id).ok_or("No item corresponding to the specified numeric ID.")?;
        Ok((item, input))
    }}
}}

const STACK_SIZES: [u8; {max_value}] = {max_stack_sizes:?};

const DURABILITIES: [Option<u16>; {max_value}] = {durabilities:?};

const DISPLAY_NAMES: [&str; {max_value}] = {display_names:?};

const TEXT_IDS: [&str; {max_value}] = {text_ids:?};
"#,
        variants = variants,
        max_value = expected,
        max_stack_sizes = items.iter().map(|i| i.stack_size).collect::<Vec<_>>(),
        durabilities = items.iter().map(|i| i.max_durability).collect::<Vec<_>>(),
        display_names = items.iter().map(|i| &i.display_name).collect::<Vec<_>>(),
        text_ids = items.iter().map(|i| &i.text_id).collect::<Vec<_>>(),
    );

    File::create("src/ids/items.rs")
        .unwrap()
        .write_all(code.as_bytes())
        .unwrap();

    items
}
