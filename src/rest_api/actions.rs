use std::sync::RwLock;

use serde::Deserialize;

use super::error_422;
use super::from_properties_by_spaces;
use super::ok_200;
use super::web;
use super::web::Data;
use super::web::Json;
use super::HandlerResult;
use super::HttpResponse;
use super::SharedState;
use mercator_db::CoreQueryParameters;

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
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn query((parameters, state): (Json<Query>, Data<RwLock<SharedState>>)) -> HandlerResult {
    trace!("POST '{:?}'", parameters);
    let context = state
        .read()
        .unwrap_or_else(|e| panic!("Can't acquire read lock of the database: {}", e));
    let query = parameters.query();

    if query.is_empty() {
        error_422(format!("Invalid query in '{:?}'", query))
    } else {
        let parameters = CoreQueryParameters {
            db: context.db(),
            output_space: None,
            threshold_volume: parameters.volume(),
            view_port: &parameters.view_port,
            resolution: parameters.resolution(),
        };

        let results = context
            .db()
            .core_keys()
            .iter()
            .filter_map(|core| {
                match context.query(query) {
                    Err(_) => None, // FIXME: Return errors here instead!!
                    Ok(tree) => match context.execute(&tree, core, &parameters) {
                        Err(_) => None, // FIXME: Return errors here instead!!
                        Ok(objects) => Some(from_properties_by_spaces(objects).collect::<Vec<_>>()),
                    },
                }
            })
            .flatten()
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
    use crate::rest_api::tests_utils::*;
    use serde_json::json;

    #[actix_web::test]
    async fn health() {
        let ep = &get_path("/health");

        expect_200(TestRequest::get(), ep).await;

        expect_405(TestRequest::post(), ep).await;
        expect_405(TestRequest::put(), ep).await;
        expect_405(TestRequest::patch(), ep).await;
        expect_405(TestRequest::delete(), ep).await;
    }

    #[actix_web::test]
    async fn query() {
        let ep = &get_path("/query");

        expect_200(
            TestRequest::post()
                .set_json(json!({"query": "json(.,inside(hyperrectangle{[0,0,0],[0,1,1]}))"})),
            ep,
        )
        .await;

        expect_422(TestRequest::post().set_json(json!({"query": "toto"})), ep).await;
        expect_422(TestRequest::post().set_json(json!({"query": ""})), ep).await;
        expect_400(TestRequest::post().set_json(json!({"invalid": true})), ep).await;
        expect_400(TestRequest::post().set_json(json!({})), ep).await;
        expect_400(TestRequest::post(), ep).await;

        expect_405(TestRequest::get(), ep).await;
        expect_405(TestRequest::put(), ep).await;
        expect_405(TestRequest::patch(), ep).await;
        expect_405(TestRequest::delete(), ep).await;
    }
}
