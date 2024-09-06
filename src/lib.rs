mod templates;

use axum::{routing::get, Router};
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest, Result};

fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .nest_service("/countdown", get(index))
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

async fn index() -> templates::Index {
    
    // sleep(Duration::from_secs(1)).await;

    templates::Index {
        title: "Beranda".to_string(), 
        guestname: "Dian".to_string(),
        datetime_remaining: 102
    }
}