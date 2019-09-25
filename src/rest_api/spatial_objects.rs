use std::sync::RwLock;

use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::web::Path;

use crate::shared_state::SharedState;

use super::error_400;
use super::error_404;
use super::ok_200;
use super::Filters;
use super::HandlerResult;

fn post(
    (core_id, parameters, state): (
        Path<String>,
        Option<Json<Filters>>,
        Data<RwLock<SharedState>>,
    ),
) -> HandlerResult {
    trace!("POST Triggered!");
    let core = core_id.to_string();
    let context = state.read().unwrap();

    match context.db().core(core) {
        Ok(core) => {
            let parameters = Filters::get(parameters);

            // Generate a list of oid.
            let mut results = match parameters.filters {
                None => core.keys().iter().map(|o| o.id().clone()).collect(),
                // FIXME: Specify from json output space + threshold volume
                Some(filter) => match context.filter(&filter, &core_id, None, None) {
                    Err(_) => vec![], // FIXME: Return errors here instead!!
                    Ok(objects) => objects.iter().map(|o| o.value.id().clone()).collect(),
                },
            };
            results.sort_unstable();
            results.dedup();

            ok_200(&results)
        }
        Err(_) => error_404(),
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
