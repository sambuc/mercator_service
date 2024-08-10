use std::collections::HashSet;
use std::sync::RwLock;

use super::error_400;
use super::error_404;
use super::error_422;
use super::from_properties_by_spaces;
use super::from_spaces_by_properties;
use super::ok_200;
use super::web;
use super::web::Data;
use super::web::Json;
use super::web::Path;
use super::CoreQueryParameters;
use super::Filters;
use super::HandlerResult;
use super::SharedState;

async fn post(
    (core_id, parameters, state): (Path<String>, Json<Filters>, Data<RwLock<SharedState>>),
) -> HandlerResult {
    trace!("POST '{:?}', {:?}", parameters, core_id);
    let core_id = core_id.to_string();
    let context = state
        .read()
        .unwrap_or_else(|e| panic!("Can't acquire read lock of the database: {}", e));
    let db = context.db();

    match db.core(&core_id) {
        Err(_) => error_404(),
        Ok(core) => match parameters.space(db) {
            Err(e) => e,
            Ok(space) => match parameters.filters() {
                None => {
                    if parameters.ids_only() {
                        // keys() contains unique values only.
                        let ids = core
                            .keys()
                            .iter()
                            .map(|properties| properties.id())
                            .collect::<Vec<_>>();

                        ok_200(&ids)
                    } else {
                        let core_parameters = CoreQueryParameters {
                            db,
                            output_space: space.as_ref().map(String::as_str),
                            threshold_volume: parameters.volume(),
                            view_port: &parameters.view_port,
                            resolution: parameters.resolution(),
                        };

                        let objects_by_spaces =
                            Box::new(core.keys().iter().filter_map(|property| {
                                match core.get_by_id(&core_parameters, property.id()) {
                                    Err(_) => None, // FIXME: Return error ?
                                    Ok(positions_by_spaces) => {
                                        Some((property, positions_by_spaces))
                                    }
                                }
                            }));
                        ok_200(&from_spaces_by_properties(objects_by_spaces).collect::<Vec<_>>())
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
                        Ok(bag) => bag,
                    };

                    let r = match context.execute(&tree, &core_id, &core_parameters) {
                        Err(e) => error_422(e),
                        Ok(objects) => {
                            if parameters.ids_only() {
                                let mut uniques = HashSet::new();
                                for (_, v) in objects {
                                    for (_, properties) in v {
                                        uniques.insert(properties.id());
                                    }
                                }

                                ok_200(&uniques.drain().collect::<Vec<_>>())
                            } else {
                                ok_200(&from_properties_by_spaces(objects).collect::<Vec<_>>())
                            }
                        }
                    };

                    r
                }
            },
        },
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

    #[actix_web::test]
    async fn post() {
        expect_200(TestRequest::post(), &get_objects("")).await;
        json::expect_200(TestRequest::post(), &get_objects(""), "".to_string()).await;

        json::expect_422(TestRequest::post(), &get_objects(""), "".to_string()).await;

        expect_400(TestRequest::post(), &get_objects("")).await;
    }

    #[actix_web::test]
    async fn put() {
        json::expect_200(TestRequest::put(), &get_objects(""), "".to_string()).await;

        json::expect_422(TestRequest::put(), &get_objects(""), "".to_string()).await;

        expect_400(TestRequest::put(), &get_objects("")).await;
    }

    #[actix_web::test]
    async fn patch() {
        json::expect_200(TestRequest::patch(), &get_objects(""), "".to_string()).await;

        json::expect_422(TestRequest::patch(), &get_objects(""), "".to_string()).await;

        expect_400(TestRequest::patch(), &get_objects("")).await;
    }

    #[actix_web::test]
    async fn delete() {
        json::expect_200(TestRequest::delete(), &get_objects(""), "".to_string()).await;

        json::expect_422(TestRequest::delete(), &get_objects(""), "".to_string()).await;

        expect_400(TestRequest::delete(), &get_objects("")).await;
    }

    #[actix_web::test]
    async fn get() {
        expect_405(TestRequest::get(), &get_objects("")).await;
    }
}
