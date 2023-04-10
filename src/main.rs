use bevy::prelude::*;
//use serde::Deserialize;
use reqwest::Error;

// Bevy Doc
// https://bevyengine.org/learn/book/getting-started/apps/
fn main() {
    println!("Game Started!");
    // WIP Update function
    //update_assets();
    App::new().run();
}

#[tokio::main]
async fn update_assets() -> Result<(), Error> {
    println!("Updating Assets!");   
    // Gets all player cards, to get encounter cards, set encounter parameter to 1
    let request_url = "https://arkhamdb.com/api/public/cards/";
    println!("{}", request_url);

    let response = reqwest::get(&request_url).await?;
    println!("{}", response);
    Ok(())
}
