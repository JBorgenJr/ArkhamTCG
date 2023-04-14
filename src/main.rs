use bevy::prelude::*;
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write};

// Bevy Doc
// https://bevyengine.org/learn/book/getting-started

const CARDPATH: &str = "src/assets/cards.json";

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

            // Write JSON to file
            let mut file: File = File::create(CARDPATH).unwrap();
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

pub struct InitGame;
impl Plugin for InitGame {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut file: File = File::open(CARDPATH).unwrap();

    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse JSON
    let json_value: Value = serde_json::from_str(&contents).unwrap();

    // Create plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // Card Deck
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.3, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
