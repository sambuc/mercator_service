#![cfg(not(feature = "static-error-pages"))]

use actix_web::http::StatusCode;
use actix_web::Either;
use actix_web::HttpResponse;

use super::HandlerResult;

fn error(code: StatusCode) -> HandlerResult {
    Ok(Either::A(HttpResponse::build(code).finish()))
}

pub fn error_400() -> HandlerResult {
    error(StatusCode::BAD_REQUEST)
}

pub fn error_404() -> HandlerResult {
    error(StatusCode::NOT_FOUND)
}

//pub fn error_405() -> HandlerResult {
//    error(StatusCode::METHOD_NOT_ALLOWED)
//}
