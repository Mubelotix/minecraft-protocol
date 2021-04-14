use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};
use std::io::{ErrorKind, Read, Write};

const VERSION: &str = "1.16.5";

fn get_data(url: &str, cache: &str) -> serde_json::Value {
    match File::open(cache) {
        // The cache file is ready
        Ok(mut file) => {
            let mut data = Vec::new();
            if let Err(e) = file.read_to_end(&mut data) {
                panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded and cached to `{}`. Unfortunately, this file cannot be read. Error: {}", cache, e)
            }

            let json_text = match String::from_utf8(data) {
                Ok(json_text) => json_text,
                Err(e) => panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded and cached to `{}`. Unfortunately, this file appears to contain invalid text data. Error: {}\nNote: Deleting the file will allow the library to download it again.", cache, e),
            };

            let json = match serde_json::from_str(&json_text) {
                Ok(json) => json,
                Err(e) => panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded and cached to `{}`. Unfortunately, this file appears to contain invalid json data. Error: {}\nNote: Deleting the file will allow the library to download it again.", cache, e),
            };

            json
        }
        // The cache file needs to be downloaded
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let response = match minreq::get(url).send() {
                Ok(response) => response,
                Err(e) => panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded from `{}`. Unfortunately, we can't access this URL. Error: {}", url, e)
            };

            let json_text = match response.as_str() {
                Ok(json_text) => json_text,
                Err(e) => panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded from `{}`. Unfortunately, this file appears to contain invalid data. Error: {}", url, e),
            };

            let mut file = match File::create(cache) {
                Ok(file) => file,
                Err(e) => panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded and cached to `{}`. Unfortunately, we can't access this path. Error: {}", cache, e),
            };

            if let Err(e) = file.write_all(json_text.as_bytes()) {
                panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded and cached to `{}`. Unfortunately, we can't write to this path. Error: {}", cache, e)
            };

            let json = match serde_json::from_str(json_text) {
                Ok(json) => json,
                Err(e) => panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded and cached to `{}`. Unfortunately, this file appears to contain invalid json data. Error: {}\nNote: Deleting the file will allow the library to download it again.", cache, e),
            };

            json
        }

        // The cache file cannot be accessed
        Err(e) => {
            panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded and cached to `{}`. Unfortunately, we can't access this path. Error: {}", cache, e);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct BlockState {
    // Empty for now
    // It would be a great feature to add
}

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    id: u32,
    explosion_resistance: f32,
    item: u32,
    default_state: u32,
    class: String,
    states: HashMap<u32, BlockState>,
}

#[allow(clippy::explicit_counter_loop)]
fn generate_block_enum(data: serde_json::Value) {
    let blocks: HashMap<String, Block> = serde_json::from_value(data).expect("Invalid block data");
    let mut blocks: Vec<(String, Block)> = blocks.into_iter().collect();

    let len = blocks.len();
    blocks.sort_by_key(|(_name, block)| block.id);

    let mut expected = 0;
    for (_name, block) in &blocks {
        if block.id != expected {
            panic!("The block with id {} is missing.", expected)
        }
        expected += 1;
    }

    let mut display_names = Vec::new();
    let mut state_id_ranges = Vec::new();
    let mut default_state_ids = Vec::new();
    let mut numeric_ids = Vec::new();
    let mut text_ids = Vec::new();
    let mut item_ids = Vec::new();
    let mut explosion_resistances = Vec::new();
    let mut raw_classes = Vec::new();
    for (name, block) in blocks {
        let display_name = name.strip_prefix("minecraft:").unwrap_or(&name);
        let display_name = display_name.from_case(Case::Snake).to_case(Case::Title);

        let mut start_state_ids = u32::MAX;
        let mut end_state_ids = u32::MIN;
        for state_id in block.states.keys() {
            if *state_id + 1 > end_state_ids {
                end_state_ids = *state_id + 1;
            }
            if *state_id < start_state_ids {
                start_state_ids = *state_id;
            }
        }
        for state_id in start_state_ids..end_state_ids {
            if !block.states.contains_key(&state_id) {
                panic!("Missing state id {} in block {}", state_id, name)
            }
        }

        display_names.push(display_name);
        state_id_ranges.push(start_state_ids..end_state_ids);
        default_state_ids.push(block.default_state);
        numeric_ids.push(block.id);
        text_ids.push(name);
        item_ids.push(block.item);
        explosion_resistances.push(block.explosion_resistance);
        raw_classes.push(block.class);
    }

    let mut different_classes = raw_classes.clone();
    different_classes.sort();
    different_classes.dedup();

    let mut class_variants = String::new();
    for class in different_classes {
        class_variants.push_str(&format!("\t{},\n", class));
    }

    let mut classes = String::new();
    classes.push('[');
    for class in raw_classes {
        classes.push_str("BlockClass::");
        classes.push_str(&class);
        classes.push_str(", ");
    }
    classes.push(']');

    let mut variants = String::new();
    for i in 0..len {
        let name = text_ids[i].strip_prefix("minecraft:").unwrap_or(&text_ids[i]);
        let name = name.from_case(Case::Snake).to_case(Case::UpperCamel);
        variants.push_str(&format!("\t{} = {},\n", name, numeric_ids[i]));
    }

    let mut state_id_match_arms = String::new();
    for i in 0..len {
        let name = text_ids[i].strip_prefix("minecraft:").unwrap_or(&text_ids[i]);
        let name = name.from_case(Case::Snake).to_case(Case::UpperCamel);
        let start = state_id_ranges[i].start;
        let stop = state_id_ranges[i].end - 1;
        if start != stop {
            state_id_match_arms.push_str(&format!(
                "\t\t\t{}..={} => Some(Block::{}),\n",
                start, stop, name
            ));
        } else {
            state_id_match_arms.push_str(&format!("\t\t\t{} => Some(Block::{}),\n", start, name));
        }
    }

    let code = format!(
        r#"use crate::*;
        
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Block {{
{variants}
}}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockClass {{
{class_variants}
}}

impl Block {{
    #[inline]
    pub fn from_id(id: u32) -> Option<Block> {{
        if id < {max_value} {{
            Some(unsafe{{std::mem::transmute(id)}})
        }} else {{
            None
        }}
    }}

    pub fn from_state_id(state_id: u32) -> Option<Block> {{
        match state_id {{
{state_id_match_arms}
            _ => None,
        }}
    }}

    /// Get the textual identifier of this block.
    /// This library supports blocks in the `minecraft` namespace, so all the identifiers will be prefixed with `minecraft:`.
    #[inline]
    pub fn get_text_id(self) -> &'static str {{
        unsafe {{*TEXT_IDS.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn get_default_state_id(self) -> u32 {{
        unsafe {{*DEFAULT_STATE_IDS.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn get_id(self) -> u32 {{
        self as u32
    }}

    #[inline]
    pub fn get_associated_item_id(self) -> u32 {{
        unsafe {{*ITEM_IDS.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn get_block_class(self) -> BlockClass {{
        unsafe {{*CLASSES.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn get_explosion_resistance(self) -> f32 {{
        unsafe {{*EXPLOSION_RESISTANCES.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn get_display_name(self) -> &'static str {{
        unsafe {{*DISPLAY_NAMES.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn get_state_id_range(self) -> std::ops::Range<u32> {{
        unsafe {{STATE_ID_RANGES.get_unchecked((self as u32) as usize).clone()}}
    }}
}}

impl<'a> MinecraftPacketPart<'a> for Block {{
    #[inline]
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {{
        VarInt((self as u32) as i32).serialize_minecraft_packet_part(output)
    }}

    #[inline]
    fn deserialize_minecraft_packet_part(input: &'a mut [u8]) -> Result<(Self, &'a mut [u8]), &'static str> {{
        let (id, input) = VarInt::deserialize_minecraft_packet_part(input)?;
        let id = std::cmp::max(id.0, 0) as u32;
        let block = Block::from_id(id).ok_or("No block corresponding to the specified numeric ID.")?;
        Ok((block, input))
    }}
}}

const TEXT_IDS: [&str; {max_value}] = {text_ids:?};

const DISPLAY_NAMES: [&str; {max_value}] = {display_names:?};

const STATE_ID_RANGES: [std::ops::Range<u32>; {max_value}] = {state_id_ranges:?};

const DEFAULT_STATE_IDS: [u32; {max_value}] = {default_state_ids:?};

const ITEM_IDS: [u32; {max_value}] = {item_ids:?};

const EXPLOSION_RESISTANCES: [f32; {max_value}] = {explosion_resistances:?};

const CLASSES: [BlockClass; {max_value}] = {classes};
"#,
        variants = variants,
        class_variants = class_variants,
        max_value = expected,
        state_id_match_arms = state_id_match_arms,
        text_ids = text_ids,
        display_names = display_names,
        state_id_ranges = state_id_ranges,
        default_state_ids = default_state_ids,
        item_ids = item_ids,
        classes = classes,
        explosion_resistances = explosion_resistances,
    );

    File::create("src/ids/blocks.rs")
        .unwrap()
        .write_all(code.as_bytes())
        .unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    display_name: String,
    text_id: String,
    numeric_id: u32,
    max_stack_size: u8,
}

/*
#[allow(clippy::explicit_counter_loop)]
fn generate_item_enum(data: &serde_json::Value) {
    let items_json = data
        .get(0)
        .expect("Burger data is not an array")
        .get("items")
        .expect("No items in burger's json data")
        .get("item")
        .expect("expected item in items")
        .as_object()
        .expect("expected item to be an object")
        .clone();

    let mut items = Vec::new();
    for (key, item) in items_json.into_iter() {
        let item: Item = match serde_json::from_value(item) {
            Ok(item) => item,
            Err(e) => panic!("Invalid item: {}, {}", key, e),
        };
        items.push(item);
    }
    let len = items.len();
    items.sort_by_key(|b| b.numeric_id);

    let mut expected = 0;
    for item in &items {
        if item.numeric_id != expected {
            panic!("The item with id {} is missing.", expected)
        }
        expected += 1;
    }

    let mut display_names = Vec::new();
    let mut max_stack_sizes = Vec::new();
    let mut numeric_ids = Vec::new();
    let mut text_ids = Vec::new();
    for item in items {
        display_names.push(item.display_name);
        numeric_ids.push(item.numeric_id);
        max_stack_sizes.push(item.max_stack_size);
        text_ids.push(item.text_id);
    }

    let mut variants = String::new();
    for i in 0..len {
        let name = text_ids[i].to_case(Case::UpperCamel);
        variants.push_str(&format!("\t{} = {},\n", name, numeric_ids[i]));
    }

    let code = format!(
        r#"use crate::*;
        
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
    pub fn get_text_id(self) -> &'static str {{
        unsafe {{*TEXT_IDS.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn get_display_name(self) -> &'static str {{
        unsafe {{*DISPLAY_NAMES.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn get_max_stack_size(self) -> u8 {{
        unsafe {{*MAX_STACK_SIZES.get_unchecked((self as u32) as usize)}}
    }}
}}

impl<'a> MinecraftPacketPart<'a> for Item {{
    #[inline]
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {{
        VarInt((self as u32) as i32).serialize_minecraft_packet_part(output)
    }}

    #[inline]
    fn deserialize_minecraft_packet_part(input: &'a mut [u8]) -> Result<(Self, &'a mut [u8]), &'static str> {{
        let (id, input) = VarInt::deserialize_minecraft_packet_part(input)?;
        let id = std::cmp::max(id.0, 0) as u32;
        let item = Item::from_id(id).ok_or("No item corresponding to the specified numeric ID.")?;
        Ok((item, input))
    }}
}}

const MAX_STACK_SIZES: [u8; {max_value}] = {max_stack_sizes:?};

const DISPLAY_NAMES: [&str; {max_value}] = {display_names:?};

const TEXT_IDS: [&str; {max_value}] = {text_ids:?};
"#,
        variants = variants,
        max_value = expected,
        max_stack_sizes = max_stack_sizes,
        display_names = display_names,
        text_ids = text_ids,
    );

    File::create("src/ids/items.rs")
        .unwrap()
        .write_all(code.as_bytes())
        .unwrap()
}*/

fn main() {
    //println!("cargo:rerun-if-changed=target/burger-cache-{}.json", VERSION);
    let block_data = get_data(&format!("https://gitlab.bixilon.de/bixilon/pixlyzer-data/-/raw/master/version/{}/blocks.min.json", VERSION), &format!("target/cache-blocks-{}.json", VERSION));
    generate_block_enum(block_data);
    //generate_item_enum(&data);
}
