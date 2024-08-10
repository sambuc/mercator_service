use std::collections::HashSet;
use std::sync::RwLock;

use super::error_400;
use super::error_422;
use super::ok_200;
use super::web;
use super::web::Data;
use super::web::Json;
use super::Core;
use super::CoreQueryParameters;
use super::Filters;
use super::HandlerResult;
use super::SharedState;

async fn post((parameters, state): (Json<Filters>, Data<RwLock<SharedState>>)) -> HandlerResult {
    trace!("POST '{:?}'", parameters);
    let context = state
        .read()
        .unwrap_or_else(|e| panic!("Can't acquire read lock of the database: {}", e));
    let db = context.db();

    match parameters.space(db) {
        Err(e) => e,
        Ok(space) => {
            match parameters.filters() {
                None => {
                    if parameters.ids_only() {
                        ok_200(db.core_keys())
                    } else {
                        let cores = db
                            .core_keys()
                            .iter()
                            .filter_map(|id| match db.core(id) {
                                Err(_) => None, // FIXME: Return error ?
                                Ok(x) => Some(Core::from(x)),
                            })
                            .collect::<Vec<_>>();

                        ok_200(&cores)
                    }
                }
                Some(filter) => {
                    let core_parameters = CoreQueryParameters {
                        db,
                        output_space: space.as_ref().map(String::as_str),
                        threshold_volume: parameters.volume(),
                        view_port: &parameters.view_port,
                        resolution: parameters.resolution(),
                    };

                    let tree = match context.filter(filter) {
                        Err(e) => return error_422(e),
                        Ok(tree) => tree,
                    };

                    // Retrieve the list of core ids.
                    let mut results = HashSet::new();
                    for core in db.core_keys() {
                        match context.execute(&tree, core, &core_parameters) {
                            Err(e) => return error_422(e),
                            Ok(objects) => {
                                // If the list of SpaceObjects is not empty, add
                                // the current core to the list.
                                if !objects.is_empty() {
                                    results.insert(core.to_string());
                                }
                            }
                        };
                    }

                    // Format the list or the whole core objects.
                    if parameters.ids_only() {
                        ok_200(&results.drain().collect::<Vec<_>>())
                    } else {
                        ok_200(
                            &results
                                .drain()
                                .map(|x| Core::from(db.core(&x).unwrap()))
                                .collect::<Vec<_>>(),
                        )
                    }
                }
            }
        }
    }
}

async fn put() -> HandlerResult {
    trace!("PUT Triggered!");
    error_400()
}

async fn patch() -> HandlerResult {
    trace!("PATCH Triggered!");
    error_400()
}

async fn delete() -> HandlerResult {
    trace!("DELETE Triggered!");
    error_400()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/cores")
            .route(web::post().to(post))
            .route(web::put().to(put))
            .route(web::patch().to(patch))
            .route(web::delete().to(delete)),
    );
}

#[cfg(test)]
mod routing {
    use super::super::tests_utils::*;

    // FIXME: Add Body to request to see difference between (in)valid bodied requests

    #[actix_web::test]
    async fn post() {
        expect_200(TestRequest::post(), &get_core("")).await;
        json::expect_200(TestRequest::post(), &get_core(""), "".to_string()).await;

        json::expect_422(TestRequest::post(), &get_core(""), "".to_string()).await;

        expect_400(TestRequest::post(), &get_core("")).await;
    }

    #[actix_web::test]
    async fn put() {
        json::expect_200(TestRequest::put(), &get_core(""), "".to_string()).await;

        json::expect_422(TestRequest::put(), &get_core(""), "".to_string()).await;

        expect_400(TestRequest::put(), &get_core("")).await;
    }

    #[actix_web::test]
    async fn patch() {
        json::expect_200(TestRequest::patch(), &get_core(""), "".to_string()).await;

        json::expect_422(TestRequest::patch(), &get_core(""), "".to_string()).await;

        expect_400(TestRequest::patch(), &get_core("")).await;
    }

    #[actix_web::test]
    async fn delete() {
        json::expect_200(TestRequest::delete(), &get_core(""), "".to_string()).await;

        json::expect_422(TestRequest::delete(), &get_core(""), "".to_string()).await;

        expect_400(TestRequest::delete(), &get_core("")).await;
    }

    #[actix_web::test]
    async fn get() {
        expect_405(TestRequest::get(), &get_core("")).await;
    }
}
