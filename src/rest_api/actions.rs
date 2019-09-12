use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Json;

use super::error_400;
use super::AppState;
use super::Filters;
use super::StringOrStaticFileResult;

pub fn health(_req: &HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn query(
    (_parameters, _state): (Option<Json<Filters>>, HttpRequest<AppState>),
) -> StringOrStaticFileResult {
    trace!("query Triggered!");
    error_400()
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
