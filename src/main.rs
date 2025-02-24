use std::{fs, net::{IpAddr, Ipv4Addr}};

use data::GameData;
use gcp_auth::CustomServiceAccount;
use rocket::{get, http::{Header, Status}, launch, options, post, routes, Config, Responder, State};
use serde::{Deserialize, Serialize};

mod data;
mod sheets;

struct StateData {
    pub account: CustomServiceAccount,
    pub settings: Settings,
}

#[get("/")]
fn index() -> &'static str {
    "WARP7 Scouting API"
}

#[derive(Responder)]
struct OptionsResponder {
    response: Status,
    allow_origin: Header<'static>,
    allow_methods: Header<'static>,
    allow_headers: Header<'static>
}

#[options("/add_report")]
fn add_report_options(state: &State<StateData>) -> OptionsResponder {
    OptionsResponder {
        response: Status::new(200),
        allow_origin: Header { name: "Access-Control-Allow-Origin".into(), value: state.settings.frontend.clone().into() },
        allow_methods: Header { name: "Access-Control-Allow-Methods".into(), value: "POST".into() },
        allow_headers: Header { name: "Access-Control-Allow-Headers".into(), value: "*".into() }
    }
}

#[post("/add_report", data = "<raw_data>")]
async fn add_report_post(state: &State<StateData>, raw_data: &[u8]) {
    let data: GameData = match serde_json::from_slice(raw_data) {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to parse data: {e:#?}");
            return;
        }
    };
    let values = serde_json::to_value(&data)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .fold(vec![], |mut values, value| {
            values.push(String::from(value.as_str().unwrap()));
            values
        });

    let worksheet = if data.is_test {
        state.settings.test_worksheet.clone()
    } else {
        state.settings.main_worksheet.clone()
    };

    println!("{data:#?}\nvalues: {values:?}\nworksheet: {worksheet}");

    let result = sheets::append(
        &state.account,
        &state.settings.spreadsheet_id,
        &worksheet,
        values,
    )
    .await;
    println!("{result:#?}");
}

#[derive(Debug, Deserialize, Serialize)]
struct Settings {
    address: IpAddr,
    port: u16,
    frontend: String,
    credentials_path: String,
    spreadsheet_id: String,
    main_worksheet: String,
    test_worksheet: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 42069,
            frontend: String::from("*"),
            spreadsheet_id: String::from(""),
            credentials_path: String::from("service_account.json"),
            main_worksheet: String::from("Raw Data"),
            test_worksheet: String::from("Test Data"),
        }
    }
}

#[launch]
async fn launch() -> _ {
    let settings = if let Ok(settings_json) = fs::read("settings.json") {
        serde_json::from_slice(&settings_json).expect("Couldn't parse settings")
    } else {
        let settings = Settings::default();
        let settings_json = serde_json::to_string_pretty(&settings).unwrap();
        println!("Failed to read settings.json, writing default settings");
        match fs::write("settings.json", &settings_json) {
            Ok(_) => {}
            Err(e) => println!("Failed to write settings: {e}"),
        }

        settings
    };

    println!("Using these {settings:#?}");

    let config = Config {
        address: settings.address,
        port: settings.port,
        ..Config::default()
    };

    rocket::custom(&config)
        .manage(StateData {
            account: sheets::get_account(&settings.credentials_path),
            settings,
        })
        .mount("/warp7api/scouting", routes![index, add_report_options, add_report_post])
}
