use std::sync::RwLock;

use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::HttpResponse;

use crate::shared_state::SharedState;

use super::error_422;
use super::ok_200;
use super::HandlerResult;

#[derive(Debug, Deserialize)]
pub struct Query {
    query: String,
}

// Also used for the root service.
pub fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn query((parameters, state): (Json<Query>, Data<RwLock<SharedState>>)) -> HandlerResult {
    trace!("query Triggered!");
    let context = state.read().unwrap();
    let query = &parameters.query;

    if query.is_empty() {
        error_422(format!("Invalid query in '{:?}'", query))
    } else {
        // FIXME: MANAGE PROJECTIONS
        let results = context
            .db()
            .core_keys()
            .iter()
            // FIXME: Specify from json output space + threshold volume
            .flat_map(|core| match context.query(query, core, None, None) {
                Err(_) => vec![], // FIXME: Return errors here instead!!
                Ok(r) => {
                    let mut r = r.into_iter().map(|o| o.space_id).collect::<Vec<_>>();
                    r.sort_unstable();
                    r.dedup();
                    r
                }
            })
            .collect::<Vec<_>>();

        ok_200(&results)
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health").route(web::get().to(health)));
    cfg.service(web::resource("/query").route(web::post().to(query)));
}

#[cfg(test)]
mod routing {
    use super::super::tests_utils::*;

    #[test]
    fn health() {
        let ep = &get_path("/health");

        expect_200(Method::GET, ep);

        expect_405(Method::POST, ep);
        expect_405(Method::PUT, ep);
        expect_405(Method::PATCH, ep);
        expect_405(Method::DELETE, ep);
    }

    #[test]
    fn query() {
        let ep = &get_path("/query");

        expect_200(Method::POST, ep);
        expect_422(Method::POST, ep);

        expect_405(Method::GET, ep);
        expect_405(Method::PUT, ep);
        expect_405(Method::PATCH, ep);
        expect_405(Method::DELETE, ep);
    }
}
