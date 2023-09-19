use serde_json::Value;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

// Bevy Doc
// https://bevyengine.org/learn/book/getting-started

const PLAYERCARDS: &str = "src/assets/player_cards.json";
const ENCCARDS: &str = "src/assets/encounter_cards.json";

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check for update argument and trigger fetch of card list
    if args.contains(&"-update".to_string()) {
        fetch_cards().await;
    }

    create_player_deck();

    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_plugin(InitGame)
    //     .run();
}

// fn get_card(code: &str, json_value: Value) -> Result<Value, String> {
//     // Search for card via card code
//     if let Some(card_value) = json_value
//         .as_array()
//         .unwrap()
//         .iter()
//         .find(|card| card["code"] == code)
//     {
//         // return card value
//         Ok(card_value.clone())
//     } else {
//         Err(format!("Card with code {} not found", code))
//     }

//     // Example call
//     // let test = get_card("01000", json_value).unwrap();
//     // println!("{}", serde_json::to_string_pretty(&test).unwrap());
// }

async fn fetch_cards() {
    const ALL_CARDS_URL: &str = "https://arkhamdb.com/api/public/cards/?encounter=1";

    // Retrieve latest card list from ArkhamDB public API
    let response: reqwest::Response = reqwest::get(ALL_CARDS_URL).await.unwrap();
    let status: reqwest::StatusCode = response.status();

    match status {
        reqwest::StatusCode::OK => {
            let json_string: String = response.text().await.unwrap();

            split_file(json_string);
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Unable to fetch cards, unauthorized");
        }
        _ => {
            panic!("Unable to fetch cards, something went wrong");
        }
    };
}

fn split_file(json_string: String) {
    // Parse JSON from string
    let json_value: Value = serde_json::from_str(&json_string).unwrap();

    // Partition the json of all cards into encounter cards, and player cards
    let (encounter_cards, player_cards): (Vec<&Value>, Vec<&Value>) = json_value
        .as_array()
        .unwrap()
        .iter()
        .partition(|card| card["encounter_code"].is_string());

    // Write encounter cards to file
    let mut enc_cards_file: File = File::create(ENCCARDS).unwrap();
    serde_json::to_writer(&mut enc_cards_file, &encounter_cards).unwrap();

    // Write player cards to file
    let mut player_cards_file: File = File::create(PLAYERCARDS).unwrap();
    serde_json::to_writer(&mut player_cards_file, &player_cards).unwrap();
}

fn _create_encounter_deck(encounter_code_value: &str) -> Vec<Value> {
    // Read encounter card file
    let mut encounter_file: File = File::open(ENCCARDS).unwrap();
    let mut contents: String = String::new();
    encounter_file.read_to_string(&mut contents).unwrap();

    // Parse JSON
    let encounter_json: Value = serde_json::from_str(&contents).unwrap();

    // Search file for requested encounter
    let encounter_deck: Vec<Value> = encounter_json
        .as_array()
        .unwrap()
        .iter()
        .filter(|card| {
            if let Some(encounter_code) = card.get("encounter_code") {
                if let Some(name) = encounter_code.as_str() {
                    return name == encounter_code_value;
                }
            }
            false
        })
        .cloned()
        .collect::<Vec<_>>();
    return encounter_deck;
}

fn create_player_deck() {
    // Read player card file
    let mut player_card_file: File = File::open(PLAYERCARDS).unwrap();
    let mut contents: String = String::new();
    player_card_file.read_to_string(&mut contents).unwrap();

    // Parse JSON
    let player_cards_json: Value = serde_json::from_str(&contents).unwrap();

    // Get all investigators
    let investigators_cards: Vec<Value> = player_cards_json
        .as_array()
        .unwrap()
        .iter()
        .filter(|card| card["type_code"] == "investigator")
        .cloned()
        .collect::<Vec<_>>();

    // TODO: Find a way to handle investigators who had alternate versions (\"Skids\" O'Toole)
    // Get all unique names
    let mut names_set: HashSet<String> = HashSet::new();
    for card in investigators_cards {
        if let Some(name) = card.get("name") {
            if let Some(name_str) = name.as_str() {
                names_set.insert(String::from(name_str));
            }
        }
    }

    // Convert set to vector and sort
    let mut investigator_names: Vec<String> = names_set.into_iter().collect();
    investigator_names.sort();

    // Select investigator
    // Example: Roland Banks - Code 01001
    // TODO: Look into a better way to select investigators. Could have a use select a name then get original and alternates via "duplicate_of_code": "01001"
    let selected_investigator: &str = "01001";

    // Enforce deck restrictions

    // Reduce card catelog by restrictions

    // Return player deck
}