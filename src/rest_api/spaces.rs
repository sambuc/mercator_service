use std::sync::RwLock;

use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Json;

use crate::shared_state::SharedState;

use super::error_400;
use super::ok_200;
use super::Filters;
use super::HandlerResult;

fn post((parameters, state): (Option<Json<Filters>>, Data<RwLock<SharedState>>)) -> HandlerResult {
    trace!("POST Triggered!");
    let context = state.read().unwrap();
    let parameters = Filters::get(parameters);

    let mut results = match parameters.filters {
        None => context.db().space_keys().clone(),
        Some(filter) => context
            .db()
            .core_keys()
            .iter()
            // FIXME: Specify from json output space + threshold volume
            .flat_map(|core| match context.filter(&filter, core, None, None) {
                Err(_) => vec![], // FIXME: Return errors here instead!!
                Ok(r) => {
                    let mut r = r.into_iter().map(|o| o.space_id).collect::<Vec<_>>();
                    r.sort_unstable();
                    r.dedup();
                    r
                }
            })
            .collect(),
    };
    results.sort_unstable();
    results.dedup();

    ok_200(&results)
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

    #[test]
    fn post() {
        expect_200(Method::POST, &get_space(""));
        json::expect_200(Method::POST, &get_space(""), "".to_string());

        json::expect_422(Method::POST, &get_space(""), "".to_string());

        expect_400(Method::POST, &get_space(""));
    }

    #[test]
    fn put() {
        json::expect_200(Method::PUT, &get_space(""), "".to_string());

        json::expect_422(Method::PUT, &get_space(""), "".to_string());

        expect_400(Method::PUT, &get_space(""));
    }

    #[test]
    fn patch() {
        json::expect_200(Method::PATCH, &get_space(""), "".to_string());

        json::expect_422(Method::PATCH, &get_space(""), "".to_string());

        expect_400(Method::PATCH, &get_space(""));
    }

    #[test]
    fn delete() {
        json::expect_200(Method::DELETE, &get_space(""), "".to_string());

        json::expect_422(Method::DELETE, &get_space(""), "".to_string());

        expect_400(Method::DELETE, &get_space(""));
    }

    #[test]
    fn get() {
        expect_405(Method::GET, &get_space(""));
    }
}
