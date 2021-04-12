use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};

const VERSION: &str = "1.16.5";

fn get_burger_data() -> serde_json::Value {
    match File::open(format!("target/burger-cache-{}.json", VERSION)) {
        // The cache file is ready
        Ok(mut file) => {
            let mut data = Vec::new();
            if let Err(e) = file.read_to_end(&mut data) {
                panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded and cached to `target/burger-cache-{}.json`. Unfortunately, this file cannot be read. Error: {}", VERSION, e)
            }

            let json_text = match String::from_utf8(data) {
                Ok(json_text) => json_text,
                Err(e) => panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded and cached to `target/burger-cache-{}.json`. Unfortunately, this file appears to contain invalid text data. Error: {}\nNote: Deleting the file will allow the library to download it again.", VERSION, e),
            };

            let json = match serde_json::from_str(&json_text) {
                Ok(json) => json,
                Err(e) => panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded and cached to `target/burger-cache-{}.json`. Unfortunately, this file appears to contain invalid json data. Error: {}\nNote: Deleting the file will allow the library to download it again.", VERSION, e),
            };

            json
        }
        // The cache file needs to be downloaded
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let response = match minreq::get(format!("https://raw.githubusercontent.com/Pokechu22/Burger/gh-pages/{}.json", VERSION)).send() {
                Ok(response) => response,
                Err(e) => panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded from `https://raw.githubusercontent.com/Pokechu22/Burger/gh-pages/{}.json`. Unfortunately, we can't access this URL. Error: {}", VERSION, e)
            };

            let json_text = match response.as_str() {
                Ok(json_text) => json_text,
                Err(e) => panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded from `https://raw.githubusercontent.com/Pokechu22/Burger/gh-pages/{}.json`. Unfortunately, this file appears to contain invalid data. Error: {}", VERSION, e),
            };

            let mut file = match File::create(format!("target/burger-cache-{}.json", VERSION)) {
                Ok(file) => file,
                Err(e) => panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded and cached to `target/burger-cache-{}.json`. Unfortunately, we can't access this path. Error: {}", VERSION, e),
            };

            if let Err(e) = file.write_all(json_text.as_bytes()) {
                panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded and cached to `target/burger-cache-{}.json`. Unfortunately, we can't write to this path. Error: {}", VERSION, e)
            };

            let json = match serde_json::from_str(json_text) {
                Ok(json) => json,
                Err(e) => panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded and cached to `target/burger-cache-{}.json`. Unfortunately, this file appears to contain invalid json data. Error: {}\nNote: Deleting the file will allow the library to download it again.", VERSION, e),
            };

            json
        }

        // The cache file cannot be accessed
        Err(e) => {
            panic!("The minecraft-format library uses a build script to generate data structures from extracted data. The extracted data is downloaded and cached to `target/burger-cache-{}.json`. Unfortunately, we can't access this path. Error: {}", VERSION, e);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    display_name: Option<String>,
    hardness: Option<f32>,
    max_state_id: u32,
    min_state_id: u32,
    numeric_id: u32,
    resistance: Option<f32>,
    text_id: String,
}

#[allow(clippy::explicit_counter_loop)]
fn generate_block_enum(data: &serde_json::Value) {
    let blocks_json = data
        .get(0)
        .expect("Burger data is not an array")
        .get("blocks")
        .expect("No blocks in burger's json data")
        .get("block")
        .expect("expected block in blocks")
        .as_object()
        .expect("expected block to be an object")
        .clone();

    let mut blocks = Vec::new();
    for (key, block) in blocks_json.into_iter() {
        let block: Block = match serde_json::from_value(block) {
            Ok(block) => block,
            Err(e) => {
                println!("Invalid block: {}, {}", key, e);
                continue;
            }
        };
        blocks.push(block);
    }
    let len = blocks.len();
    blocks.sort_by_key(|b| b.numeric_id);

    let mut expected = 0;
    for block in &blocks {
        if block.numeric_id != expected {
            panic!("The block with id {} is missing.", expected)
        }
        expected += 1;
    }

    let mut display_names = Vec::new();
    let mut hardnesses = Vec::new();
    let mut state_id_ranges = Vec::new();
    let mut numeric_ids = Vec::new();
    let mut resistances = Vec::new();
    let mut text_ids = Vec::new();
    for block in blocks {
        let display_name = match block.display_name {
            Some(display_name) => display_name,
            None => block.text_id.from_case(Case::Snake).to_case(Case::Title),
        };

        display_names.push(display_name);
        hardnesses.push(block.hardness);
        state_id_ranges.push(block.min_state_id..block.max_state_id + 1);
        numeric_ids.push(block.numeric_id);
        resistances.push(block.resistance);
        text_ids.push(block.text_id);
    }

    let mut variants = String::new();
    for i in 0..len {
        let name = text_ids[i].to_case(Case::UpperCamel);
        variants.push_str(&format!("\t{} = {},\n", name, numeric_ids[i]));
    }

    let mut state_id_match_arms = String::new();
    for i in 0..len {
        let name = text_ids[i].to_case(Case::UpperCamel);
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
#[derive(Debug, Clone, Copy)]
pub enum Block {{
{variants}
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

    /// Some hardness values are missing from the dataset so we won't be able to get them
    pub fn get_hardness(self) -> Option<f32> {{
        unsafe {{*HARDNESSES.get_unchecked((self as u32) as usize)}}
    }}

    /// Some resistance values are missing from the dataset so we won't be able to get them
    pub fn get_resistance(self) -> Option<f32> {{
        unsafe {{*RESISTANCES.get_unchecked((self as u32) as usize)}}
    }}

    pub fn get_text_id(self) -> &'static str {{
        unsafe {{*TEXT_IDS.get_unchecked((self as u32) as usize)}}
    }}

    pub fn get_display_name(self) -> &'static str {{
        unsafe {{*DISPLAY_NAMES.get_unchecked((self as u32) as usize)}}
    }}

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

const HARDNESSES: [Option<f32>; {max_value}] = {hardnesses:?};

const RESISTANCES: [Option<f32>; {max_value}] = {resistances:?};

const TEXT_IDS: [&str; {max_value}] = {text_ids:?};

const DISPLAY_NAMES: [&str; {max_value}] = {display_names:?};

const STATE_ID_RANGES: [std::ops::Range<u32>; {max_value}] = {state_id_ranges:?};
"#,
        variants = variants,
        max_value = expected,
        state_id_match_arms = state_id_match_arms,
        hardnesses = hardnesses,
        resistances = resistances,
        text_ids = text_ids,
        display_names = display_names,
        state_id_ranges = state_id_ranges,
    );

    File::create("src/ids/blocks.rs")
        .unwrap()
        .write_all(code.as_bytes())
        .unwrap()
}

fn main() {
    //println!("cargo:rerun-if-changed=target/burger-cache-{}.json", VERSION);
    let data = get_burger_data();
    generate_block_enum(&data);
}
