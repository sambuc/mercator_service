mod actions;

mod space;
mod spaces;

mod core;
mod cores;

mod spatial_object;
mod spatial_objects;

mod default;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::fs;
use actix_web::http;
use actix_web::http::Method;
use actix_web::http::StatusCode;
use actix_web::middleware;
use actix_web::middleware::cors::Cors;
use actix_web::pred;
use actix_web::server;
use actix_web::server::HttpHandler;
use actix_web::server::HttpHandlerTask;
use actix_web::App;
use actix_web::Either;
use actix_web::Json;
use serde::Serialize;

use crate::SharedState;

// Application shared state
pub struct AppState {
    shared: Arc<RwLock<SharedState>>,
}

#[derive(Debug, Deserialize)]
pub struct Filters {
    filters: Option<String>,
    ids_only: Option<bool>,
}

impl Filters {
    pub fn get(parameters: Option<Json<Filters>>) -> Self {
        trace!("PARAMETERS {:#?}", parameters);

        match parameters {
            None => Filters {
                filters: None,
                ids_only: Some(true),
            },
            Some(p) => p.0,
        }
    }
}

type StringOrStaticFileResult = Either<String, fs::NamedFile>;

pub fn ok_200<T>(data: &T) -> StringOrStaticFileResult
where
    T: Serialize,
{
    Either::A(
        serde_json::to_string_pretty(data)
            .unwrap_or_else(|e| format!("Internal Error 500: {:?}", e)),
    )
}

pub fn error_400() -> StringOrStaticFileResult {
    Either::B(
        fs::NamedFile::open("static/400.html")
            .unwrap()
            .set_status_code(StatusCode::BAD_REQUEST),
    )
}

pub fn error_404() -> StringOrStaticFileResult {
    Either::B(
        fs::NamedFile::open("static/404.html")
            .unwrap()
            .set_status_code(StatusCode::NOT_FOUND),
    )
}

// From: https://stackoverflow.com/a/52367953
fn into_static<S>(s: S) -> &'static str
where
    S: Into<String>,
{
    Box::leak(s.into().into_boxed_str())
}

fn get_app<S>(
    prefix: S,
    allowed_origins: &[&'static str],
    state: Arc<RwLock<SharedState>>,
) -> Vec<Box<HttpHandler<Task = Box<HttpHandlerTask>>>>
where
    S: Into<String>,
{
    vec![
        App::with_state(AppState { shared: state })
            .prefix(into_static(prefix).to_string())
            .middleware(middleware::Logger::new(
                r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T[s] %D[ms]"#,
            ))
            // ACTIONS           -------------------------------------------------------------------
            .resource("/health", |r| {
                r.method(Method::GET).f(actions::health);
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(default::page_400);
            })
            // DEFAULT           -------------------------------------------------------------------
            .default_resource(|r| {
                r.f(default::page_400);
            })
            // REQUIRES CORS Support ---------------------------------------------------------------
            .configure(|app| {
                let mut cors = Cors::for_app(app);
                for origin in allowed_origins {
                    cors.allowed_origin(origin);
                }
                cors.allowed_methods(vec!["GET", "POST", "UPDATE", "PATCH", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(600)
                    .resource("/queries", |r| {
                        r.method(Method::POST).with(actions::query);
                        r.route()
                            .filter(pred::Not(pred::Post()))
                            .f(default::page_400);
                    })
                    // SPACES            -------------------------------------------------------------------
                    .resource("/spaces", |r| {
                        r.method(Method::POST).with(spaces::post);
                        r.method(Method::PUT).with(spaces::put);
                        r.method(Method::PATCH).with(spaces::patch);
                        r.method(Method::DELETE).with(spaces::delete);
                    })
                    .resource("/spaces/{name}", |r| {
                        r.method(Method::PUT).with(space::put);
                        r.method(Method::PATCH).with(space::patch);
                        r.method(Method::GET).with(space::get);
                        r.method(Method::DELETE).with(space::delete);
                    })
                    // DATASETS          -------------------------------------------------------------------
                    .resource("/cores", |r| {
                        r.method(Method::POST).with(&cores::post);
                        r.method(Method::PUT).with(&cores::put);
                        r.method(Method::PATCH).with(&cores::patch);
                        r.method(Method::DELETE).with(&cores::delete);
                    })
                    .resource("/cores/{name}", |r| {
                        r.method(Method::PUT).with(core::put);
                        r.method(Method::GET).with(core::get);
                        r.method(Method::PATCH).with(core::patch);
                        r.method(Method::DELETE).with(core::delete);
                    })
                    // SPATIAL OBJECTS   -------------------------------------------------------------------
                    .resource("/cores/{name}/spatial_objects", |r| {
                        r.method(Method::POST).with(spatial_objects::post);
                        r.method(Method::PUT).with(spatial_objects::put);
                        r.method(Method::PATCH).with(spatial_objects::patch);
                        r.method(Method::DELETE).with(spatial_objects::delete);
                    })
                    .resource("/cores/{name}/spatial_objects/{id}", |r| {
                        r.method(Method::PUT).with(spatial_object::put);
                        r.method(Method::GET).with(spatial_object::get);
                        r.method(Method::PATCH).with(spatial_object::patch);
                        r.method(Method::DELETE).with(spatial_object::delete);
                    })
                    .register()
            })
            .boxed(),
        App::new()
            .resource("/static/{file}", |r| {
                r.method(Method::GET).with(default::static_file)
            })
            .default_resource(|r| {
                // 404 for GET request
                r.method(Method::GET).f(default::page_404);

                // all requests that are not `GET`
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(default::page_400_no_state);
            })
            .boxed(),
    ]
}

pub fn run<S>(
    host: S,
    port: u16,
    prefix: S,
    allowed_origins: Vec<S>,
    state: Arc<RwLock<SharedState>>,
) where
    S: Into<String>,
{
    info!("Initializing server...");

    let sys = actix::System::new("spatial-search");
    let prefix = into_static(prefix);
    let host = host.into();

    let mut origins = Vec::with_capacity(allowed_origins.len());
    for origin in allowed_origins {
        origins.push(into_static(origin));
    }

    server::new(move || get_app(prefix, &origins, state.clone()))
        .bind(format!("{}:{}", host, port))
        .unwrap()
        .start();

    info!("Started http server: {}:{}{}", host, port, prefix);

    let _ = sys.run();
}

#[cfg(test)]
mod tests {
    use super::get_app;
    use super::{Arc, RwLock, SharedState};

    pub use actix_web::http;
    pub use actix_web::http::Method;
    pub use actix_web::test::TestServer;

    pub const PREFIX: &str = "spatial-search";

    fn get_start_state() -> Arc<RwLock<SharedState>> {
        Arc::new(RwLock::new(0))
    }

    pub fn get_test_server() -> TestServer {
        TestServer::with_factory(move || get_app(PREFIX, get_start_state().clone()))
    }

    pub fn get_path(path: &str) -> String {
        format!("{}{}", PREFIX, path)
    }

    pub fn expect_200(method: Method, path: String) -> () {
        let mut srv = get_test_server();
        let req = srv.client(method, path.as_str()).finish().unwrap();
        let response = srv.execute(req.send()).unwrap();
        assert_eq!(http::StatusCode::OK, response.status());
    }

    pub fn expect_400(method: Method, path: String) -> () {
        let mut srv = get_test_server();
        let req = srv.client(method, path.as_str()).finish().unwrap();
        let response = srv.execute(req.send()).unwrap();
        assert_eq!(http::StatusCode::BAD_REQUEST, response.status());
    }

    pub fn expect_404(method: Method, path: String) -> () {
        let mut srv = get_test_server();
        let req = srv.client(method, path.as_str()).finish().unwrap();
        let response = srv.execute(req.send()).unwrap();
        assert_eq!(http::StatusCode::NOT_FOUND, response.status());
    }

    pub fn expect_422(method: Method, path: String) -> () {
        let mut srv = get_test_server();
        let req = srv.client(method, path.as_str()).finish().unwrap();
        let response = srv.execute(req.send()).unwrap();
        assert_eq!(http::StatusCode::UNPROCESSABLE_ENTITY, response.status());
    }

    pub mod json {
        use super::*;

        pub fn expect_200(method: Method, path: String, json: String) -> () {
            let mut srv = get_test_server();
            let req = srv.client(method, path.as_str()).json(json).unwrap();
            let response = srv.execute(req.send()).unwrap();
            assert_eq!(http::StatusCode::OK, response.status());
        }

        /*
        pub fn expect_400(method: Method, path: String, json: String) -> () {
            let mut srv = get_test_server();
            let req = srv.client(method, path.as_str()).json(json).unwrap();
            let response = srv.execute(req.send()).unwrap();
            assert_eq!(http::StatusCode::BAD_REQUEST, response.status());
        }
        */

        pub fn expect_404(method: Method, path: String, json: String) -> () {
            let mut srv = get_test_server();
            let req = srv.client(method, path.as_str()).json(json).unwrap();
            let response = srv.execute(req.send()).unwrap();
            assert_eq!(http::StatusCode::NOT_FOUND, response.status());
        }

        pub fn expect_422(method: Method, path: String, json: String) -> () {
            let mut srv = get_test_server();
            let req = srv.client(method, path.as_str()).json(json).unwrap();
            let response = srv.execute(req.send()).unwrap();
            assert_eq!(http::StatusCode::UNPROCESSABLE_ENTITY, response.status());
        }
    }
}
