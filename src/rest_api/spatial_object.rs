use std::sync::RwLock;

use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Path;
use mercator_db::CoreQueryParameters;

use crate::model::to_spatial_objects;
use crate::shared_state::SharedState;

use super::error_400;
use super::error_404;
use super::ok_200;
use super::HandlerResult;

fn put(path: Path<String>) -> HandlerResult {
    trace!("PUT '{:?}'", path);
    error_400()
}

fn get((path, state): (Path<(String, String)>, Data<RwLock<SharedState>>)) -> HandlerResult {
    trace!("GET '{:?}'", path);
    let (core, id) = path.into_inner();
    let core = core.to_string();
    let id = id.to_string();
    let context = state.read().unwrap();
    let db = context.db();

    // FIXME: Should we allow setting the resolution/threshold_volume?
    let parameters = CoreQueryParameters {
        db,
        output_space: None,
        // Enforce highest resolution index.
        threshold_volume: None,
        view_port: &None,
        resolution: Some(vec![0]),
    };

    match db.core(core) {
        Ok(core) => match core.get_by_id(&parameters, &id) {
            Ok(objects) => {
                let results = to_spatial_objects(db, objects);
                if results.is_empty() {
                    error_404()
                } else {
                    ok_200(&results)
                }
            }
            Err(_) => error_404(),
        },
        Err(_) => error_404(),
    }
}

fn patch(path: Path<String>) -> HandlerResult {
    trace!("PATCH Triggered on {}", path);
    error_400()
}

fn delete(path: Path<String>) -> HandlerResult {
    trace!("DELETE Triggered on {}", path);
    error_400()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/cores/{name}/spatial_objects/{id}")
            .route(web::get().to(get))
            .route(web::put().to(put))
            .route(web::patch().to(patch))
            .route(web::delete().to(delete)),
    );
}

#[cfg(test)]
mod routing {
    use super::super::tests_utils::*;

    const INSTANCE_EXISTS: &str = SPATIAL_OBJECT;
    const INSTANCE_INVALID: &str = "/21-doesnotexists";

    // FIXME: Add Body to request to see difference between (in)valid bodied requests

    #[test]
    fn put() {
        json::expect_200(Method::PUT, &get_objects(INSTANCE_EXISTS), "".to_string());
        json::expect_422(Method::PUT, &get_objects(INSTANCE_EXISTS), "".to_string());
        json::expect_200(Method::PUT, &get_objects(INSTANCE_INVALID), "".to_string());
    }

    #[test]
    fn patch() {
        json::expect_200(Method::PATCH, &get_objects(INSTANCE_EXISTS), "".to_string());
        json::expect_422(Method::PATCH, &get_objects(INSTANCE_EXISTS), "".to_string());
        expect_400(Method::PATCH, &get_objects(INSTANCE_INVALID));
    }

    #[test]
    fn get() {
        expect_200(Method::GET, &get_objects(INSTANCE_EXISTS));
        expect_404(Method::GET, &get_objects(INSTANCE_INVALID));
    }

    #[test]
    fn delete() {
        expect_200(Method::DELETE, &get_objects(INSTANCE_EXISTS));
        expect_404(Method::DELETE, &get_objects(INSTANCE_INVALID));
    }

    #[test]
    fn post() {
        expect_405(Method::POST, &get_objects(INSTANCE_EXISTS));
        expect_405(Method::POST, &get_objects(INSTANCE_INVALID));
    }
}
