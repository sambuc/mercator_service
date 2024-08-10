use std::sync::RwLock;

use super::error_400;
use super::error_404;
use super::from_properties_by_spaces;
use super::ok_200;
use super::web;
use super::web::Data;
use super::web::Path;
use super::CoreQueryParameters;
use super::HandlerResult;
use super::Properties;
use super::SharedState;
use mercator_db::{IterObjects, IterObjectsBySpaces};

async fn put(path: Path<String>) -> HandlerResult {
    trace!("PUT '{:?}'", path);
    error_400()
}

async fn get((path, state): (Path<(String, String)>, Data<RwLock<SharedState>>)) -> HandlerResult {
    trace!("GET '{:?}'", path);
    let (core, id) = path.into_inner();
    let context = state
        .read()
        .unwrap_or_else(|e| panic!("Can't acquire read lock of the database: {}", e));
    let db = context.db();

    // FIXME: Should we allow setting the resolution/threshold_volume?
    let parameters = CoreQueryParameters {
        db,
        output_space: None,
        // Enforce highest resolution index.
        threshold_volume: None,
        view_port: &None,
        resolution: &Some(vec![0]),
    };

    match db.core(&core) {
        Ok(core) => match core.get_by_id(&parameters, &id) {
            Ok(positions_by_spaces) => {
                let value = Properties::Feature(id);
                let tmp: IterObjectsBySpaces = positions_by_spaces
                    .into_iter()
                    .map(|(space, positions)| {
                        let objects: IterObjects =
                            Box::new(positions.map(|position| (position, &value)));
                        (space, objects)
                    })
                    .collect();

                let results = from_properties_by_spaces(tmp).collect::<Vec<_>>();

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

async fn patch(path: Path<String>) -> HandlerResult {
    trace!("PATCH Triggered on {}", path);
    error_400()
}

async fn delete(path: Path<String>) -> HandlerResult {
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

    #[actix_web::test]
    async fn put() {
        json::expect_200(TestRequest::put(), &get_objects(INSTANCE_EXISTS), "".to_string()).await;
        json::expect_422(TestRequest::put(), &get_objects(INSTANCE_EXISTS), "".to_string()).await;
        json::expect_200(TestRequest::put(), &get_objects(INSTANCE_INVALID), "".to_string()).await;
    }

    #[actix_web::test]
    async fn patch() {
        json::expect_200(TestRequest::patch(), &get_objects(INSTANCE_EXISTS), "".to_string()).await;
        json::expect_422(TestRequest::patch(), &get_objects(INSTANCE_EXISTS), "".to_string()).await;
        expect_400(TestRequest::patch(), &get_objects(INSTANCE_INVALID)).await;
    }

    #[actix_web::test]
    async fn get() {
        expect_200(TestRequest::get(), &get_objects(INSTANCE_EXISTS)).await;
        expect_404(TestRequest::get(), &get_objects(INSTANCE_INVALID)).await;
    }

    #[actix_web::test]
    async fn delete() {
        expect_200(TestRequest::delete(), &get_objects(INSTANCE_EXISTS)).await;
        expect_404(TestRequest::delete(), &get_objects(INSTANCE_INVALID)).await;
    }

    #[actix_web::test]
    async fn post() {
        expect_405(TestRequest::post(), &get_objects(INSTANCE_EXISTS)).await;
        expect_405(TestRequest::post(), &get_objects(INSTANCE_INVALID)).await;
    }
}
