use data::GameData;
use rocket::{Config, get, launch, post, routes};

mod api;
mod data;
mod sheets;

#[get("/")]
fn index() -> &'static str {
    "WARP7 Scouting API"
}

#[post("/add_report", data = "<raw_data>")]
fn add_report(raw_data: &[u8]) {
    let data: GameData = if let Ok(data) = serde_json::from_slice(raw_data) {
        data
    } else {
        return;
    };
    let values = serde_json::to_value(data)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .fold(vec![], |mut values, value| {
            values.push(String::from(value.as_str().unwrap()));
            values
        });
    println!("{values:#?}")
}

#[launch]
fn launch() -> _ {
    let config = Config {
        port: 42069,
        ..Config::default()
    };

    rocket::custom(&config).mount("/warp7api/scouting", routes![index, add_report])
}
