use reqwest::Result;
use std::env;

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
        // if the user provided the "get-card" command
        "get-card" if args.len() == 3 => {
            let argument = &args[2];
            if let Err(err) = fetch_data(format!("{}card/{}", BASE_URL, argument).as_str()).await {
                eprintln!("Error: {:?}", err);
            }
        }
        // if the user provided the "get-pack" command
        "get-pack" if args.len() == 3 => {
            let argument = &args[2];
            if let Err(err) = fetch_data(format!("{}cards/{}", BASE_URL, argument).as_str()).await {
                eprintln!("Error: {:?}", err);
            }
        }
        // if the user provided the "get-all-cards" command
        "get-all-cards" if args.len() == 2 => {
            // if let Err(err) = fetch_data(format!("{}/cards", BASE_URL).as_str()).await {
            //     eprintln!("Error: {:?}", err);
            // }
            eprintln!("Not implemented yet.");
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
    let response = reqwest::get(url).await?;
    let response_bytes = response.bytes().await?;
    let body = String::from_utf8(response_bytes.to_vec()).unwrap();

    println!("API response: {:?}", body);

    Ok(())
}

// display the help text
fn display_help() {
    println!("Available commands:");
    println!("get-all-cards        - Fetch information for all cards");
    println!("get-card <card_id>   - Fetch information for a card");
    println!("get-pack <pack_id>   - Fetch information for a pack");
}