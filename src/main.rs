use bevy::prelude::*;
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write};

// Bevy Doc
// https://bevyengine.org/learn/book/getting-started/apps/
#[tokio::main]
async fn main() {
    println!("Game Started!");

    let args: Vec<String> = std::env::args().collect();

    // Check for update argument and trigger fetch of card list
    if args.contains(&"-update".to_string()) {
        fetch_cards().await;
    }

    App::new().add_system(init_system).run();
}

fn init_system() {
    get_card("01000");
}

fn get_card(code: &str) {
    // Get json file with card data
    let mut file = File::open("src/assets/cards.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse JSON
    let json_value: Value = serde_json::from_str(&contents).unwrap();

    // Search for card via card code
    if let Some(card_value) = json_value
        .as_array()
        .unwrap()
        .iter()
        .find(|card| card["code"] == code)
    {
        // Print card value
        println!("{}", serde_json::to_string_pretty(&card_value).unwrap());
    } else {
        println!("Card with code {} not found", code);
    }
}

async fn fetch_cards() {
    // Retrieve latest card list from ArkhamDB public API
    let response = reqwest::get("https://arkhamdb.com/api/public/cards/?encounter=1")
        .await
        .unwrap();

    let status = response.status();

    match status {
        reqwest::StatusCode::OK => {
            let json_string = response.text().await.unwrap();

            // Write JSON to file
            let mut file = File::create("src/assets/cards.json").unwrap();
            file.write_all(json_string.as_bytes()).unwrap();
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Unable to fetch cards, unauthorized");
        }
        _ => {
            panic!("Unable to fetch cards, something went wrong");
        }
    };
}
