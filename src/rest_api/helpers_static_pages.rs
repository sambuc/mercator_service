#![cfg(feature = "static-error-pages")]

use actix_files::NamedFile;
use actix_web::http::StatusCode;
use actix_web::Either;

use super::HandlerResult;

fn error(code: StatusCode) -> HandlerResult {
    let path = format!("static/errors/{}.html", u16::from(code));

    Ok(Either::B(NamedFile::open(path)?.set_status_code(code)))
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
