// WebService framework
//#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
//extern crate actix;
//extern crate actix_web;

#[macro_use]
extern crate measure_time;

#[macro_use]
extern crate serde_derive;

mod model;
mod rest_api;
mod storage;

use std::process::exit;
use std::sync::Arc;
use std::sync::RwLock;

use mercator_db::DataBase;

pub type SharedState = DataBase;

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
    /* UNUSED FOR NOW
    if std::env::var("MERCATOR_DATA").is_err() {
        std::env::set_var("MERCATOR_DATA", ".");
    }
    */

    let hostname;
    let port;
    let base;
    let allowed_origins: Vec<String>;
    //let data;

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

    match std::env::var("MERCATOR_BASE") {
        Ok(val) => base = val,
        Err(val) => {
            error!("Could not fetch {} : `{}`", "MERCATOR_BASE", val);
            exit(1);
        }
    };

    match std::env::var("MERCATOR_ALLOWED_ORIGINS") {
        Ok(val) => {
            allowed_origins = val
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>()
        }
        Err(val) => {
            error!("Could not fetch {} : `{}`", "MERCATOR_ALLOWED_ORIGINS", val);
            exit(1);
        }
    };

    /* UNUSED FOR NOW
    match std::env::var("MERCATOR_DATA") {
        Ok(val) => data = val,
        Err(val) => {
            error!("Could not fetch {} : `{}`", "MERCATOR_DATA", val);
            exit(1);
        }
    };*/

    let db;
    {
        // Temporary, until data ingestion can be done through the REST API.
        let import;

        if std::env::var("MERCATOR_IMPORT_DATA").is_err() {
            std::env::set_var("MERCATOR_IMPORT_DATA", "test_data");
        }

        match std::env::var("MERCATOR_IMPORT_DATA") {
            Ok(val) => import = val,
            Err(val) => {
                error!("Could not fetch {} : `{}`", "MERCATOR_IMPORT_DATA", val);
                exit(1);
            }
        };

        // Convert to binary the JSON data:
        if true {
            info_time!("Converting to binary JSON data");
            storage::convert(&import);
        }

        // Build a Database Index:
        if true {
            info_time!("Building database index");
            storage::build(&import);
        }

        // Load a Database:
        {
            info_time!("Loading database index");
            db = DataBase::load(&import).unwrap();
        }
        /*
        let core = db.core(&import).unwrap();

        let space = db.space("std").unwrap();
        let lower = space.encode(&[0.2, 0.2, 0.2]).unwrap();
        let higher = space.encode(&[0.8, 0.8, 0.8]).unwrap();

        let shape = Shape::BoundingBox(lower.clone(), higher.clone());
        let r;
        {
            info_time!("Query by box {:?} - {:?}", lower, higher);
            r = core.get_by_shape(&shape, 0.0).unwrap();
        }

        println!("get_by_shape {:?}: {}", shape, r.len());
        println!("{:?}: {:?}\n", shape, r[0]);
        */
        // END of Temporary bloc
    }

    rest_api::run(
        hostname,
        port,
        base,
        allowed_origins,
        Arc::new(RwLock::new(db)),
    );
}
