mod blocks;
mod entities;
mod items;
mod recipes;

use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::io::{ErrorKind, Read, Write};
use std::{collections::HashMap, fs::File};

const VERSION: &str = "1.20.1";

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

fn main() {
    println!(
        "cargo:rerun-if-changed=target/cache-file-location-{}.json",
        VERSION
    );
    println!(
        "cargo:rerun-if-changed=build"
    );

    let mut file_locations = get_data(
        "https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/dataPaths.json",
        &format!("target/cache-file-location-{}.json", VERSION),
    );
    let file_locations = file_locations.get_mut("pc").unwrap().take();
    let file_locations: HashMap<String, HashMap<String, String>> =
        serde_json::from_value(file_locations).unwrap();
    let file_locations = file_locations
        .get(VERSION)
        .expect("There is no generated data for this minecraft version yet");

    let blocks_url = format!(
        "https://github.com/PrismarineJS/minecraft-data/raw/master/data/{}/blocks.json",
        file_locations.get("blocks").unwrap()
    );
    let block_data = get_data(
        &blocks_url,
        &format!("target/cache-blocks-{}.json", VERSION),
    );
    blocks::generate_block_enum(block_data.clone());
    blocks::generate_block_with_state_enum(block_data);

    let items_url = format!(
        "https://github.com/PrismarineJS/minecraft-data/raw/master/data/{}/items.json",
        file_locations.get("items").unwrap()
    );
    let items_data = get_data(&items_url, &format!("target/cache-items-{}.json", VERSION));
    let items = items::generate_item_enum(items_data);

    let entities_url = format!(
        "https://github.com/PrismarineJS/minecraft-data/raw/master/data/{}/entities.json",
        file_locations.get("entities").unwrap()
    );
    let entities_data = get_data(
        &entities_url,
        &format!("target/cache-entities-{}.json", VERSION),
    );
    entities::generate_entity_enum(entities_data);

    let recipes_url = format!(
        "https://github.com/PrismarineJS/minecraft-data/raw/master/data/{}/recipes.json",
        file_locations.get("recipes").unwrap()
    );
    let recipes_data = get_data(
        &recipes_url,
        &format!("target/cache-recipes-{}.json", VERSION),
    );
    recipes::generate_recipes(recipes_data, items);
}
