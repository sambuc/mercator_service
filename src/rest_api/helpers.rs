use std::fmt::Debug;
use std::io::Error;
use std::io::ErrorKind;

use serde::Serialize;

use super::error_404;
use super::web::Path;
use super::Either;
use super::HandlerResult;
use super::HttpResponse;
use super::NamedFile;
use super::StatusCode;

pub fn ok_200<T>(data: &T) -> HandlerResult
where
    T: Serialize,
{
    match serde_json::to_string(data) {
        Ok(response) => Ok(Either::Left(
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
    Ok(Either::Left(HttpResponse::UnprocessableEntity().body(
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

pub async fn page_404() -> HandlerResult {
    trace!("404 Triggered!");
    error_404()
}

//pub fn page_405() -> HandlerResult {
//    trace!("405 Triggered!");
//    error_405()
//}

pub async fn api(path: Path<String>) -> Result<NamedFile, Error> {
    trace!("api/{} Triggered!", path);

    match NamedFile::open(format!("static/api/{}", path).as_str()) {
        Ok(o) => Ok(o),
        Err(_) => {
            Ok(NamedFile::open("static/errors/404.html")?.set_status_code(StatusCode::NOT_FOUND))
        }
    }
}

pub async fn static_file(path: Path<String>) -> Result<NamedFile, Error> {
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

    #[actix_web::test]
    async fn page_400() {
        expect_400(TestRequest::patch(), &get_core(INVALID_CORE)).await;
    }
}
