#![forbid(unsafe_code)]

#[macro_use]
extern crate measure_time;

mod rest_api;
mod shared_state;

use std::process::exit;
use std::sync::RwLock;

use glob::glob;

use rest_api::Data;
use rest_api::DataBase;
use shared_state::SharedState;

/*
fn into_bool(string: &str) -> bool {
    string.eq_ignore_ascii_case("true") || string.eq_ignore_ascii_case("on")
}
*/

fn main() {
    // If RUST_LOG is unset, set it to INFO, otherwise keep it as-is.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    if std::env::var("MERCATOR_HOST").is_err() {
        std::env::set_var("MERCATOR_HOST", "0.0.0.0");
    }

    if std::env::var("MERCATOR_PORT").is_err() {
        std::env::set_var("MERCATOR_PORT", "8888");
    }

    if std::env::var("MERCATOR_BASE").is_err() {
        std::env::set_var("MERCATOR_BASE", "/spatial-search");
    }

    if std::env::var("MERCATOR_ALLOWED_ORIGINS").is_err() {
        // Allow by default access from a locally running Swagger Editor instance.
        std::env::set_var("MERCATOR_ALLOWED_ORIGINS", "http://localhost:3200");
    }

    if std::env::var("MERCATOR_DATA").is_err() {
        std::env::set_var("MERCATOR_DATA", ".");
    }

    let hostname;
    let port;
    let data;

    match std::env::var("MERCATOR_HOST") {
        Ok(val) => hostname = val,
        Err(val) => {
            error!("Invalid environment {} : `{}`", "MERCATOR_HOST", val);
            exit(1);
        }
    };

    match std::env::var("MERCATOR_PORT") {
        Ok(val) => match val.parse::<u16>() {
            Ok(v) => port = v,
            Err(e) => {
                error!("Could not convert to u16 {} : `{}`", "MERCATOR_PORT", e);
                exit(1);
            }
        },
        Err(val) => {
            error!("Could not fetch {} : `{}`", "MERCATOR_PORT", val);
            exit(1);
        }
    };

    match std::env::var("MERCATOR_DATA") {
        Ok(val) => data = val,
        Err(val) => {
            error!("Could not fetch {} : `{}`", "MERCATOR_DATA", val);
            exit(1);
        }
    };

    let datasets = glob(&format!("{}/*.index", data))
        .expect("Failed to read glob pattern")
        .filter_map(|entry| match entry {
            Ok(path) => match path.canonicalize() {
                Ok(path) => Some(format!("{}", path.display())),
                Err(_) => None,
            },
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    // FIXME: Why do we have to go through a temporary variable?
    let datasets = datasets.iter().map(String::as_str).collect::<Vec<_>>();

    let db;
    // Load a Database:
    {
        // Load all the index contained in the folder, and fail if anyone of
        // those is corrupted / incompatible.
        info_time!("Loading database index");

        db = DataBase::load(&datasets)
            .unwrap_or_else(|e| panic!("Error while loading indices: {}", e));
    }

    rest_api::run(
        &hostname,
        port,
        Data::new(RwLock::new(SharedState::new(db))),
    );
}
