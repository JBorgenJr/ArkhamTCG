use bevy::prelude::*;
use serde_json::Value;
use std::fs::File;
// use std::io::Read;

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
        .partition(|card| card["encounter_name"].is_string());

    // Write encounter cards to file
    let mut enc_cards_file: File = File::create(ENCCARDS).unwrap();
    serde_json::to_writer(&mut enc_cards_file, &encounter_cards).unwrap();

    // Write player cards to file
    let mut player_cards_file: File = File::create(PLAYERCARDS).unwrap();
    serde_json::to_writer(&mut player_cards_file, &player_cards).unwrap();
}

// #[derive(Component)]
// struct Deck;

// fn create_deck(mut commands: Commands) {
//     commands.spawn((Deck));
// }

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
    // Create plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // Card Deck
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 0.5, 2.0))),
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
