use actix_web::HttpRequest;
use actix_web::Path;

use crate::model;

use super::error_400;
use super::error_404;
use super::ok_200;
use super::AppState;
use super::StringOrStaticFileResult;

/*
pub fn post(_req: &HttpRequest<AppState>) ->StringOrStaticFileResult {
    info!("POST Triggered!");
    error_400()
}
*/

pub fn put((path, _state): (Path<String>, HttpRequest<AppState>)) -> StringOrStaticFileResult {
    trace!("PUT Triggered on {}", path);
    error_400()
}

pub fn get((path, state): (Path<String>, HttpRequest<AppState>)) -> StringOrStaticFileResult {
    trace!("GET Triggered on '{}'", path);
    let name = path.to_string();
    let db = state.state().shared.read().unwrap();

    match db.space(name) {
        Ok(space) => {
            let space: model::Space = space.into();
            ok_200(&space)
        }
        Err(_) => error_404(),
    }
}

pub fn patch((path, _state): (Path<String>, HttpRequest<AppState>)) -> StringOrStaticFileResult {
    trace!("PATCH Triggered on {}", path);
    error_400()
}

pub fn delete((path, _state): (Path<String>, HttpRequest<AppState>)) -> StringOrStaticFileResult {
    trace!("DELETE Triggered on {}", path);
    error_400()
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;

    const INSTANCE_EXISTS: &str = "/spaces/42";
    const INSTANCE_INVALID: &str = "/spaces/21";

    // FIXME: Add Body to request to see difference between (in)valid bodied requests

    #[test]
    fn put() {
        json::expect_200(http::Method::PUT, get_path(INSTANCE_EXISTS), "".to_string());
        json::expect_422(http::Method::PUT, get_path(INSTANCE_EXISTS), "".to_string());
        json::expect_200(
            http::Method::PUT,
            get_path(INSTANCE_INVALID),
            "".to_string(),
        );
    }

    #[test]
    fn patch() {
        json::expect_200(
            http::Method::PATCH,
            get_path(INSTANCE_EXISTS),
            "".to_string(),
        );
        json::expect_422(
            http::Method::PATCH,
            get_path(INSTANCE_EXISTS),
            "".to_string(),
        );
        expect_400(http::Method::PATCH, get_path(INSTANCE_INVALID));
    }

    #[test]
    fn get() {
        expect_200(http::Method::GET, get_path(INSTANCE_EXISTS));
        expect_404(http::Method::GET, get_path(INSTANCE_INVALID));
    }

    #[test]
    fn delete() {
        expect_200(http::Method::DELETE, get_path(INSTANCE_EXISTS));
        expect_404(http::Method::DELETE, get_path(INSTANCE_INVALID));
    }

    #[test]
    fn post() {
        expect_400(http::Method::POST, get_path(INSTANCE_EXISTS));
        expect_400(http::Method::POST, get_path(INSTANCE_INVALID));
    }
}
