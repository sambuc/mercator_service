// WebService framework
//#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
extern crate actix;
extern crate actix_web;

// Logging & Console output.
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::sync::Arc;
use std::sync::RwLock;

mod solr_api;

/*
fn into_bool(string: &str) -> bool {
    string.eq_ignore_ascii_case("true") || string.eq_ignore_ascii_case("on")
}
*/

fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    //TODO Retrieve from environment values, with fall back to defaults if unset.
    let hostname = "0.0.0.0";
    let base = "/spatial-search";
    let port = 8888;

    solr_api::run(hostname, port, base, Arc::new(RwLock::new(0)));
}
