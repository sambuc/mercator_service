use std::sync::RwLock;

use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::HttpResponse;

use crate::model::to_spatial_objects;
use crate::shared_state::SharedState;

use super::error_422;
use super::ok_200;
use super::HandlerResult;

#[derive(Debug, Deserialize)]
pub struct Query {
    query: String,
    resolution: Option<Vec<u32>>, // None means automatic selection, based on ViewPort
    view_port: Option<(Vec<f64>, Vec<f64>)>,
}

impl Query {
    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn resolution(&self) -> &Option<Vec<u32>> {
        &self.resolution
    }

    pub fn volume(&self) -> Option<f64> {
        match &self.view_port {
            None => None,
            Some(_view) => None, // FIXME: Need to move the Volume functions from mercator_parser to mercator_db.
        }
    }
}
// Also used for the root service.
pub fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn query((parameters, state): (Json<Query>, Data<RwLock<SharedState>>)) -> HandlerResult {
    trace!("POST '{:?}'", parameters);
    let context = state.read().unwrap();
    let query = parameters.query();

    if query.is_empty() {
        error_422(format!("Invalid query in '{:?}'", query))
    } else {
        ok_200(
            &context
                .db()
                .core_keys()
                .iter()
                .flat_map(|core| {
                    match context.query(
                        query,
                        core,
                        parameters.volume(),
                        &parameters.view_port,
                        parameters.resolution(),
                    ) {
                        Err(_) => vec![], // FIXME: Return errors here instead!!
                        Ok(objects) => to_spatial_objects(objects),
                    }
                })
                .collect::<Vec<_>>(),
        )
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
