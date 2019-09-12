use actix_web::HttpRequest;
use actix_web::Json;
use actix_web::Path;

use super::error_400;
use super::error_404;
use super::ok_200;
use super::AppState;
use super::StringOrStaticFileResult;

#[derive(Clone, Deserialize, Serialize)]
pub struct Core {
    name: String,
    version: String,
    scales: Vec<Vec<i32>>,
}

pub fn put(
    (_path, _core, _state): (Path<String>, Json<Core>, HttpRequest<AppState>),
) -> StringOrStaticFileResult {
    trace!("PUT Triggered!");
    error_400()
}

pub fn get((core, state): (Path<String>, HttpRequest<AppState>)) -> StringOrStaticFileResult {
    trace!("GET Triggered!");
    let core = core.to_string();
    let context = state.state().shared.read().unwrap();

    match context.db().core(core) {
        Ok(core) => ok_200(&Core {
            name: core.name().clone(),
            version: core.version().clone(),
            scales: vec![vec![0, 0, 0]],
            //FIXME: Report the actual values. Might need to change the format
            //       to per reference space.
        }),
        Err(_) => error_404(),
    }
}

pub fn patch(
    (_path, _core, _state): (Path<String>, Json<Core>, HttpRequest<AppState>),
) -> StringOrStaticFileResult {
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

    const INSTANCE_EXISTS: &str = "/cores/42";
    const INSTANCE_INVALID: &str = "/cores/21";

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
