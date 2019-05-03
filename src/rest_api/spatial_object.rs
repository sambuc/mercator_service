use super::AppState;

use actix_web::http::StatusCode;
use actix_web::{fs, HttpRequest, Path, Result};

/*
pub fn post((_path, _state): (Path<String>, HttpRequest<AppState>)) -> Result<fs::NamedFile> {
    info!("POST Triggered!");
    Ok(fs::NamedFile::open("static/400.html")?.set_status_code(StatusCode::BAD_REQUEST))
}
*/

pub fn put((_path, _state): (Path<String>, HttpRequest<AppState>)) -> Result<fs::NamedFile> {
    info!("PUT Triggered!");
    Ok(fs::NamedFile::open("static/400.html")?.set_status_code(StatusCode::BAD_REQUEST))
}

pub fn get((_path, _state): (Path<String>, HttpRequest<AppState>)) -> Result<fs::NamedFile> {
    info!("GET Triggered!");
    Ok(fs::NamedFile::open("static/400.html")?.set_status_code(StatusCode::BAD_REQUEST))
}

pub fn patch((_path, _state): (Path<String>, HttpRequest<AppState>)) -> Result<fs::NamedFile> {
    info!("PATCH Triggered!");
    Ok(fs::NamedFile::open("static/400.html")?.set_status_code(StatusCode::BAD_REQUEST))
}

pub fn delete((_path, _state): (Path<String>, HttpRequest<AppState>)) -> Result<fs::NamedFile> {
    info!("DELETE Triggered!");
    Ok(fs::NamedFile::open("static/400.html")?.set_status_code(StatusCode::BAD_REQUEST))
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;

    const INSTANCE_EXISTS: &str = "/datasets/42/spatial_objects/42";
    const INSTANCE_INVALID: &str = "/datasets/42/spatial_objects/21";

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
