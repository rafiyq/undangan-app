use askama_axum::Template;
use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

fn router() -> Router {
    Router::new().route("/", get(root))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

async fn root() -> HelloTemplate<'static> {
    HelloTemplate { name: "askama" }
}