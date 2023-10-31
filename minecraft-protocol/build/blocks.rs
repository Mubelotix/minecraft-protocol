use super::*;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct BlockState {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    num_values: usize,
    values: Option<Vec<String>>,
}

impl BlockState {
    fn ty(&self, block_name: &str, competing_definitions: bool) -> String {
        match self.ty.as_str() {
            "int" => {
                let values: Vec<i128> = self
                    .values
                    .as_ref()
                    .expect("No values for int block state")
                    .iter()
                    .map(|v| v.parse().expect("Invalid block state value: expected int"))
                    .collect();
                let mut min_value: i128 = *values.first().unwrap_or(&0);
                let mut max_value: i128 = *values.first().unwrap_or(&0);

                for value in values {
                    if value < min_value {
                        min_value = value;
                    }
                    if value > max_value {
                        max_value = value;
                    }
                }

                if min_value >= u8::MIN as i128 && max_value <= u8::MAX as i128 {
                    return String::from("u8");
                }
                if min_value >= i8::MIN as i128 && max_value <= i8::MAX as i128 {
                    return String::from("i8");
                }
                if min_value >= u16::MIN as i128 && max_value <= u16::MAX as i128 {
                    return String::from("u16");
                }
                if min_value >= i16::MIN as i128 && max_value <= i16::MAX as i128 {
                    return String::from("i16");
                }
                if min_value >= u32::MIN as i128 && max_value <= u32::MAX as i128 {
                    return String::from("u32");
                }
                if min_value >= i32::MIN as i128 && max_value <= i32::MAX as i128 {
                    return String::from("i32");
                }
                if min_value >= u64::MIN as i128 && max_value <= u64::MAX as i128 {
                    return String::from("u64");
                }
                if min_value >= i64::MIN as i128 && max_value <= i64::MAX as i128 {
                    return String::from("i64");
                }
                String::from("i128")
            }
            "enum" => match competing_definitions {
                true => format!("{}_{}", block_name, self.name),
                false => self.name.to_string(),
            }
            .from_case(Case::Snake)
            .to_case(Case::UpperCamel),
            "bool" => String::from("bool"),
            _ => unimplemented!(),
        }
    }

    fn define_enum(&self, block_name: &str, competing_definitions: bool) -> String {
        if self.ty.as_str() != "enum" {
            panic!("Called defined enum on non-enum");
        }

        let mut variants = String::new();
        for (i, value) in self
            .values
            .as_ref()
            .expect("Expecting values in enum (state id)")
            .iter()
            .enumerate()
        {
            variants.push_str(&format!(
                "\n\t{} = {},",
                value.from_case(Case::Snake).to_case(Case::UpperCamel),
                i
            ));
        }

        format!(
            r#"#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[cfg_attr(test, derive(PartialEq))]
pub enum {} {{{}
}}"#,
            self.ty(block_name, competing_definitions),
            variants
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "camelCase")]
struct Block {
    id: u32,
    #[serde(rename = "name")]
    text_id: String,
    display_name: String,
    hardness: f32,
    resistance: f32,
    diggable: bool,
    transparent: bool,
    filter_light: u8,
    emit_light: u8,
    default_state: u32,
    min_state_id: u32,
    max_state_id: u32,
    drops: Vec<u32>,
    material: Option<String>,
    #[serde(default)]
    harvest_tools: HashMap<u32, bool>,
    states: Vec<BlockState>,
}

#[allow(clippy::explicit_counter_loop)]
pub fn generate_block_enum(data: serde_json::Value) {
    let mut blocks: Vec<Block> = serde_json::from_value(data).expect("Invalid block data");
    blocks.sort_by_key(|block| block.id);

    // Look for missing blocks in the array
    let mut expected = 0;
    for block in &blocks {
        if block.id != expected {
            panic!("The block with id {} is missing.", expected)
        }
        expected += 1;
    }

    // Process a few fields
    let mut raw_harvest_tools: Vec<Vec<u32>> = Vec::new();
    let mut raw_materials: Vec<String> = Vec::new();
    for block in &blocks {
        raw_harvest_tools.push(
            block
                .harvest_tools
                .clone()
                .into_iter()
                .map(|(k, _v)| k)
                .collect(),
        );
        let mut material = block
            .material
            .clone()
            .unwrap_or_else(|| "unknown_material".to_string())
            .split(';')
            .next()
            .unwrap()
            .to_string();
        if material.starts_with("mineable") {
            material = "unknown_material".to_string();
        }
        raw_materials.push(material.from_case(Case::Snake).to_case(Case::UpperCamel));
    }

    // Generate the MaterialBlock enum and array
    let mut different_materials = raw_materials.clone();
    different_materials.sort();
    different_materials.dedup();
    let mut material_variants = String::new();
    for material in different_materials {
        material_variants.push_str(&format!("\t{},\n", material));
    }
    let mut materials = String::new();
    materials.push('[');
    for material in raw_materials {
        materials.push_str("Some(BlockMaterial::");
        materials.push_str(&material);
        materials.push_str("), ");
    }
    materials.push(']');

    // Generate the HARVEST_TOOLS array
    let mut harvest_tools = String::new();
    harvest_tools.push('[');
    for block_harvest_tools in raw_harvest_tools {
        harvest_tools.push_str("&[");
        for harvest_tool in block_harvest_tools {
            harvest_tools.push_str(&harvest_tool.to_string());
            harvest_tools.push_str(", ");
        }
        harvest_tools.push_str("], ");
    }
    harvest_tools.push(']');

    // Enumerate the air blocks
    let mut air_blocks = vec![false; expected as usize];
    for air_block in &[
        "air",
        "cave_air",
        "grass",
        "torch",
        "wall_torch",
        "wheat",
        "soul_torch",
        "soul_wall_torch",
        "carrots",
        "potatoes",
    ] {
        let mut success = false;
        for block in &blocks {
            if &block.text_id.as_str() == air_block {
                air_blocks[block.id as usize] = true;
                success = true;
                break;
            }
        }
        if !success {
            panic!("Could not find block {} in the block array", air_block);
        }
    }

    // Generate the variants of the Block enum
    let mut variants = String::new();
    for block in &blocks {
        let name = block
            .text_id
            .from_case(Case::Snake)
            .to_case(Case::UpperCamel);
        variants.push_str(&format!("\t{} = {},\n", name, block.id));
    }

    // Generate the `match` of state ids
    let mut state_id_match_arms = String::new();
    for block in &blocks {
        let name = block
            .text_id
            .from_case(Case::Snake)
            .to_case(Case::UpperCamel);
        let start = block.min_state_id;
        let stop = block.max_state_id;
        if start != stop {
            state_id_match_arms.push_str(&format!(
                "\t\t\t{}..={} => Some(Block::{}),\n",
                start, stop, name
            ));
        } else {
            state_id_match_arms.push_str(&format!("\t\t\t{} => Some(Block::{}),\n", start, name));
        }
    }

    // Generate the code
    let code = format!(
        r#"use crate::*;

/// See [implementations](#implementations) for useful methods.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Block {{
{variants}
}}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockMaterial {{
{material_variants}
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
    #[inline]
    pub fn text_id(self) -> &'static str {{
        unsafe {{*TEXT_IDS.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn default_state_id(self) -> u32 {{
        unsafe {{*DEFAULT_STATE_IDS.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn id(self) -> u32 {{
        self as u32
    }}

    /// This returns the item that will be dropped if you break the block.
    /// If the item is Air, there is actually no drop.
    #[inline]
    pub fn associated_item_id(self) -> u32 {{
        unsafe {{*ITEM_IDS.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn resistance(self) -> f32 {{
        unsafe {{*RESISTANCES.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn hardness(self) -> f32 {{
        unsafe {{*HARDNESSES.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn material(self) -> Option<BlockMaterial> {{
        unsafe {{*MATERIALS.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn display_name(self) -> &'static str {{
        unsafe {{*DISPLAY_NAMES.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn state_id_range(self) -> std::ops::Range<u32> {{
        unsafe {{STATE_ID_RANGES.get_unchecked((self as u32) as usize).clone()}}
    }}

    #[inline]
    pub fn is_diggable(self) -> bool {{
        unsafe {{*DIGGABLE.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn is_transparent(self) -> bool {{
        unsafe {{*TRANSPARENT.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn compatible_harvest_tools(self) -> &'static [u32] {{
        unsafe {{*HARVEST_TOOLS.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn light_emissions(self) -> u8 {{
        unsafe {{*LIGHT_EMISSIONS.get_unchecked((self as u32) as usize)}}
    }}

    #[inline]
    pub fn light_absorption(self) -> u8 {{
        unsafe {{*LIGHT_ABSORPTION.get_unchecked((self as u32) as usize)}}
    }}

    /// A "air block" is a block on which a player cannot stand, like air, wheat, torch...
    /// Fire is excluded since you may not want your clients to walk trought fire by default.
    /// The list of air blocks is maintained by hand.
    /// It could not be exhaustive.
    /// See also [Block::is_blocking].
    #[inline]
    pub fn is_air_block(self) -> bool {{
        unsafe {{*AIR_BLOCKS.get_unchecked((self as u32) as usize)}}
    }}

    /// The opposite of [Block::is_air_block].
    /// Fire is included since you may not want your clients to walk trought fire by default.
    /// The list of blocking blocks is maintained by hand.
    /// It could not be exhaustive.
    #[inline]
    pub fn is_blocking(self) -> bool {{
        unsafe {{!(*AIR_BLOCKS.get_unchecked((self as u32) as usize))}}
    }}
}}

impl From<super::block_states::BlockWithState> for Block {{
    #[inline]
    fn from(block_with_state: super::block_states::BlockWithState) -> Block {{
        unsafe {{std::mem::transmute(block_with_state.block_id())}}
    }}
}}

impl<'a> MinecraftPacketPart<'a> for Block {{
    #[inline]
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {{
        VarInt((self as u32) as i32).serialize_minecraft_packet_part(output)
    }}

    #[inline]
    fn deserialize_minecraft_packet_part(input: &'a[u8]) -> Result<(Self, &'a[u8]), &'static str> {{
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
const RESISTANCES: [f32; {max_value}] = {resistances:?};
const MATERIALS: [Option<BlockMaterial>; {max_value}] = {materials};
const HARVEST_TOOLS: [&[u32]; {max_value}] = {harvest_tools};
const HARDNESSES: [f32; {max_value}] = {hardnesses:?};
const LIGHT_EMISSIONS: [u8; {max_value}] = {light_emissions:?};
const LIGHT_ABSORPTION: [u8; {max_value}] = {light_absorption:?};
const DIGGABLE: [bool; {max_value}] = {diggable:?};
const TRANSPARENT: [bool; {max_value}] = {transparent:?};
const AIR_BLOCKS: [bool; {max_value}] = {air_blocks:?};
"#,
        variants = variants,
        material_variants = material_variants,
        max_value = expected,
        state_id_match_arms = state_id_match_arms,
        text_ids = blocks.iter().map(|b| &b.text_id).collect::<Vec<_>>(),
        display_names = blocks.iter().map(|b| &b.display_name).collect::<Vec<_>>(),
        state_id_ranges = blocks
            .iter()
            .map(|b| b.min_state_id..b.max_state_id + 1)
            .collect::<Vec<_>>(),
        default_state_ids = blocks.iter().map(|b| b.default_state).collect::<Vec<_>>(),
        item_ids = blocks
            .iter()
            .map(|b| b.drops.get(0).copied().unwrap_or(0))
            .collect::<Vec<_>>(),
        materials = materials,
        resistances = blocks.iter().map(|b| b.resistance).collect::<Vec<_>>(),
        harvest_tools = harvest_tools,
        hardnesses = blocks.iter().map(|b| b.hardness).collect::<Vec<_>>(),
        light_emissions = blocks.iter().map(|b| b.emit_light).collect::<Vec<_>>(),
        light_absorption = blocks.iter().map(|b| b.filter_light).collect::<Vec<_>>(),
        diggable = blocks.iter().map(|b| b.diggable).collect::<Vec<_>>(),
        transparent = blocks.iter().map(|b| b.transparent).collect::<Vec<_>>(),
        air_blocks = air_blocks,
    );

    File::create("src/ids/blocks.rs")
        .unwrap()
        .write_all(code.as_bytes())
        .unwrap()
}

#[allow(clippy::explicit_counter_loop)]
pub fn generate_block_with_state_enum(data: serde_json::Value) {
    let mut blocks: Vec<Block> = serde_json::from_value(data).expect("Invalid block data");
    blocks.sort_by_key(|block| block.min_state_id);

    // Look for missing blocks in the array
    let mut expected = 0;
    for block in &blocks {
        if block.id != expected {
            panic!("The block with id {} is missing.", expected)
        }
        expected += 1;
    }

    // Generate the enum definitions
    let mut enum_definitions = Vec::new();
    let mut enum_definitions_string = String::new();
    let mut already_defined_enums = Vec::new();
    for block in &blocks {
        for state in &block.states {
            if state.ty.as_str() == "enum" {
                enum_definitions.push((&block.text_id, state));
            }
        }
    }
    for (block_name, enum_definition) in &enum_definitions {
        let mut competing_definitions = false;
        for (_, enum_definition2) in &enum_definitions {
            if enum_definition.name == enum_definition2.name
                && enum_definition.values != enum_definition2.values
            {
                competing_definitions = true;
                break;
            }
        }
        if !already_defined_enums.contains(&enum_definition.ty(block_name, competing_definitions)) {
            enum_definitions_string
                .push_str(&enum_definition.define_enum(block_name, competing_definitions));
            enum_definitions_string.push('\n');
            enum_definitions_string.push('\n');

            already_defined_enums.push(enum_definition.ty(block_name, competing_definitions));
        }
    }

    // Generate the variants of the Block enum
    let mut variants = String::new();
    for block in &blocks {
        let name = block
            .text_id
            .from_case(Case::Snake)
            .to_case(Case::UpperCamel);
        let mut fields = String::new();
        for state in &block.states {
            let name = match state.name.as_str() == "type" {
                true => "ty",
                false => state.name.as_str(),
            };
            let competing_definitions =
                already_defined_enums.contains(&state.ty(&block.text_id, true));
            let doc = if state.ty == "int" {
                let values: Vec<i128> = state
                    .values
                    .as_ref()
                    .expect("No values for int block state")
                    .iter()
                    .map(|v| v.parse().expect("Invalid block state value: expected int"))
                    .collect();

                let mut expected = values[0];
                let mut standard = true;
                for value in &values {
                    if value != &expected {
                        standard = false;
                        break;
                    }
                    expected += 1;
                }

                match standard {
                    true => format!("\t\t/// Valid if {} <= {} <= {}\n", values[0], name, values.last().unwrap()),
                    false => format!("\t\t/// Valid if {} ∈ {:?}\n", name, values),
                }
            } else {
                String::new()
            };
            fields.push_str(&format!(
                "{}\t\t{}: {},\n",
                doc,
                name,
                state.ty(&block.text_id, competing_definitions)
            ));
        }
        if fields.is_empty() {
            variants.push_str(&format!("\t{},\n", name));
        } else {
            variants.push_str(&format!("\t{} {{\n{}\t}},\n", name, fields));
        }
    }

    // Generate the `match` of state ids
    let mut state_id_match_arms = String::new();
    let mut state_id_rebuild_arms = String::new();
    for block in &blocks {
        let name = block
            .text_id
            .from_case(Case::Snake)
            .to_case(Case::UpperCamel);
        let start = block.min_state_id;
        let stop = block.max_state_id;

        if block.states.is_empty() {
            state_id_match_arms.push_str(&format!(
                "\n\t\t\t{} => Some(BlockWithState::{}),",
                start, name
            ));
            state_id_rebuild_arms.push_str(&format!(
                "\n\t\t\tBlockWithState::{} => Some({}),",
                name, start
            ));
            continue;
        }

        let mut state_calculations = String::new();
        let mut fields = String::new();
        for (i, state) in block.states.iter().enumerate().rev() {
            let competing_definitions =
                already_defined_enums.contains(&state.ty(&block.text_id, true));
            let ty = state.ty(&block.text_id, competing_definitions);
            let name = match state.name.as_str() {
                "type" => "ty",
                _ => &state.name,
            };
            fields.push_str(&format!("{}, ", name));

            if i == 0 {
                state_calculations.push_str("\n\t\t\t\tlet field_value = state_id;");
            } else {
                state_calculations.push_str(&format!(
                    "\n\t\t\t\tlet field_value = state_id.rem_euclid({});\
                        \n\t\t\t\tstate_id -= field_value;\
                        \n\t\t\t\tstate_id /= {};",
                    state.num_values, state.num_values
                ));
            }

            match state.ty.as_str() {
                "enum" => {
                    state_calculations.push_str(&format!(
                            "\n\t\t\t\tlet {}: {} = unsafe{{std::mem::transmute(field_value as u8)}};\n",
                            name, ty
                        ));
                }
                "int" => {
                    let values: Vec<i128> = state
                        .values
                        .as_ref()
                        .expect("No values for int block state")
                        .iter()
                        .map(|v| v.parse().expect("Invalid block state value: expected int"))
                        .collect();

                    let mut expected = values[0];
                    let mut standard = true;
                    for value in &values {
                        if value != &expected {
                            standard = false;
                            break;
                        }
                        expected += 1;
                    }

                    if standard && values[0] == 0 {
                        state_calculations.push_str(&format!(
                            "\n\t\t\t\tlet {}: {} = field_value as {};\n",
                            name, ty, ty
                        ));
                    } else if standard {
                        state_calculations.push_str(&format!(
                            "\n\t\t\t\tlet {}: {} = {} + field_value as {};\n",
                            name, ty, values[0], ty
                        ));
                    } else {
                        state_calculations.push_str(&format!(
                            "\n\t\t\t\tlet {}: {} = {:?}[field_value as usize];\n",
                            name, ty, values
                        ));
                    }
                }
                "bool" => {
                    state_calculations.push_str(&format!(
                        "\n\t\t\t\tlet {}: bool = field_value == 0;\n",
                        name
                    ));
                }
                other => panic!("Unknown {} type", other),
            }
        }

        let mut state_reformation = String::new();
        for (i, state) in block.states.iter().enumerate() {
            let name = match state.name.as_str() {
                "type" => "ty",
                _ => &state.name,
            };

            match state.ty.as_str() {
                "enum" => {
                    state_reformation.push_str(&format!(
                        "\n\t\t\t\tlet field_value = (*{} as u8) as u32;",
                        name
                    ));
                }
                "int" => {
                    let values: Vec<i128> = state
                        .values
                        .as_ref()
                        .expect("No values for int block state")
                        .iter()
                        .map(|v| v.parse().expect("Invalid block state value: expected int"))
                        .collect();

                    let mut expected = values[0];
                    let mut standard = true;
                    for value in &values {
                        if value != &expected {
                            standard = false;
                            break;
                        }
                        expected += 1;
                    }

                    if standard && values[0] == 0 {
                        state_reformation.push_str(&format!(
                            "\n\t\t\t\tif *{name} > {max} {{ return None }}\
                                \n\t\t\t\tlet field_value = *{name} as u32;",
                            name = name,
                            max = values.last().unwrap()
                        ));
                    } else if standard {
                        state_reformation.push_str(&format!(
                            "\n\t\t\t\tif *{name} < {min} || *{name} > {max} {{ return None }}\
                                \n\t\t\t\tlet field_value = ({name} - {min}) as u32;",
                            name = name,
                            min = values[0],
                            max = values.last().unwrap()
                        ));
                    } else {
                        state_reformation.push_str(&format!(
                            "\n\t\t\t\tlet field_value = {:?}.find({})?;",
                            values, name
                        ));
                    }
                }
                "bool" => {
                    state_reformation.push_str(&format!(
                        "\n\t\t\t\tlet field_value = if *{} {{0}} else {{1}};",
                        name
                    ));
                }
                other => panic!("Unknown {} type", other),
            }

            if i == 0 {
                state_reformation.push_str("\n\t\t\t\tlet mut state_id = field_value;\n");
            } else {
                state_reformation.push_str(&format!(
                    "\n\t\t\t\tstate_id *= {};\
                        \n\t\t\t\tstate_id += field_value;\n",
                    state.num_values
                ));
            }
        }

        state_id_match_arms.push_str(&format!(
            "
            {}..={} => {{
                state_id -= {};
                {}
                Some(BlockWithState::{}{{ {}}})
            }},",
            start, stop, start, state_calculations, name, fields
        ));
        state_id_rebuild_arms.push_str(&format!(
            "
            BlockWithState::{}{{ {}}} => {{
                {}
                state_id += {};
                Some(state_id)
            }},",
            name, fields, state_reformation, start
        ));
    }

    // Generate the code
    let code = format!(
        r#"//! Contains the [BlockWithState] enum to help with block state IDs.
            
use crate::*;

{enum_definitions}

/// Can be converted for free to [super::blocks::Block] which implements [useful methods](super::blocks::Block#implementations).
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
#[repr(u32)]
pub enum BlockWithState {{
{variants}
}}

impl BlockWithState {{
    #[inline]
    pub fn from_state_id(mut state_id: u32) -> Option<BlockWithState> {{
        match state_id {{
{state_id_match_arms}
            _ => None,
        }}
    }}

    /// Returns the block id, **not the block state id**.
    #[inline]
    pub fn block_id(&self) -> u32 {{
        unsafe {{std::mem::transmute(std::mem::discriminant(self))}}
    }}

    /// Returns the block state id.
    /// Returns None in case of error (invalid field value).
    #[inline]
    pub fn block_state_id(&self) -> Option<u32> {{
        match self {{
{state_id_rebuild_arms}
        }}
    }}
}}

impl From<super::blocks::Block> for BlockWithState {{
    #[inline]
    fn from(block: super::blocks::Block) -> BlockWithState {{
        BlockWithState::from_state_id(block.default_state_id()).unwrap() // TODO: unwrap unchecked
    }}
}}

impl<'a> MinecraftPacketPart<'a> for BlockWithState {{
    #[inline]
    fn serialize_minecraft_packet_part(self, _output: &mut Vec<u8>) -> Result<(), &'static str> {{
        VarInt::from(self.block_state_id().unwrap_or(0)).serialize_minecraft_packet_part(_output)
    }}

    #[inline]
    fn deserialize_minecraft_packet_part(input: &'a[u8]) -> Result<(Self, &'a[u8]), &'static str> {{
        let (id, input) = VarInt::deserialize_minecraft_packet_part(input)?;
        let id = std::cmp::max(id.0, 0) as u32;
        let block_with_state = BlockWithState::from_state_id(id).ok_or("No block corresponding to the specified block state ID.")?;
        Ok((block_with_state, input))
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_block_states() {{
        for id in 0..={max_block_state_id} {{
            let block = BlockWithState::from_state_id(id).unwrap();
            let id_from_block = block.block_state_id().unwrap();
            assert_eq!(id, id_from_block);
        }}
    }}
}}
"#,
        enum_definitions = enum_definitions_string,
        state_id_match_arms = state_id_match_arms,
        state_id_rebuild_arms = state_id_rebuild_arms,
        variants = variants,
        max_block_state_id = blocks.last().unwrap().max_state_id
    );

    File::create("src/ids/block_states.rs")
        .unwrap()
        .write_all(code.as_bytes())
        .unwrap()
}
