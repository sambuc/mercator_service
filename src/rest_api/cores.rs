use std::collections::HashSet;
use std::sync::RwLock;

use super::error_400;
use super::error_422;
use super::ok_200;
use super::web;
use super::web::Data;
use super::web::Json;
use super::Core;
use super::Filters;
use super::HandlerResult;
use super::SharedState;

fn post((parameters, state): (Json<Filters>, Data<RwLock<SharedState>>)) -> HandlerResult {
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
                    // Retrieve the list of core ids.
                    let mut results = HashSet::new();
                    for core in db.core_keys() {
                        match context.filter(
                            filter,
                            core,
                            &space,
                            parameters.volume(),
                            &parameters.view_port,
                            parameters.resolution(),
                        ) {
                            Err(e) => return error_422(e),
                            Ok(objects) => {
                                // If the list of SpaceObjects is not empty, add
                                // the current core to the list.
                                if !objects.is_empty() {
                                    results.insert(core.to_string());
                                }
                            }
                        }
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

fn put() -> HandlerResult {
    trace!("PUT Triggered!");
    error_400()
}

fn patch() -> HandlerResult {
    trace!("PATCH Triggered!");
    error_400()
}

fn delete() -> HandlerResult {
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

    const COLLECTION: &str = "/cores";

    // FIXME: Add Body to request to see difference between (in)valid bodied requests

    #[test]
    fn post() {
        expect_200(Method::POST, &get_core(""));
        json::expect_200(Method::POST, &get_core(""), "".to_string());

        json::expect_422(Method::POST, &get_core(""), "".to_string());

        expect_400(Method::POST, &get_core(""));
    }

    #[test]
    fn put() {
        json::expect_200(Method::PUT, &get_core(""), "".to_string());

        json::expect_422(Method::PUT, &get_core(""), "".to_string());

        expect_400(Method::PUT, &get_core(""));
    }

    #[test]
    fn patch() {
        json::expect_200(Method::PATCH, &get_core(""), "".to_string());

        json::expect_422(Method::PATCH, &get_core(""), "".to_string());

        expect_400(Method::PATCH, &get_core(""));
    }

    #[test]
    fn delete() {
        json::expect_200(Method::DELETE, &get_core(""), "".to_string());

        json::expect_422(Method::DELETE, &get_core(""), "".to_string());

        expect_400(Method::DELETE, &get_core(""));
    }

    #[test]
    fn get() {
        expect_405(Method::GET, &get_core(""));
    }
}
