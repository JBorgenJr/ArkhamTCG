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

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InitGame)
        .run();
}

fn get_card(code: &str, json_value: Value) -> Result<Value, String> {
    // Search for card via card code
    if let Some(card_value) = json_value
        .as_array()
        .unwrap()
        .iter()
        .find(|card| card["code"] == code)
    {
        // return card value
        Ok(card_value.clone())
    } else {
        Err(format!("Card with code {} not found", code))
    }
}

async fn fetch_cards() {
    // Retrieve latest card list from ArkhamDB public API
    let response: reqwest::Response =
        reqwest::get("https://arkhamdb.com/api/public/cards/?encounter=1")
            .await
            .unwrap();

    let status: reqwest::StatusCode = response.status();

    match status {
        reqwest::StatusCode::OK => {
            let json_string: String = response.text().await.unwrap();

            // Write JSON to file
            let mut file: File = File::create("src/assets/cards.json").unwrap();
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

// #[derive(Component)]
// struct Card;
// #[derive(Component)]
// struct Code(String);

// fn create_card(mut commands: Commands) {
//     commands.spawn((Card, Code("01001".to_string())));
//     commands.spawn((Card, Code("01002".to_string())));
// }

pub struct InitGame;
impl Plugin for InitGame {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_config);
    }
}

fn load_config() {
    let mut file: File = File::open("src/assets/cards.json").unwrap();

    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse JSON
    let json_value: Value = serde_json::from_str(&contents).unwrap();

    let test = get_card("01000", json_value).unwrap();

    println!("{}", serde_json::to_string_pretty(&test).unwrap());
}
