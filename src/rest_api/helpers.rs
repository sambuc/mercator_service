use std::fmt::Debug;
use std::io::Error;
use std::io::ErrorKind;

use actix_files::NamedFile;
use actix_web::Either;
use actix_web::HttpResponse;
use serde::Serialize;

use super::HandlerResult;
use super::*;

pub fn ok_200<T>(data: &T) -> HandlerResult
where
    T: Serialize,
{
    match serde_json::to_string(data) {
        Ok(response) => Ok(Either::A(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(response),
        )),
        Err(e) => error_500(e),
    }
}

pub fn error_422<S>(reason: S) -> HandlerResult
where
    S: Debug,
{
    Ok(Either::A(HttpResponse::UnprocessableEntity().body(
        format!("422 - Unprocessable Entity:\n{:?}", reason),
    )))
}

pub fn error_500<S>(reason: S) -> HandlerResult
where
    S: Debug,
{
    Err(Error::new(
        ErrorKind::Other,
        format!("500 - Internal Server Error: {:?}", reason),
    ))
}

//pub fn page_400() -> HandlerResult {
//    trace!("400 Triggered!");
//    error_400()
//}

pub fn page_404() -> HandlerResult {
    trace!("404 Triggered!");
    error_404()
}

//pub fn page_405() -> HandlerResult {
//    trace!("405 Triggered!");
//    error_405()
//}

pub fn api(path: Path<String>) -> Result<NamedFile, Error> {
    trace!("api/{} Triggered!", path);

    match NamedFile::open(format!("static/api/{}", path).as_str()) {
        Ok(o) => Ok(o),
        Err(_) => {
            Ok(NamedFile::open("static/errors/404.html")?.set_status_code(StatusCode::NOT_FOUND))
        }
    }
}

pub fn static_file(path: Path<String>) -> Result<NamedFile, Error> {
    trace!("static/{} Triggered!", path);

    match NamedFile::open(format!("static/{}", path).as_str()) {
        Ok(o) => Ok(o),
        Err(_) => {
            Ok(NamedFile::open("static/errors/404.html")?.set_status_code(StatusCode::NOT_FOUND))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests_utils::*;
    use super::*;

    #[test]
    fn page_400() {
        // expect_400(Method::PATCH, get_core(INSTANCE_INVALID));
    }
}
