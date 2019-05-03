use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::{Method, StatusCode};
use actix_web::server::{HttpHandler, HttpHandlerTask};
use actix_web::{fs, pred, server, App, HttpRequest, HttpResponse, Result};

pub type SharedState = i32;

// Application shared state
struct AppState {
    /*host: &'static str,
    port: u16,
    prefix: &'static str,*/
    shared: Arc<RwLock<SharedState>>,
}

// simple handle
fn index(req: &HttpRequest<AppState>) -> HttpResponse {
    println!("{:?}", req);
    {
        // So that we release ASAP the exclusive lock.
        *(req.state().shared.write().unwrap()) += 1;
    }

    HttpResponse::BadRequest().body(format!(
        "Num of requests: {}",
        req.state().shared.read().unwrap()
    ))
}

/// 400 handler
fn page_400(_req: &HttpRequest<AppState>) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/400.html")?.set_status_code(StatusCode::BAD_REQUEST))
}

/// 404 handler
fn page_404(_req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

fn get_app(
    prefix: &'static str,
    state: Arc<RwLock<SharedState>>,
) -> Vec<Box<HttpHandler<Task = Box<HttpHandlerTask>>>> {
    vec![
        App::with_state(AppState { shared: state })
            .prefix(format!("{}", prefix))
            // register simple handler, handle all methods
            .resource("/", |r| r.f(index))
            // default
            .default_resource(|r| {
                // 400 for GET request
                r.f(page_400);
            })
            .boxed(),
        App::new()
            .default_resource(|r| {
                // 404 for GET request
                r.method(Method::GET).f(page_404);

                // all requests that are not `GET`
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(|_req| HttpResponse::MethodNotAllowed());
            })
            .boxed(),
    ]
}

pub fn run(
    host: &'static str,
    port: u16,
    prefix: &'static str,
    state: Arc<RwLock<SharedState>>,
) -> () {
    info!("Initializing server...");

    let sys = actix::System::new("spatial-search");

    server::new(move || get_app(prefix, state.clone()))
        .bind(format!("{}:{}", host, port))
        .unwrap()
        .start();

    info!("Started http server: {}:{}{}", host, port, prefix);

    let _ = sys.run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http;
    use actix_web::test::{TestRequest, TestServer};

    const PREFIX: &str = "spatial-search";

    fn get_test_server() -> TestServer {
        let state = Arc::new(RwLock::new(0));
        TestServer::with_factory(move || get_app(PREFIX, state.clone()))
    }

    fn expect_200(path: &str) -> () {
        let mut srv = get_test_server();
        let req = srv.client(http::Method::GET, path).finish().unwrap();
        let response = srv.execute(req.send()).unwrap();
        assert_eq!(http::StatusCode::OK, response.status());
    }

    fn expect_400(path: &str) -> () {
        let mut srv = get_test_server();
        let req = srv.client(http::Method::GET, path).finish().unwrap();
        let response = srv.execute(req.send()).unwrap();
        assert_eq!(http::StatusCode::BAD_REQUEST, response.status());
    }

    fn expect_404(path: &str) -> () {
        let mut srv = get_test_server();
        let req = srv.client(http::Method::GET, path).finish().unwrap();
        let response = srv.execute(req.send()).unwrap();
        assert_eq!(http::StatusCode::NOT_FOUND, response.status());
    }

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
    fn page_404() {
        let response = TestRequest::default().run(&super::page_404).unwrap();
        assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
    }

    #[test]
    fn default_no_path() {
        expect_404("");
    }

    #[test]
    fn default_slash() {
        expect_404("/");
        expect_404("//");
        expect_404("/ /");
        expect_404("/ //");
        expect_404("// ");
    }

    #[test]
    fn default_invalid_prefix() {
        expect_404("/test");
        expect_404(format!("{}test", PREFIX).as_str());
    }

    #[test]
    fn default_prefix_no_slash() {
        expect_400(PREFIX);
    }

    #[test]
    fn default_prefix_final_slash() {
        expect_400(format!("{}/", PREFIX).as_str());
    }

}
