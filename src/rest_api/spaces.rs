use super::AppState;

use actix_web::http::StatusCode;
use actix_web::{fs, HttpRequest, Path, Result};

pub fn post(_req: &HttpRequest<AppState>) -> Result<fs::NamedFile> {
    info!("POST Triggered!");
    Ok(fs::NamedFile::open("static/400.html")?.set_status_code(StatusCode::BAD_REQUEST))
}

pub fn put(_state: &HttpRequest<AppState>) -> Result<fs::NamedFile> {
    info!("PUT Triggered!");
    Ok(fs::NamedFile::open("static/400.html")?.set_status_code(StatusCode::BAD_REQUEST))
}

/*
pub fn get(_state: &HttpRequest<AppState>) -> Result<fs::NamedFile> {
    info!("GET Triggered!");
    Ok(fs::NamedFile::open("static/400.html")?.set_status_code(StatusCode::BAD_REQUEST))
}*/

pub fn patch(_state: &HttpRequest<AppState>) -> Result<fs::NamedFile> {
    info!("PATCH Triggered!");
    Ok(fs::NamedFile::open("static/400.html")?.set_status_code(StatusCode::BAD_REQUEST))
}

pub fn delete(_state: &HttpRequest<AppState>) -> Result<fs::NamedFile> {
    info!("DELETE Triggered!");
    Ok(fs::NamedFile::open("static/400.html")?.set_status_code(StatusCode::BAD_REQUEST))
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
