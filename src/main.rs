//use bevy::prelude::*;
use std::fs::File;
use std::io::Write;

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

    //App::new().run();
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
