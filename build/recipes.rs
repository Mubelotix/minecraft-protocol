use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
enum CountedItem {
    IDAndMetadataAndCount { id: u32, metadata: u32, count: u8 },
    IDAndMetadata { id: u32, metadata: u32 },
    IDAndCount { id: u32, count: u8 },
    ID(u32),
}

impl CountedItem {
    fn to_id_and_count(&self) -> (u32, u8) {
        match self {
            CountedItem::IDAndMetadataAndCount { .. } => panic!("Metadata not handled"),
            CountedItem::IDAndMetadata { .. } => panic!("Metadata not handled"),
            CountedItem::IDAndCount { id, count } => (*id, *count),
            CountedItem::ID(id) => (*id, 1),
        }
    }

    fn format(&self, items: &[super::items::Item]) -> String {
        let (id, count) = self.to_id_and_count();
        let item_ident = item_id_to_item(id, items);
        format!(
            "CountedItem {{item: Item::{}, count: {}}}",
            item_ident, count
        )
    }

    fn format_count1(&self, items: &[super::items::Item]) -> String {
        let (id, count) = self.to_id_and_count();
        assert!(count == 1);
        let item_ident = item_id_to_item(id, items);
        format!(
            "Item::{}",
            item_ident
        )
    }
}

#[allow(dead_code)]
fn format_option_item(item: &Option<CountedItem>, items: &[super::items::Item]) -> String {
    match item {
        Some(item) => format!("Some({})", item.format(items)),
        None => "None".to_string(),
    }
}

fn format_option_item_count1(item: &Option<CountedItem>, items: &[super::items::Item]) -> String {
    match item {
        Some(item) => format!("Some({})", item.format_count1(items)),
        None => "None".to_string(),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Shape {
    ThreeByThree([[Option<CountedItem>; 3]; 3]),
    ThreeByTwo([[Option<CountedItem>; 3]; 2]),
    ThreeByOne([[Option<CountedItem>; 3]; 1]),
    TwoByThree([[Option<CountedItem>; 2]; 3]),
    TwoByTwo([[Option<CountedItem>; 2]; 2]),
    TwoByOne([[Option<CountedItem>; 2]; 1]),
    OneByThree([[Option<CountedItem>; 1]; 3]),
    OneByTwo([[Option<CountedItem>; 1]; 2]),
    OneByOne([[Option<CountedItem>; 1]; 1]),
}

impl Shape {
    #[allow(dead_code)]
    fn format(&self, i: &[super::items::Item]) -> String {
        match self {
            Shape::ThreeByThree([[v1, v2, v3], [v4, v5, v6], [v7, v8, v9]]) => {
                format!(
                    "Shape::ThreeByThree([[{}, {}, {}], [{}, {}, {}], [{}, {}, {}]])",
                    format_option_item(v1, i),
                    format_option_item(v2, i),
                    format_option_item(v3, i),
                    format_option_item(v4, i),
                    format_option_item(v5, i),
                    format_option_item(v6, i),
                    format_option_item(v7, i),
                    format_option_item(v8, i),
                    format_option_item(v9, i)
                )
            }
            Shape::ThreeByTwo([[v1, v2, v3], [v4, v5, v6]]) => {
                format!(
                    "Shape::ThreeByTwo([[{}, {}, {}], [{}, {}, {}]])",
                    format_option_item(v1, i),
                    format_option_item(v2, i),
                    format_option_item(v3, i),
                    format_option_item(v4, i),
                    format_option_item(v5, i),
                    format_option_item(v6, i)
                )
            }
            Shape::ThreeByOne([[v1, v2, v3]]) => {
                format!(
                    "Shape::ThreeByOne([[{}, {}, {}]])",
                    format_option_item(v1, i),
                    format_option_item(v2, i),
                    format_option_item(v3, i)
                )
            }
            Shape::TwoByThree([[v1, v2], [v3, v4], [v5, v6]]) => {
                format!(
                    "Shape::TwoByThree([[{}, {}], [{}, {}], [{}, {}]])",
                    format_option_item(v1, i),
                    format_option_item(v2, i),
                    format_option_item(v3, i),
                    format_option_item(v4, i),
                    format_option_item(v5, i),
                    format_option_item(v6, i)
                )
            }
            Shape::TwoByTwo([[v1, v2], [v3, v4]]) => {
                format!(
                    "Shape::TwoByTwo([[{}, {}], [{}, {}]])",
                    format_option_item(v1, i),
                    format_option_item(v2, i),
                    format_option_item(v3, i),
                    format_option_item(v4, i)
                )
            }
            Shape::TwoByOne([[v1, v2]]) => {
                format!(
                    "Shape::TwoByOne([[{}, {}]])",
                    format_option_item(v1, i),
                    format_option_item(v2, i)
                )
            }
            Shape::OneByThree([[v1], [v2], [v3]]) => {
                format!(
                    "Shape::OneByThree([[{}], [{}], [{}]])",
                    format_option_item(v1, i),
                    format_option_item(v2, i),
                    format_option_item(v3, i)
                )
            }
            Shape::OneByTwo([[v1], [v2]]) => {
                format!(
                    "Shape::OneByTwo([[{}], [{}]])",
                    format_option_item(v1, i),
                    format_option_item(v2, i)
                )
            }
            Shape::OneByOne([[v1]]) => {
                format!("Shape::OneByOne([[{}]])", format_option_item(v1, i))
            }
        }
    }

    fn format_count1(&self, i: &[super::items::Item]) -> String {
        match self {
            Shape::ThreeByThree([[v1, v2, v3], [v4, v5, v6], [v7, v8, v9]]) => {
                format!(
                    "Shape::ThreeByThree([[{}, {}, {}], [{}, {}, {}], [{}, {}, {}]])",
                    format_option_item_count1(v1, i),
                    format_option_item_count1(v2, i),
                    format_option_item_count1(v3, i),
                    format_option_item_count1(v4, i),
                    format_option_item_count1(v5, i),
                    format_option_item_count1(v6, i),
                    format_option_item_count1(v7, i),
                    format_option_item_count1(v8, i),
                    format_option_item_count1(v9, i)
                )
            }
            Shape::ThreeByTwo([[v1, v2, v3], [v4, v5, v6]]) => {
                format!(
                    "Shape::ThreeByTwo([[{}, {}, {}], [{}, {}, {}]])",
                    format_option_item_count1(v1, i),
                    format_option_item_count1(v2, i),
                    format_option_item_count1(v3, i),
                    format_option_item_count1(v4, i),
                    format_option_item_count1(v5, i),
                    format_option_item_count1(v6, i)
                )
            }
            Shape::ThreeByOne([[v1, v2, v3]]) => {
                format!(
                    "Shape::ThreeByOne([[{}, {}, {}]])",
                    format_option_item_count1(v1, i),
                    format_option_item_count1(v2, i),
                    format_option_item_count1(v3, i)
                )
            }
            Shape::TwoByThree([[v1, v2], [v3, v4], [v5, v6]]) => {
                format!(
                    "Shape::TwoByThree([[{}, {}], [{}, {}], [{}, {}]])",
                    format_option_item_count1(v1, i),
                    format_option_item_count1(v2, i),
                    format_option_item_count1(v3, i),
                    format_option_item_count1(v4, i),
                    format_option_item_count1(v5, i),
                    format_option_item_count1(v6, i)
                )
            }
            Shape::TwoByTwo([[v1, v2], [v3, v4]]) => {
                format!(
                    "Shape::TwoByTwo([[{}, {}], [{}, {}]])",
                    format_option_item_count1(v1, i),
                    format_option_item_count1(v2, i),
                    format_option_item_count1(v3, i),
                    format_option_item_count1(v4, i)
                )
            }
            Shape::TwoByOne([[v1, v2]]) => {
                format!(
                    "Shape::TwoByOne([[{}, {}]])",
                    format_option_item_count1(v1, i),
                    format_option_item_count1(v2, i)
                )
            }
            Shape::OneByThree([[v1], [v2], [v3]]) => {
                format!(
                    "Shape::OneByThree([[{}], [{}], [{}]])",
                    format_option_item_count1(v1, i),
                    format_option_item_count1(v2, i),
                    format_option_item_count1(v3, i)
                )
            }
            Shape::OneByTwo([[v1], [v2]]) => {
                format!(
                    "Shape::OneByTwo([[{}], [{}]])",
                    format_option_item_count1(v1, i),
                    format_option_item_count1(v2, i)
                )
            }
            Shape::OneByOne([[v1]]) => {
                format!("Shape::OneByOne([[{}]])", format_option_item_count1(v1, i))
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Recipe {
    #[serde(rename_all = "camelCase")]
    DoubleShaped {
        result: CountedItem,
        in_shape: Shape,
        out_shape: Shape,
    },
    #[serde(rename_all = "camelCase")]
    Shaped { in_shape: Shape, result: CountedItem },
    #[serde(rename_all = "camelCase")]
    ShapeLess {
        result: CountedItem,
        ingredients: Vec<CountedItem>,
    },
}

fn item_id_to_item(id: u32, items: &[super::items::Item]) -> String {
    for item in items {
        if item.id == id {
            return item
                .text_id
                .from_case(Case::Snake)
                .to_case(Case::UpperCamel);
        }
    }

    panic!("Item ID from recipe not found")
}

pub fn generate_recipes(data: serde_json::Value, items: Vec<super::items::Item>) {
    let mut item_recipes: HashMap<u32, Vec<Recipe>> =
        serde_json::from_value(data).expect("Invalid recipes");

    // Count recipes
    let mut recipes_count = 0;
    for recipes in item_recipes.values_mut() {
        let recipes_len = recipes.len();
        recipes.retain(|recipe| !matches!(recipe, Recipe::DoubleShaped{..}));
        if recipes.len() != recipes_len {
            println!("Contains a double shaped recipe, which support has been removed as an optimization. It needs to be enabled again if required by future minecraft updates.");
        }
        recipes_count += recipes.len();
    }

    // Generate recipes
    let mut recipes_data = String::new();
    for recipes in item_recipes.values() {
        for recipe in recipes {
            match recipe {
                Recipe::ShapeLess {
                    result,
                    ingredients,
                } => {
                    let mut ingredients_string = String::new();
                    for ingredient in ingredients {
                        ingredients_string.push_str(&ingredient.format_count1(&items));
                        ingredients_string.push_str(", ");
                    }

                    recipes_data.push_str(&format!(
                        "\tRecipe::ShapeLess {{ result: {}, ingredients: &[{}] }},\n",
                        result.format(&items),
                        ingredients_string,
                    ));
                }
                Recipe::Shaped { result, in_shape } => {
                    recipes_data.push_str(&format!(
                        "\tRecipe::Shaped {{ result: {}, in_shape: {} }},\n",
                        result.format(&items),
                        in_shape.format_count1(&items),
                    ));
                }
                Recipe::DoubleShaped {
                    result,
                    in_shape,
                    out_shape,
                } => {
                    recipes_data.push_str(&format!(
                        "\tRecipe::DoubleShaped {{ result: {}, in_shape: {}, out_shape: {} }},\n",
                        result.format(&items),
                        in_shape.format_count1(&items),
                        out_shape.format_count1(&items),
                    ));
                }
            }
        }
    }

    // Generate shortcuts
    let mut idx_in_array = 0;
    let mut shortcuts = Vec::new();
    for item_id in 0..items.len() {
        let vec_default = Vec::new();
        let recipes = item_recipes.get(&(item_id as u32)).unwrap_or(&vec_default);
        shortcuts.push((idx_in_array, idx_in_array + recipes.len()));
        idx_in_array += recipes.len();
    }

    #[allow(clippy::useless_format)]
    let code = format!(
        r#"//! All crafting recipes

use crate::ids::items::Item;

/// An [Item](crate::ids::items::Item) associated with a count of this item
#[derive(Debug, Clone)]
pub struct CountedItem {{
    pub item: Item,
    pub count: u8,
}}

#[derive(Debug, Clone)]
pub enum Shape {{
    ThreeByThree([[Option<Item>; 3]; 3]),
    ThreeByTwo([[Option<Item>; 3]; 2]),
    ThreeByOne([[Option<Item>; 3]; 1]),
    TwoByThree([[Option<Item>; 2]; 3]),
    TwoByTwo([[Option<Item>; 2]; 2]),
    TwoByOne([[Option<Item>; 2]; 1]),
    OneByThree([[Option<Item>; 1]; 3]),
    OneByTwo([[Option<Item>; 1]; 2]),
    OneByOne([[Option<Item>; 1]; 1]),
}}

impl Shape {{
    /// Returns the size of the shape.
    /// (width, height)
    pub const fn size(&self) -> (u8, u8) {{
        match self {{
            Shape::ThreeByThree(_) => (3, 3),
            Shape::ThreeByTwo(_) => (3, 2),
            Shape::ThreeByOne(_) => (3, 1),
            Shape::TwoByThree(_) => (2, 3),
            Shape::TwoByTwo(_) => (2, 2),
            Shape::TwoByOne(_) => (2, 1),
            Shape::OneByThree(_) => (1, 3),
            Shape::OneByTwo(_) => (1, 2),
            Shape::OneByOne(_) => (1, 1),
        }}
    }}
}}

#[derive(Debug, Clone)]
pub enum Recipe {{
    Shaped {{ in_shape: Shape, result: CountedItem }},
    ShapeLess {{ ingredients: &'static [Item], result: CountedItem }},
}}

impl Recipe {{
    /// Returns all the recipes for an item
    #[inline]
    pub fn get_recipes_for_item(item: Item) -> &'static [Recipe] {{
        unsafe {{
            let (start, end) = SHORTCUTS.get_unchecked((item as u32) as usize);
            RECIPES.get_unchecked(*start..*end)
        }}
    }}

    #[inline]
    pub const fn result(&self) -> &CountedItem {{
        match self {{
            Recipe::Shaped {{ result, .. }} => result,
            Recipe::ShapeLess {{ result, .. }} => result,
        }}
    }}

    #[inline]
    pub const fn in_shape(&self) -> Option<&Shape> {{
        match self {{
            Recipe::Shaped {{ in_shape, .. }} => Some(in_shape),
            Recipe::ShapeLess {{ .. }} => None,
        }}
    }}

    #[inline]
    pub const fn ingredients(&self) -> Option<&'static [Item]> {{
        match self {{
            Recipe::Shaped {{ .. }} => None,
            Recipe::ShapeLess {{ ingredients, .. }} => Some(ingredients),
        }}
    }}
}}

const RECIPES: [Recipe; {recipes_count}] = [
{recipes_data}
];

const SHORTCUTS: [(usize, usize); {item_count}] = {shortcuts:?};
"#,
        recipes_count = recipes_count,
        recipes_data = recipes_data,
        item_count = items.len(),
        shortcuts = shortcuts,
    );

    File::create("src/ids/recipes.rs")
        .unwrap()
        .write_all(code.as_bytes())
        .unwrap()
}
