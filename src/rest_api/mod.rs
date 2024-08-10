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
use mercator_db::storage::model::v2::from_properties_by_spaces;
use mercator_db::storage::model::v2::from_spaces_by_properties;
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
        self.ids_only.unwrap_or(true)
    }

    pub fn space(&self, db: &DataBase) -> Result<&Option<String>, HandlerResult> {
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
        self.view_port.as_ref().map(|(low, high)|
            Shape::BoundingBox(low.into(), high.into()).volume()
        )
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
pub fn into_static<S>(s: S) -> &'static str
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
    let prefix = match std::env::var("MERCATOR_BASE") {
        Ok(val) => val,
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
    let mut cors = Cors::default();

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
            .app_data($state.clone())
            .wrap(middleware::Logger::new(
                r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T[s] %D[ms]"#,
            ))
            .wrap(middleware::Compress::default())
            .wrap(get_cors())
            .configure(config)
            .default_service(web::to(page_404))
    };
}

pub async fn run(host: &str, port: u16, state: Data<RwLock<SharedState>>) -> std::io::Result<()> {
    info!("Starting http server: {}:{}", host, port);

    // Create & run the server.
    HttpServer::new(move || get_app!(state))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}

#[cfg(test)]
mod tests_utils {
    use super::*;
    use actix_web::test;
    pub use actix_web::test::TestRequest;

    pub const CORE_FILE: &str = "10k.index";
    pub const CORE_ID: [&str; 1] = ["10k"];

    pub const PREFIX: &str = "/spatial-search-test";
    pub const CORE: &str = "/10k";
    pub const INVALID_CORE: &str = "/INVALID_CORE";
    pub const SPACE: &str = "/std";
    pub const SPATIAL_OBJECT: &str = "/oid0.44050628835072825";


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

    macro_rules! expect_code {
        ($request:expr, $path:expr, $code:expr) => {
            {
                std::env::set_var("MERCATOR_BASE", PREFIX);
                let db = DataBase::load(&[CORE_FILE]).unwrap();
                let app = test::init_service(
                    get_app!(Data::new(RwLock::new(SharedState::new(db))))).await;
                let request = $request.uri(&$path).to_request();
                let response = test::call_service(&app, request).await;
                assert_eq!(response.status(), $code);
                // let json = test::read_body(response).await;
                // println!("BODY: {:?}", json);
            }
        };
    }

    /// Checks status code OK
    pub async fn expect_200(method: TestRequest, path: &str) {
        expect_code!(method, path, StatusCode::OK);
    }

    /// Checks status code BAD_REQUEST
    pub async fn expect_400(method: TestRequest, path: &str) {
        expect_code!(method, path, StatusCode::BAD_REQUEST);
    }

    /// Checks status code NOT_FOUND
    pub async fn expect_404(method: TestRequest, path: &str) {
        expect_code!(method, path, StatusCode::NOT_FOUND);
    }

    /// Checks status code METHOD_NOT_ALLOWED
    pub async fn expect_405(method: TestRequest, path: &str) {
        expect_code!(method, path, StatusCode::METHOD_NOT_ALLOWED);
    }

    /// Checks status code UNPROCESSABLE_ENTITY
    pub async fn expect_422(method: TestRequest, path: &str) {
        expect_code!(method, path, StatusCode::UNPROCESSABLE_ENTITY);
    }

    pub mod json {
        use super::*;

        pub async fn expect_200(method: TestRequest, path: &str, _json: String) {
            expect_code!(method, path, StatusCode::OK);
        }

        pub async fn expect_404(method: TestRequest, path: &str, _json: String) {
            expect_code!(method, path, StatusCode::NOT_FOUND);
        }

        pub async fn expect_422(method: TestRequest, path: &str, _json: String) {
            expect_code!(method, path, StatusCode::UNPROCESSABLE_ENTITY);
        }
    }
}

#[cfg(test)]
mod routing {
    use super::tests_utils::*;
    use std::panic;

    #[ignore] // Don't know how to make work the catch_unwind in an async context
    #[actix_web::test]
    async fn default_no_path() {
        // _FIXME: Currently the string is validated by the URI constructor which
        //        simply unwraps, thus we have to resort to this ugly workaround.
        //        The goal is to catch if that behavior changes in the future.
        let result = panic::catch_unwind(|| {
            //expect_404(TestRequest::get(), "").await;
        });
        assert!(result.is_err());
    }

    #[actix_web::test]
    async fn default_slash() {
        // We have to manually URL-encode spaces.
        expect_404(TestRequest::get(), "/").await;
        expect_404(TestRequest::get(), "//").await;
        expect_404(TestRequest::get(), "/%20/").await;
        expect_404(TestRequest::get(), "/%20//").await;
        expect_404(TestRequest::get(), "//%20").await;
    }

    #[actix_web::test]
    async fn default_invalid_prefix() {
        expect_404(TestRequest::get(), "/test").await;
        expect_404(TestRequest::get(), &format!("{}test", PREFIX)).await;
    }

    #[actix_web::test]
    async fn default_prefix_no_slash() {
        expect_404(TestRequest::put(), PREFIX).await;
        expect_404(TestRequest::get(), PREFIX).await;
        expect_404(TestRequest::post(), PREFIX).await;
        expect_404(TestRequest::patch(), PREFIX).await;
        expect_404(TestRequest::delete(), PREFIX).await;
    }

    #[actix_web::test]
    async fn default_prefix_final_slash() {
        let path = &format!("{}/", PREFIX);
        expect_404(TestRequest::put(), path).await;
        expect_404(TestRequest::get(), path).await;
        expect_404(TestRequest::post(), path).await;
        expect_404(TestRequest::patch(), path).await;
        expect_404(TestRequest::delete(), path).await;
    }
}
