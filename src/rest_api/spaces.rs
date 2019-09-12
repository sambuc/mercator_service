use actix_web::HttpRequest;
use actix_web::Json;

use super::error_400;
use super::ok_200;
use super::AppState;
use super::Filters;
use super::StringOrStaticFileResult;

pub fn post(
    (parameters, state): (Option<Json<Filters>>, HttpRequest<AppState>),
) -> StringOrStaticFileResult {
    trace!("POST Triggered!");
    let context = state.state().shared.read().unwrap();
    let parameters = Filters::get(parameters);

    let mut results = match parameters.filters {
        None => context.db().space_keys().clone(),
        Some(filter) => context
            .db()
            .core_keys()
            .iter()
            .flat_map(|core| match context.filter(&filter, core, None, None) {
                Err(_) => vec![], //FIXME: Return errors here instead!!
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

pub fn put(
    (_parameters, _state): (Option<Json<Filters>>, HttpRequest<AppState>),
) -> StringOrStaticFileResult {
    trace!("PUT Triggered!");
    error_400()
}

pub fn patch(
    (_parameters, _state): (Option<Json<Filters>>, HttpRequest<AppState>),
) -> StringOrStaticFileResult {
    trace!("PATCH Triggered!");
    error_400()
}

pub fn delete(
    (_parameters, _state): (Option<Json<Filters>>, HttpRequest<AppState>),
) -> StringOrStaticFileResult {
    trace!("DELETE Triggered!");
    error_400()
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;

    const COLLECTION: &str = "/spaces";

    // FIXME: Add Body to request to see difference between (in)valid bodied requests

    #[test]
    fn post() {
        expect_200(http::Method::POST, get_path(COLLECTION));
        json::expect_200(http::Method::POST, get_path(COLLECTION), "".to_string());

        json::expect_422(http::Method::POST, get_path(COLLECTION), "".to_string());

        expect_400(http::Method::POST, get_path(COLLECTION));
    }

    #[test]
    fn put() {
        json::expect_200(http::Method::PUT, get_path(COLLECTION), "".to_string());

        json::expect_422(http::Method::PUT, get_path(COLLECTION), "".to_string());

        expect_400(http::Method::PUT, get_path(COLLECTION));
    }

    #[test]
    fn patch() {
        json::expect_200(http::Method::PATCH, get_path(COLLECTION), "".to_string());

        json::expect_422(http::Method::PATCH, get_path(COLLECTION), "".to_string());

        expect_400(http::Method::PATCH, get_path(COLLECTION));
    }

    #[test]
    fn delete() {
        json::expect_200(http::Method::DELETE, get_path(COLLECTION), "".to_string());

        json::expect_422(http::Method::DELETE, get_path(COLLECTION), "".to_string());

        expect_400(http::Method::DELETE, get_path(COLLECTION));
    }

    #[test]
    fn get() {
        expect_400(http::Method::GET, get_path(COLLECTION));
    }
}
