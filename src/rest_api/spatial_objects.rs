use actix_web::HttpRequest;
use actix_web::Path;

use super::error_400;
use super::error_404;
use super::ok_200;
use super::AppState;
use super::StringOrStaticFileResult;

pub fn post((core, state): (Path<String>, HttpRequest<AppState>)) -> StringOrStaticFileResult {
    trace!("POST Triggered!");
    let core = core.to_string();
    let db = state.state().shared.read().unwrap();

    match db.core(core) {
        Ok(core) => {
            // Generate a list of oid.
            let v: Vec<&String> = core.keys().iter().map(|o| o.id()).collect();

            ok_200(&v)
        }
        Err(_) => error_404(),
    }
}

pub fn put((_path, _state): (Path<String>, HttpRequest<AppState>)) -> StringOrStaticFileResult {
    trace!("PUT Triggered!");
    error_400()
}

/*
pub fn get((_path, _state): (Path<String>, HttpRequest<AppState>)) -> StringOrStaticFileResult {
    trace!("GET Triggered!");
    error400()
*/

pub fn patch((_path, _state): (Path<String>, HttpRequest<AppState>)) -> StringOrStaticFileResult {
    trace!("PATCH Triggered!");
    error_400()
}

pub fn delete((_path, _state): (Path<String>, HttpRequest<AppState>)) -> StringOrStaticFileResult {
    trace!("DELETE Triggered!");
    error_400()
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;

    const COLLECTION: &str = "/cores/42/spatial_objects";

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
