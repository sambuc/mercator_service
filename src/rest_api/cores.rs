use actix_web::HttpRequest;

use super::error_400;
use super::ok_200;
use super::AppState;
use super::StringOrStaticFileResult;

pub fn post(state: &HttpRequest<AppState>) -> StringOrStaticFileResult {
    trace!("POST Triggered!");
    let db = state.state().shared.read().unwrap();

    ok_200(db.core_keys())
}

pub fn put(_state: &HttpRequest<AppState>) -> StringOrStaticFileResult {
    trace!("PUT Triggered!");
    error_400()
}

/*
pub fn get(_state: &HttpRequest<AppState>) -> StringOrStaticFileResult {
    trace!("GET Triggered!");
    error400()
}*/

pub fn patch(_state: &HttpRequest<AppState>) -> StringOrStaticFileResult {
    trace!("PATCH Triggered!");
    error_400()
}

pub fn delete(_state: &HttpRequest<AppState>) -> StringOrStaticFileResult {
    trace!("DELETE Triggered!");
    error_400()
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;

    const COLLECTION: &str = "/cores";

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
