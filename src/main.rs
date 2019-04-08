// WebService framework
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
extern crate actix;
extern crate actix_web;

// Logging & Console output.
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::sync::Arc;
use std::sync::RwLock;

mod solr_api {

    use std::sync::Arc;
    use std::sync::RwLock;

    use actix::SystemRunner;
    use actix_web::{middleware, server, App, HttpRequest, HttpResponse};

    /// simple handle
    fn index(req: &HttpRequest<AppState>) -> HttpResponse {
        println!("{:?}", req);
        {
            // So that we release ASAP the exclusive lock.
            *(req.state().shared.write().unwrap()) += 1;
        }

        HttpResponse::Ok().body(format!(
            "Num of requests: {}",
            req.state().shared.read().unwrap()
        ))
    }

    // Application shared state
    struct AppState {
        host: &'static str,
        port: u16,
        prefix: &'static str,
        shared: Arc<RwLock<i32>>,
    }

    pub fn run(host: &'static str, port: u16, prefix: &'static str, state: Arc<RwLock<i32>>) -> () {
        info!("Initializing server...");

        let sys = actix::System::new("spatial-search");

        server::new(move || {
            App::with_state(AppState {
                host,
                port,
                prefix,
                shared: state.clone(),
            }) // <- create app with shared state
            // register simple handler, handle all methods
            .resource("/", |r| r.f(index))
        })
        .bind(format!("{}:{}", host, port))
        .unwrap()
        .start();

        info!("Started http server: {:?}:{:?}", host, port);

        let _ = sys.run();
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    //TODO Retrieve from environment values, with fall back to defaults if unset.
    let hostname = "0.0.0.0";
    let base = "/spatial-search";
    let port = 8888;

    solr_api::run(hostname, port, base, Arc::new(RwLock::new(0)));
}
