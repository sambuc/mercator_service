use std::collections::HashSet;
use std::sync::RwLock;

use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::web::Path;
use mercator_db::CoreQueryParameters;

use crate::model::to_spatial_objects;
use crate::shared_state::SharedState;

use super::error_400;
use super::error_404;
use super::error_422;
use super::ok_200;
use super::Filters;
use super::HandlerResult;

fn post(
    (core_id, parameters, state): (Path<String>, Json<Filters>, Data<RwLock<SharedState>>),
) -> HandlerResult {
    trace!("POST '{:?}', {:?}", parameters, core_id);
    let core_id = core_id.to_string();
    let context = state.read().unwrap();
    let db = context.db();

    match db.core(core_id.clone()) {
        Err(_) => error_404(),
        Ok(core) => match parameters.space(db) {
            Err(e) => e,
            Ok(space) => match parameters.filters() {
                None => {
                    let mut results = HashSet::new();
                    for property in core.keys().iter() {
                        results.insert(property.id().clone());
                    }

                    if parameters.ids_only() {
                        ok_200(&results.drain().collect::<Vec<_>>())
                    } else {
                        let core_parameters = CoreQueryParameters {
                            db,
                            output_space: space.as_ref().map(String::as_str),
                            threshold_volume: parameters.volume(),
                            view_port: &parameters.view_port,
                            resolution: parameters.resolution(),
                        };

                        let objects = results
                            .drain()
                            .flat_map(|id| match core.get_by_id(&core_parameters, id) {
                                Err(_) => vec![], // FIXME: Return error ?
                                Ok(r) => r,
                            })
                            .collect::<Vec<_>>();
                        let objects = to_spatial_objects(db, objects);

                        ok_200(&objects)
                    }
                }
                Some(filter) => {
                    match context.filter(
                        filter,
                        &core_id,
                        space,
                        parameters.volume(),
                        &parameters.view_port,
                        parameters.resolution(),
                    ) {
                        Err(e) => error_422(e),
                        Ok(objects) => {
                            if parameters.ids_only() {
                                let mut uniques = HashSet::new();
                                for o in objects.iter() {
                                    uniques.insert(o.value.id().clone());
                                }

                                ok_200(&uniques.drain().collect::<Vec<_>>())
                            } else {
                                let objects = to_spatial_objects(db, objects);

                                ok_200(&objects)
                            }
                        }
                    }
                }
            },
        },
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
        web::resource("/cores/{name}/spatial_objects")
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

    #[test]
    fn post() {
        expect_200(Method::POST, &get_objects(""));
        json::expect_200(Method::POST, &get_objects(""), "".to_string());

        json::expect_422(Method::POST, &get_objects(""), "".to_string());

        expect_400(Method::POST, &get_objects(""));
    }

    #[test]
    fn put() {
        json::expect_200(Method::PUT, &get_objects(""), "".to_string());

        json::expect_422(Method::PUT, &get_objects(""), "".to_string());

        expect_400(Method::PUT, &get_objects(""));
    }

    #[test]
    fn patch() {
        json::expect_200(Method::PATCH, &get_objects(""), "".to_string());

        json::expect_422(Method::PATCH, &get_objects(""), "".to_string());

        expect_400(Method::PATCH, &get_objects(""));
    }

    #[test]
    fn delete() {
        json::expect_200(Method::DELETE, &get_objects(""), "".to_string());

        json::expect_422(Method::DELETE, &get_objects(""), "".to_string());

        expect_400(Method::DELETE, &get_objects(""));
    }

    #[test]
    fn get() {
        expect_405(Method::GET, &get_objects(""));
    }
}
