use std::collections::HashSet;
use std::sync::RwLock;

use super::error_400;
use super::error_422;
use super::model;
use super::ok_200;
use super::web;
use super::web::Data;
use super::web::Json;
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
                        ok_200(db.space_keys())
                    } else {
                        let spaces = db
                            .space_keys()
                            .iter()
                            .filter_map(|id| match db.space(id) {
                                Err(_) => None, // FIXME: Return error ?
                                Ok(x) => Some(model::Space::from(x)),
                            })
                            .collect::<Vec<_>>();

                        ok_200(&spaces)
                    }
                }
                Some(filter) => {
                    // Retrieve the list of space ids.
                    let mut results = HashSet::new();

                    let core_parameters = CoreQueryParameters {
                        db: context.db(),
                        output_space: space.as_ref().map(String::as_str),
                        threshold_volume: parameters.volume(),
                        view_port: &parameters.view_port,
                        resolution: parameters.resolution(),
                    };
                    let tree = match context.filter(filter) {
                        Err(e) => return error_422(e),
                        Ok(bag) => bag,
                    };

                    for core in db.core_keys() {
                        match context.execute(&tree, core, &core_parameters) {
                            Err(e) => return error_422(e),
                            Ok(v) => {
                                // We have a list of SpaceObjects, so extract
                                // the space Ids
                                for (space_id, _) in v {
                                    results.insert(space_id);
                                }
                            }
                        }
                    }

                    // Format the list or the whole space objects.
                    if parameters.ids_only() {
                        ok_200(&results.drain().collect::<Vec<_>>())
                    } else {
                        ok_200(
                            &results
                                .drain()
                                .map(|id| match db.space(id) {
                                    Err(_) => None,
                                    Ok(x) => Some(model::Space::from(x)),
                                })
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
        web::resource("/spaces")
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
        expect_200(TestRequest::post(), &get_space("")).await;
        json::expect_200(TestRequest::post(), &get_space(""), "".to_string()).await;

        json::expect_422(TestRequest::post(), &get_space(""), "".to_string()).await;

        expect_400(TestRequest::post(), &get_space("")).await;
    }

    #[actix_web::test]
    async fn put() {
        json::expect_200(TestRequest::put(), &get_space(""), "".to_string()).await;

        json::expect_422(TestRequest::put(), &get_space(""), "".to_string()).await;

        expect_400(TestRequest::put(), &get_space("")).await;
    }

    #[actix_web::test]
    async fn patch() {
        json::expect_200(TestRequest::patch(), &get_space(""), "".to_string()).await;

        json::expect_422(TestRequest::patch(), &get_space(""), "".to_string()).await;

        expect_400(TestRequest::patch(), &get_space("")).await;
    }

    #[actix_web::test]
    async fn delete() {
        json::expect_200(TestRequest::delete(), &get_space(""), "".to_string()).await;

        json::expect_422(TestRequest::delete(), &get_space(""), "".to_string()).await;

        expect_400(TestRequest::delete(), &get_space("")).await;
    }

    #[actix_web::test]
    async fn get() {
        expect_405(TestRequest::get(), &get_space("")).await;
    }
}
