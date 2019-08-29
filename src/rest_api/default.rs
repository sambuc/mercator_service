use super::AppState;

use actix_web::fs;
use actix_web::http::StatusCode;
use actix_web::HttpRequest;
use actix_web::Result;

pub fn page_400(_req: &HttpRequest<AppState>) -> Result<fs::NamedFile> {
    trace!("400 Triggered!");
    Ok(fs::NamedFile::open("static/400.html")?.set_status_code(StatusCode::BAD_REQUEST))
}

pub fn page_400_no_state(_req: &HttpRequest) -> Result<fs::NamedFile> {
    trace!("400 Triggered!");
    Ok(fs::NamedFile::open("static/400.html")?.set_status_code(StatusCode::BAD_REQUEST))
}
pub fn page_404(_req: &HttpRequest) -> Result<fs::NamedFile> {
    trace!("404 Triggered!");
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;
    use std::sync::RwLock;

    use actix_web::http;
    use actix_web::test::TestRequest;

    #[test]
    fn page_400() {
        let response = TestRequest::with_state(AppState {
            shared: Arc::new(RwLock::new(0)),
        })
        .run(&super::page_400)
        .unwrap();
        assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn page_400_no_state() {
        let response = TestRequest::default()
            .run(&super::page_400_no_state)
            .unwrap();
        assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
    }
    #[test]
    fn page_404() {
        let response = TestRequest::default().run(&super::page_404).unwrap();
        assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
    }
}

#[cfg(test)]
mod routing {
    use super::super::tests::*;

    #[test]
    fn default_no_path() {
        expect_404(http::Method::GET, "".into());
    }

    #[test]
    fn default_slash() {
        expect_404(http::Method::GET, "/".into());
        expect_404(http::Method::GET, "//".into());
        expect_404(http::Method::GET, "/ /".into());
        expect_404(http::Method::GET, "/ //".into());
        expect_404(http::Method::GET, "// ".into());
    }

    #[test]
    fn default_invalid_prefix() {
        expect_404(http::Method::GET, "/test".into());
        expect_404(http::Method::GET, format!("{}test", PREFIX));
    }

    #[test]
    fn default_prefix_no_slash() {
        expect_400(http::Method::GET, PREFIX.into());
    }

    #[test]
    fn default_prefix_final_slash() {
        expect_400(http::Method::GET, format!("{}/", PREFIX));
    }
}
