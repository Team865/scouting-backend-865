use std::{
    fs,
    fs::OpenOptions,
    io::Cursor,
    net::{IpAddr, Ipv4Addr},
};

use chrono::Local;
use data::GameData;
use fern::Dispatch;
use gcp_auth::CustomServiceAccount;
use log::{error, info};
use rocket::{
    Config, Response, State, get,
    http::{Header, Status},
    launch, options, post,
    response::Responder,
    routes,
};
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

struct CORSResponder {
    status: Status,
    body: String,
}

impl<'r> Responder<'r, 'static> for CORSResponder {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        Response::build()
            .status(self.status)
            .sized_body(self.body.len(), Cursor::new(self.body))
            .header(Header {
                name: "Access-Control-Allow-Origin".into(),
                value: "*".into(),
            })
            .header(Header {
                name: "Access-Control-Allow-Methods".into(),
                value: "POST".into(),
            })
            .header(Header {
                name: "Access-Control-Allow-Headers".into(),
                value: "*".into(),
            })
            .ok()
    }
}

impl Default for CORSResponder {
    fn default() -> Self {
        Self {
            status: Status::new(200),
            body: String::new(),
        }
    }
}

#[options("/add_report")]
fn add_report_options() -> CORSResponder {
    CORSResponder::default()
}

#[post("/add_report", data = "<raw_data>")]
async fn add_report_post(state: &State<StateData>, raw_data: &[u8]) -> CORSResponder {
    let data: GameData = match serde_json::from_slice(raw_data) {
        Ok(data) => data,
        Err(e) => {
            let body = format!("{e:#?}");
            error!("{body}");
            return CORSResponder {
                body,
                status: Status::new(400),
            };
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

    info!("{data:#?}\nvalues: {values:?}\nworksheet: {worksheet}");

    let result = sheets::append(
        &state.account,
        &state.settings.spreadsheet_id,
        &worksheet,
        &values,
    )
    .await;
    info!("{result:#?}");

    CORSResponder {
        body: format!("{data:?}"),
        ..Default::default()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Settings {
    root: String,
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
            root: String::from("/"),
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

// init log
fn setup_logging() -> Result<(), fern::InitError> {
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("output.log")?;

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        // Do not send to standard output.
        // .chain(std::io::stdout())
        .chain(log_file)
        .apply()?; // Apply the logging configuration
    Ok(())
}

#[launch]
async fn launch() -> _ {
    setup_logging().expect("Failed to set up logging");

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

    info!("Using these {settings:#?}");

    let config = Config {
        address: settings.address,
        port: settings.port,
        ..Config::default()
    };

    rocket::custom(&config)
        .manage(StateData {
            account: sheets::get_account(&settings.credentials_path),
            settings: settings.clone(),
        })
        .mount(
            settings.root.as_str(),
            routes![index, add_report_options, add_report_post],
        )
}
