mod templates;
mod utils;

use axum::{routing::get, Router};
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest, Result};

const DT_UNDANGAN: &str = "2024-10-27T08:00:00+07:00";

fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/update", get(update))
        // .nest_service("/update", get(update().))
        // .route_layer("/update", get(update))
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
    let dt_remaining = utils::make_duration(DT_UNDANGAN);
    let day_time = templates::DayAndTime::from_timedelta(dt_remaining);

    templates::Index {
        title: "Beranda".to_string(), 
        guestname: "Dian".to_string(),
        countdown_ongoing: !day_time.is_timeout(),
        countdown_remaining: day_time,
    }
}

async fn update() -> templates::Countdown {
    let dt_remaining = utils::make_duration(DT_UNDANGAN);    
    templates::Countdown { 
        countdown_remaining: templates::DayAndTime::from_timedelta(dt_remaining) 
    }
}