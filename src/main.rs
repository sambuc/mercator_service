// WebService framework
//#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
extern crate actix;
extern crate actix_web;

// Logging & Console output.
#[macro_use]
extern crate log;

use std::process::exit;
use std::sync::Arc;
use std::sync::RwLock;

mod rest_api;

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

    if std::env::var("MERCATOR_DATA").is_err() {
        std::env::set_var("MERCATOR_DATA", ".");
    }
    let hostname;
    let port;
    let base;
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

    match std::env::var("MERCATOR_BASE") {
        Ok(val) => base = val,
        Err(val) => {
            error!("Could not fetch {} : `{}`", "MERCATOR_BASE", val);
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

    rest_api::run(hostname, port, base, Arc::new(RwLock::new(0)));
}
