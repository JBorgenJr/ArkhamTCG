use reqwest::{Response, Result};
use serde_json::{from_str, Value};
use std::{env, fs};

const CARDS_JSON: &str = "assets/cards/cards.json";
const BASE_URL: &str = "https://arkhamdb.com/api/public/";

#[tokio::main]
async fn main() {
    // get the command line arguments
    let args: Vec<String> = env::args().collect();

    // check if the user provided a command
    if args.len() < 2 {
        eprintln!("Usage: cargo run <command> [argument]");
        display_help();
        return;
    }

    // check which command the user provided
    match args[1].as_str() {
        // if the user provided the "update-player-cards" command
        "update-player-cards" if args.len() == 2 => {
            if let Err(err) = fetch_data(format!("{}cards/", BASE_URL).as_str()).await {
                eprintln!("Error: {:?}", err);
            }
        }
        // if the user provided the "update-all-cards" command
        "update-all-cards" if args.len() == 2 => {
            if let Err(err) = fetch_data(format!("{}cards/?encounter=1", BASE_URL).as_str()).await {
                eprintln!("Error: {:?}", err);
            }
        }
        _ => {
            // if the user provided an unknown command
            eprintln!("Unknown command or invalid usage.");
            display_help();
        }
    }
}

// fetch data from the API
async fn fetch_data(url: &str) -> Result<()> {
    let response: Response = reqwest::get(url).await?;

    // check if the request was successful
    if !response.status().is_success() {
        eprintln!("Error: {:?}", response.status());
        return Ok(());
    }

    // deserialize the response into a JSON object
    let response_json: Value = response.json().await?;

    update_cards(Some(response_json));

    Ok(())
}

fn update_cards(cards: Option<Value>) {
    println!("Updating cards...");

    // check if cards was provided
    if let Some(cards) = cards {
        println!("Updating card data...");
        // create cards.json file in assets folder from cards variable
        let cards_json = serde_json::to_string_pretty(&cards).unwrap();
        fs::write(CARDS_JSON, cards_json).unwrap();
    }

    // create a new directory for the cards in assets folder if it doesnt exist
    fs::create_dir_all("assets/cards/investigators").unwrap();
    fs::create_dir_all("assets/cards/encounters").unwrap();

    // get the cards.json file
    let cards_json = fs::read_to_string(CARDS_JSON).unwrap();

    // parse the cards.json file into a JSON object
    let cards_json: Value = from_str(&cards_json).unwrap();

    // seperate by card types
    let (investigator_cards, encounter_cards): (Vec<&Value>, Vec<&Value>) = cards_json
        .as_array()
        .unwrap()
        .iter()
        .partition(|card| card["type_code"].as_str().unwrap() == "investigator");

    // seperate encounter cards by unique pack_code
    let mut encounter_cards_by_pack: Vec<Vec<&Value>> = Vec::new();
    for card in encounter_cards.iter() {
        let pack_code = card["pack_code"].as_str().unwrap();
        let mut pack_found = false;
        for pack in encounter_cards_by_pack.iter_mut() {
            if pack[0]["pack_code"].as_str().unwrap() == pack_code {
                pack.push(card);
                pack_found = true;
                break;
            }
        }
        if !pack_found {
            encounter_cards_by_pack.push(vec![card]);
        }
    }

    // save investigator_cards to file
    println!("Saving investigator card data...");
    let investigator_cards_json = serde_json::to_string_pretty(&investigator_cards).unwrap();
    fs::write(
        "assets/cards/investigators/investigators.json",
        investigator_cards_json,
    )
    .unwrap();

    // save each encounter to its own file in encounter folder
    println!("Saving encounter card data...");
    for pack in encounter_cards_by_pack.iter() {
        let pack_code = pack[0]["pack_code"].as_str().unwrap();
        let pack_json = serde_json::to_string_pretty(&pack).unwrap();
        fs::write(
            format!("assets/cards/encounters/{}.json", pack_code),
            pack_json,
        )
        .unwrap();
    }
}

// display the help text
fn display_help() {
    println!("Available commands:");
    println!("update-player-cards   - Fetch and save all player cards");
    println!("update-all-cards      - Fetch and save player and encounter cards");
}
