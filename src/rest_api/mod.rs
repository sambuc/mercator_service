mod actions;

mod space;
mod spaces;

mod core;
mod cores;

mod spatial_object;
mod spatial_objects;

mod helpers;
mod helpers_dynamic_pages;
mod helpers_static_pages;

use std::io::Error;
use std::process::exit;
use std::sync::RwLock;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::http;
use actix_web::http::StatusCode;
use actix_web::middleware;
use actix_web::web;
pub use actix_web::web::Data;
use actix_web::App;
use actix_web::Either;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use mercator_db::space::Shape;
use mercator_db::storage::model;
use mercator_db::storage::model::v2::to_spatial_objects;
use mercator_db::CoreQueryParameters;
pub use mercator_db::DataBase;
use mercator_db::Properties;
use serde::Deserialize;
use serde::Serialize;

use crate::SharedState;

pub use helpers::*;

#[cfg(not(feature = "static-error-pages"))]
pub use helpers_dynamic_pages::*;

#[cfg(feature = "static-error-pages")]
pub use helpers_static_pages::*;

pub type HandlerResult = Result<Either<HttpResponse, NamedFile>, Error>;

#[derive(Clone, Debug, Deserialize)]
pub struct Filters {
    filters: Option<String>,
    ids_only: Option<bool>,
    space: Option<String>, // Output space, None, means each object in its own original space
    resolution: Option<Vec<u32>>, // None means automatic selection, based on ViewPort
    view_port: Option<(Vec<f64>, Vec<f64>)>,
}

impl Filters {
    pub fn filters(&self) -> &Option<String> {
        &self.filters
    }

    pub fn ids_only(&self) -> bool {
        match self.ids_only {
            None => true, // Defaults to true
            Some(b) => b,
        }
    }

    pub fn space(&self, db: &mercator_db::DataBase) -> Result<&Option<String>, HandlerResult> {
        if let Some(space_id) = &self.space {
            if !db.space_keys().contains(&space_id.to_string()) {
                return Err(error_422(format!(
                    "Invalid reference space id in '{:?}'",
                    self
                )));
            }
        }
        Ok(&self.space)
    }

    pub fn resolution(&self) -> &Option<Vec<u32>> {
        &self.resolution
    }

    pub fn volume(&self) -> Option<f64> {
        match &self.view_port {
            None => None,
            Some((low, high)) => Some(Shape::BoundingBox(low.into(), high.into()).volume()),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Core {
    name: String,
    version: String,
    scales: Vec<Vec<i32>>,
}

impl From<&mercator_db::Core> for Core {
    fn from(core: &mercator_db::Core) -> Self {
        Core {
            name: core.name().clone(),
            version: core.version().clone(),
            scales: vec![vec![0, 0, 0]],
            // FIXME: Report the actual values. Might need to change the format
            //        to per reference space.
        }
    }
}

// From: https://stackoverflow.com/a/52367953
fn into_static<S>(s: S) -> &'static str
where
    S: Into<String>,
{
    Box::leak(s.into().into_boxed_str())
}

fn config_v1(cfg: &mut web::ServiceConfig) {
    // Warning: Order matters, as a more generic path would catch calls for a
    // more specific one when registered first.

    space::config(cfg);
    spaces::config(cfg);

    core::config(cfg);
    cores::config(cfg);

    spatial_object::config(cfg);
    spatial_objects::config(cfg);

    actions::config(cfg);

    cfg.route("/static/{file:.*}", web::get().to(static_file));
    cfg.route("/api/{file:.*}", web::get().to(api));
    cfg.route("/", web::to(page_404));
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let prefix;

    match std::env::var("MERCATOR_BASE") {
        Ok(val) => prefix = val,
        Err(val) => {
            error!("Could not fetch {} : `{}`", "MERCATOR_BASE", val);
            exit(1);
        }
    };

    cfg.service(web::scope(into_static(format!("{}/v1", prefix))).configure(config_v1))
        .service(web::scope(into_static(prefix)).configure(config_v1))
        .route("/health", web::get().to(actions::health))
        .route("/static/{file:.*}", web::get().to(static_file));
}

pub fn get_cors() -> Cors {
    // Setup CORS support.
    let mut cors = Cors::new();

    match std::env::var("MERCATOR_ALLOWED_ORIGINS") {
        Ok(val) => {
            let allowed_origins = val.split(',').map(|s| s.trim()).collect::<Vec<_>>();

            for origin in allowed_origins {
                if !origin.is_empty() {
                    cors = cors.allowed_origin(into_static(origin));
                }
            }
        }
        Err(val) => {
            warn!(
                "Could not fetch {} : `{}`, allowing all origins",
                "MERCATOR_ALLOWED_ORIGINS", val
            );
        }
    }

    cors.allowed_methods(vec!["GET", "POST", "UPDATE", "PATCH", "DELETE", "OPTIONS"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(600)
}

macro_rules! get_app {
    ($state:expr) => {
        App::new()
            .register_data($state.clone())
            .wrap(middleware::Logger::new(
                r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T[s] %D[ms]"#,
            ))
            .wrap(middleware::Compress::default())
            .wrap(get_cors())
            .configure(config)
            .default_service(
                web::resource("/")
                    // 404 for GET request
                    .route(web::to(page_404)),
            )
    };
}

pub fn run(host: &str, port: u16, state: Data<RwLock<SharedState>>) {
    info!("Starting http server: {}:{}", host, port);

    // Create & run the server.
    match HttpServer::new(move || get_app!(state))
        .bind(format!("{}:{}", host, port))
        .unwrap()
        .run()
    {
        Ok(_) => info!("Server Stopped!"),
        Err(e) => error!("Error running the server: {}", e),
    };
}

#[cfg(test)]
mod tests_utils {
    use super::*;

    //use actix_server_config::ServerConfig;
    //use actix_service::IntoNewService;
    //use actix_service::NewService;
    use actix_service::Service;
    //use actix_web::dev::ServiceResponse;
    use actix_web::test;
    //use actix_web::test::TestRequest;
    use mercator_db::DataBase;

    pub const CORE_ID: &str = "10k";

    pub const PREFIX: &str = "/spatial-search";
    pub const CORE: &str = "/10k";
    pub const SPACE: &str = "/std";
    pub const SPATIAL_OBJECT: &str = "/oid0.44050628835072825";

    pub enum Method {
        GET,
        POST,
        PUT,
        PATCH,
        DELETE,
    }

    pub fn get_path(path: &str) -> String {
        format!("{}{}", PREFIX, path)
    }

    pub fn get_space(name: &str) -> String {
        format!("{}{}", get_path("/spaces"), name)
    }

    pub fn get_core(name: &str) -> String {
        format!("{}{}", get_path("/cores"), name)
    }

    pub fn get_objects(name: &str) -> String {
        format!("{}{}{}", get_core(CORE), "/spatial_objects", name)
    }

    pub fn expect(method: Method, path: &str, code: http::StatusCode) {
        std::env::set_var("MERCATOR_BASE", PREFIX);

        let mut app = test::init_service(get_app!(Data::new(RwLock::new(SharedState::new(
            DataBase::load(CORE_ID).unwrap()
        )))));

        let request = match method {
            Method::GET => test::TestRequest::get(),
            Method::POST => test::TestRequest::post(),
            Method::PUT => test::TestRequest::put(),
            Method::PATCH => test::TestRequest::patch(),
            Method::DELETE => test::TestRequest::delete(),
        };

        let request = request.uri(&path).to_request();
        let response = test::block_on(app.call(request)).unwrap();

        assert_eq!(response.status(), code);
    }

    pub fn expect_200(method: Method, path: &str) {
        expect(method, path, http::StatusCode::OK);
    }

    pub fn expect_400(method: Method, path: &str) {
        expect(method, path, http::StatusCode::BAD_REQUEST);
    }

    pub fn expect_404(method: Method, path: &str) {
        expect(method, path, http::StatusCode::NOT_FOUND);
    }

    pub fn expect_405(method: Method, path: &str) {
        expect(method, path, http::StatusCode::METHOD_NOT_ALLOWED);
    }

    pub fn expect_422(method: Method, path: &str) {
        expect(method, path, http::StatusCode::UNPROCESSABLE_ENTITY);
    }

    pub mod json {
        use super::*;

        pub fn expect_200(method: Method, path: &str, json: String) {
            expect(method, path, http::StatusCode::OK);
        }

        pub fn expect_404(method: Method, path: &str, json: String) {
            expect(method, path, http::StatusCode::NOT_FOUND);
        }

        pub fn expect_422(method: Method, path: &str, json: String) {
            expect(method, path, http::StatusCode::UNPROCESSABLE_ENTITY);
        }
    }
}

#[cfg(test)]
mod routing {
    use std::panic;

    use super::tests_utils::*;

    #[test]
    fn default_no_path() {
        // _FIXME: Currently the string is validated by the URI constructor which
        //        simply unwraps, thus we have to resort to this ugly workaround.
        //        The goal is to catch if that behavior changes in the future.
        let result = panic::catch_unwind(|| {
            expect_404(Method::GET, "");
        });
        assert!(result.is_err());
    }

    #[test]
    fn default_slash() {
        // We have to manually URL-encode spaces.
        expect_404(Method::GET, "/");
        expect_404(Method::GET, "//");
        expect_404(Method::GET, "/%20/");
        expect_404(Method::GET, "/%20//");
        expect_404(Method::GET, "//%20");
    }

    #[test]
    fn default_invalid_prefix() {
        expect_404(Method::GET, "/test");
        expect_404(Method::GET, &format!("{}test", PREFIX));
    }

    #[test]
    fn default_prefix_no_slash() {
        expect_404(Method::PUT, PREFIX);
        expect_404(Method::GET, PREFIX);
        expect_404(Method::POST, PREFIX);
        expect_404(Method::PATCH, PREFIX);
        expect_404(Method::DELETE, PREFIX);
    }

    #[test]
    fn default_prefix_final_slash() {
        let path = &format!("{}/", PREFIX);
        expect_404(Method::PUT, path);
        expect_404(Method::GET, path);
        expect_404(Method::POST, path);
        expect_404(Method::PATCH, path);
        expect_404(Method::DELETE, path);
    }
}
