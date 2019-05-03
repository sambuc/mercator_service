use super::AppState;

use actix_web::http::StatusCode;
use actix_web::{fs, HttpRequest, HttpResponse, Path, Result};

pub fn health(_req: &HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn query(_req: &HttpRequest<AppState>) -> Result<fs::NamedFile> {
    info!("query Triggered!");
    Ok(fs::NamedFile::open("static/400.html")?.set_status_code(StatusCode::BAD_REQUEST))
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;

    #[test]
    fn health() {
        let ep = get_path("/health".into());
        expect_200(http::Method::GET, ep.clone());

        expect_400(http::Method::POST, ep.clone());
        expect_400(http::Method::PUT, ep.clone());
        expect_400(http::Method::PATCH, ep.clone());
        expect_400(http::Method::DELETE, ep.clone());
    }

    #[test]
    fn query() {
        let ep = get_path("/query".into());
        expect_200(http::Method::POST, ep.clone());
        expect_422(http::Method::POST, ep.clone());

        expect_400(http::Method::GET, ep.clone());
        expect_400(http::Method::PUT, ep.clone());
        expect_400(http::Method::PATCH, ep.clone());
        expect_400(http::Method::DELETE, ep.clone());
    }
}
